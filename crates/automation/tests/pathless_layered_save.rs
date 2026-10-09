//! A pathless save to an opened layered file is a real save, not a separate export (#1547).

use photocraft_automation::Headless;
use serde_json::json;

#[test]
fn pathless_layered_saves_update_saved_revision_without_hiding_new_edits() {
    let dir = std::env::temp_dir().join(format!(
        "pc-layered-save-state-{}-{}",
        std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();

    for ext in ["psd", "psb", "pcraft"] {
        let path = dir.join(format!("document.{ext}"));
        let mut app = Headless::trusted_local();
        app.handle("doc.new", json!({"width": 8, "height": 8, "background": "white"})).unwrap();
        app.handle("doc.save", json!({"path": path.to_str().unwrap()})).unwrap();
        app.handle("doc.open", json!({"path": path.to_str().unwrap()})).unwrap();
        let index = app.session.active_index().unwrap();

        app.handle("engine.execute", json!({"command": "image.adjustments.invert"})).unwrap();
        assert!(app.session.documents()[index].is_dirty(), "{ext}: edit should make document dirty");
        let edited_revision = app.session.documents()[index].revision;

        let saved = app.handle("doc.save", json!({})).unwrap();
        assert_eq!(saved["path"], path.to_str().unwrap(), "{ext}: save target");
        assert!(path.is_file(), "{ext}: save should write the file");
        let state = &app.session.documents()[index];
        assert!(!state.is_dirty(), "{ext}: successful in-place save should clear dirty");
        assert_eq!(state.saved_revision, edited_revision, "{ext}: saved revision");
        assert_eq!(state.path.as_deref(), path.to_str(), "{ext}: original path");

        app.handle("doc.save", json!({})).unwrap();
        assert!(!app.session.documents()[index].is_dirty(), "{ext}: repeated save");

        app.handle("engine.execute", json!({"command": "image.adjustments.invert"})).unwrap();
        assert!(app.session.documents()[index].is_dirty(), "{ext}: a subsequent edit must be dirty");
        app.handle("doc.open", json!({"path": path.to_str().unwrap()})).unwrap();
        assert!(!app.session.active().unwrap().is_dirty(), "{ext}: reopened saved copy");
    }

    std::fs::remove_dir_all(dir).unwrap();
}
