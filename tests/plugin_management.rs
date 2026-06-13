use rest_reminder::web::plugin::{parse_plugin_info, set_ignore_marker};
use std::path::Path;

#[test]
fn set_ignore_marker_updates_existing_flag() {
    let code = "_SHOULD_IGNORE = 1\n\ndef on_init(context):\n    pass\n";
    let updated = set_ignore_marker(code, false);

    assert!(updated.starts_with("_SHOULD_IGNORE = 0"));
    assert!(updated.contains("def on_init"));
}

#[test]
fn set_ignore_marker_adds_missing_flag() {
    let code = "def on_break_reminder(context):\n    pass\n";
    let updated = set_ignore_marker(code, true);

    assert!(updated.starts_with("_SHOULD_IGNORE = 1\n"));
    assert!(updated.contains("def on_break_reminder"));
}

#[test]
fn parse_plugin_info_reads_metadata_and_hooks() {
    let code = r#"
_SHOULD_IGNORE = 0
_RUN_IN_SUBPROCESS = 1
def on_init(context):
    pass
PLUGIN_INFO = {
    "name": "Example",
    "version": "1.2.3",
    "description": "Demo plugin",
    "author": "Tester"
}
"#;

    let info = parse_plugin_info(Path::new("plugins/example.py"), code, &[]);

    assert_eq!(info.name, "Example");
    assert_eq!(info.file_name, "example");
    assert!(info.enabled);
    assert_eq!(info.version.as_deref(), Some("1.2.3"));
    assert_eq!(info.hooks, vec!["on_init"]);
    assert!(info.run_in_subprocess);
}
