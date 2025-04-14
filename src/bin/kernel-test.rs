use bootloader::DiskImageBuilder;
use std::{
    env,
    path::PathBuf,
    process::{self, Command},
};

fn main() {
    let test_kernel_bin = env::var("KERNEL_TEST_BIN")
        .expect("Please set KERNEL_TEST_BIN to point to your raw test kernel binary");

    let disk_builder = DiskImageBuilder::new(PathBuf::from(&test_kernel_bin));

    let temp_dir = env::temp_dir();
    let uefi_image_path = temp_dir.join("kernel_test_uefi.img");

    disk_builder
        .create_uefi_image(&uefi_image_path)
        .expect("Failed to create UEFI test image");

    println!("Created test image: {}", uefi_image_path.display());

    let mut qemu = Command::new("qemu-system-x86_64");

    qemu.arg("-serial").arg("stdio");
    qemu.arg("-drive")
        .arg(format!("format=raw,file={}", uefi_image_path.display()));

    qemu.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());

    let exit_status = qemu.status().expect("Failed to execute QEMU");
    process::exit(exit_status.code().unwrap_or(-1));
}
