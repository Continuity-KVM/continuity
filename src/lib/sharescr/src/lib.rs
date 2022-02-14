#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux {
    pub mod wayland;
    pub mod x11;
}
