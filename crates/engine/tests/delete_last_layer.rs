use photocraft_engine::Session;
use serde_json::json;

#[test]
fn explicitly_deleting_the_last_layer_leaves_the_document_unchanged() {
    let mut session = Session::new();
    session.execute("file.new", json!({"width": 10, "height": 5, "background": "transparent"})).unwrap();
    let id = session.active().unwrap().doc.layers[0].id.0;
    let result = session.execute("layer.delete", json!({"layer": id}));
    assert!(result.is_err(), "last-layer deletion should be refused");
    let doc = &session.active().unwrap().doc;
    assert_eq!(doc.layers.len(), 1);
    assert_eq!(doc.layers[0].id.0, id);
    assert_eq!(session.active().unwrap().active_layer.map(|l| l.0), Some(id));
}

#[test]
fn explicitly_deleting_a_layer_still_works_when_another_remains() {
    let mut session = Session::new();
    session.execute("file.new", json!({"width": 10, "height": 5, "background": "transparent"})).unwrap();
    let original_id = session.active().unwrap().doc.layers[0].id.0;
    let added_id = session.execute("layer.new.layer", json!({})).unwrap()["layer"].as_u64().unwrap();
    session.execute("layer.delete", json!({"layer": added_id})).unwrap();
    let doc = &session.active().unwrap().doc;
    assert_eq!(doc.layers.len(), 1);
    assert_eq!(doc.layers[0].id.0, original_id);
}
