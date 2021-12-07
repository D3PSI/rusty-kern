use std::{
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    let mut args = std::env::args().skip(1); // skip executable name

    let kernel_binary_path = {
        let path = PathBuf::from(args.next().unwrap());
        path.canonicalize().unwrap()
    };
    let mut no_boot = false;
    let mut bios_boot = false;
    let mut gdb = false;
    for arg in args {
        match arg.as_str() {
            "--no-run" => no_boot = true,
            "--bios-boot" => bios_boot = true,
            "--gdb" => gdb = true,
            other => panic!("unexpected argument `{}`", other),
        }
    }

    let (bios, uefi) = create_disk_images(&kernel_binary_path);

    if no_boot {
        println!(
            "Created disk images at `{}` and `{}`",
            bios.display(),
            uefi.display()
        );
        return;
    }

    let mut run_cmd = Command::new("qemu-system-x86_64");
    if bios_boot {
        run_cmd
            .arg("-drive")
            .arg(format!("format=raw,file={}", bios.display()));
    } else {
        run_cmd
            .arg("-bios")
            .arg("/usr/share/ovmf/x64/OVMF.fd")
            .arg("-drive")
            .arg(format!("format=raw,file={}", uefi.display()));
    }
    if gdb {
        run_cmd.arg("-gdb").arg("tcp:localhost:10000");
    } else {
        run_cmd.arg("-s");
    }
    run_cmd.arg("--no-reboot");

    let exit_status = run_cmd.status().unwrap();
    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(1));
    }
}

pub fn create_disk_images(kernel_binary_path: &Path) -> (PathBuf, PathBuf) {
    let bootloader_manifest_path = bootloader_locator::locate_bootloader("bootloader").unwrap();
    let kernel_manifest_path = locate_cargo_manifest::locate_manifest().unwrap();

    let mut build_cmd = Command::new(env!("CARGO"));
    build_cmd.current_dir(bootloader_manifest_path.parent().unwrap());
    build_cmd.arg("builder");
    build_cmd
        .arg("--kernel-manifest")
        .arg(&kernel_manifest_path);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary_path);
    build_cmd
        .arg("--target-dir")
        .arg(kernel_manifest_path.parent().unwrap().join("target"));
    build_cmd
        .arg("--out-dir")
        .arg(kernel_binary_path.parent().unwrap());
    build_cmd.arg("--quiet");

    if !build_cmd.status().unwrap().success() {
        panic!("build failed");
    }

    let kernel_binary_name = kernel_binary_path.file_name().unwrap().to_str().unwrap();
    let bios_image = kernel_binary_path
        .parent()
        .unwrap()
        .join(format!("boot-bios-{}.img", kernel_binary_name));
    let uefi_image = kernel_binary_path
        .parent()
        .unwrap()
        .join(format!("boot-uefi-{}.img", kernel_binary_name));
    if !bios_image.exists() || !uefi_image.exists() {
        panic!("Failed to build disk images");
    }
    (bios_image, uefi_image)
}
