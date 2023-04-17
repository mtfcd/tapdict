fn main() {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=User32");
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-lib=Crypt32");
}
