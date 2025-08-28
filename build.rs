use std::io;
#[cfg(windows)]
use winresource::WindowsResource;

fn main() -> io::Result<()> {
    // Windows icon configuration
    #[cfg(windows)]
    {
        println!("cargo:rerun-if-changed=assets/icon.ico");
        println!("cargo:rerun-if-changed=resources/icon.rc");
        
        WindowsResource::new()
            .set_icon("assets/icon.ico")
            .set_icon_with_id("assets/icon.ico", "app-icon")
            .compile()?;
    }

    Ok(())
}