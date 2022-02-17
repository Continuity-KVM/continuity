//! Cross-platform crate for sharing a computer's screen(s).
#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

#[cfg(target_os = "windows")]
pub mod windows;

/// The Linux-specific modules of this crate.
#[cfg(target_os = "linux")]
pub mod linux {
    pub mod wayland;
    pub mod x11;
}
