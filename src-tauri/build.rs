fn main() {
    // Skip Windows resource compilation for development
    #[cfg(target_os = "windows")]
    {
        std::env::set_var("TAURI_SKIP_EMBEDDED_METADATA", "true");
    }
    
    tauri_build::build()
}
