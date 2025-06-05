#![feature(unsized_const_params, trivial_bounds)]
#![allow(incomplete_features, trivial_bounds)]

#[diagnostic::on_unimplemented(
    message = "Configuration requirements are not always met",
    label = "The compile-time condition cfg({Self} = {F}) is not always true in this scope",
    note = "Consider adding a `where {Self}: Is<{F}>` bound to ensure the caller respects the required configuration"
)]
#[allow(dead_code)]
pub trait Is<const F: &'static str, const CHECK_CFG: bool = false>
where
    Self: CanBe<F>,
{
}

pub trait CanBe<const F: &'static str> {}

const fn is_check_cfg() -> bool {
    match option_env!("CHECK_CFG") {
        Some(x) => matches!(x.as_bytes(), b"1" | b"true" | b"TRUE" | b"yes" | b"YES"),
        None => false,
    }
}

#[macro_export]
macro_rules! cfgs {
    ($($key:ident = { $($feature:literal),* $(,)? } $(,)?)*) => {
        $(
            #[allow(non_camel_case_types)]
            pub struct $key;
            $(
                #[diagnostic::do_not_recommend]
                impl $crate::CanBe<$feature> for $key {}
                #[cfg(all($key = $feature))]
                #[diagnostic::do_not_recommend]
                impl $crate::Is<$feature, { $crate::is_check_cfg() }> for $key {}
            )*
        )*
    };
}

cfgs!(
    target_arch = { "x86", "x86_64", "mips", "powerpc", "powerpc64", "arm", "aarch64" },
    target_feature = { "avx", "avx2", "crt-static", "rdrand", "sse", "sse2", "sse4.1" },
    target_os = {
        "windows",
        "macos",
        "ios",
        "linux",
        "android",
        "freebsd",
        "dragonfly",
        "openbsd",
        "netbsd",
        "none",
    },
    target_family = { "unix", "windows", "wasm" },
    target_env = { "", "gnu", "msvc", "musl", "sgx" },
    target_abi = { "", "llvm", "eabihf", "abi64", "sim", "macabi" },
    target_endian = { "little", "big" },
    target_pointer_width = { "16", "32", "64" },
    target_vendor = { "apple", "fortanix", "pc", "unknown" },
    panic = { "abort", "unwind" },
);
