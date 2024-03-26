fn main() {
    println!("cargo:rustc-cfg=target_arch=\"loongarch64\"");
    println!("cargo:rustc-cfg=platform_family=\"loongarch64-qemu-virt\"");
}