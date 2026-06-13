use std::path::PathBuf;
use tokio::fs::{self, File};
use tokio::io::AsyncWriteExt;

pub async fn generate_plugin_template(file_name: &str) -> std::io::Result<PathBuf> {
    let python_content = r#""""
DO NOT REMOVE THESE 2 CONSTANTS!
Set _SHOULD_IGNORE = 1 if you do not wish to load this plugin.
Set _RUN_IN_SUBPROCESS = 1 if you wish this plugin to be run in a subprocess
    that won't block the main process. Change it to 0 is STRONGLY UNRECOMMENDED as it
    is very likely resulting in bugs unless you know clearly what you're doing.
"""
_SHOULD_IGNORE = 0
_RUN_IN_SUBPROCESS = 1

# Uncomment any wanted hooks or delete any unwanted hooks.

# def on_init(context):
#     pass

# def on_work_start(context):
#     pass

# def on_break_reminder(context):
#     pass

# Plugin info (optional)
PLUGIN_INFO = {
    "name": "",
    "version": "",
    "description": "",
    "author": ""
}
    "#;

    fs::create_dir_all("./plugins").await?;
    let file_path = PathBuf::from(format!("./plugins/{}.py", file_name));
    let mut file = File::create(&file_path).await?;

    file.write_all(python_content.as_bytes()).await?;
    file.flush().await?;

    println!("Successfully generated plugin.");
    Ok(file_path)
}
