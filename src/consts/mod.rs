cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "loongarch64", platform_family = "loongarch64-qemu-virt"))] {
        mod loongson_3a5000;
        pub use self::loongson_3a5000::*;
    }
}