use photocraft_engine::Session;
use serde_json::json;

fn three_documents() -> Session {
    let mut session = Session::new();
    for width in [10, 20, 30] {
        session.execute("file.new", json!({"width": width, "height": 8})).unwrap();
    }
    session
}

#[test]
fn closing_an_earlier_tab_keeps_the_same_active_document() {
    let mut session = three_documents();
    session.execute("file.close", json!({"document": 0})).unwrap();
    assert_eq!(session.documents().len(), 2);
    assert_eq!(session.active_index(), Some(1));
    assert_eq!(session.active().unwrap().doc.size.width, 30);
}

#[test]
fn closing_a_later_tab_keeps_the_same_active_document() {
    let mut session = three_documents();
    assert!(session.set_active(0));
    session.execute("file.close", json!({"document": 2})).unwrap();
    assert_eq!(session.active_index(), Some(0));
    assert_eq!(session.active().unwrap().doc.size.width, 10);
}

#[test]
fn closing_the_active_tab_activates_a_neighbour_or_none() {
    let mut session = three_documents();
    assert!(session.set_active(1));
    session.execute("file.close", json!({"document": 1})).unwrap();
    assert_eq!(session.active().unwrap().doc.size.width, 30);
    session.execute("file.close", json!({"document": 1})).unwrap();
    assert_eq!(session.active().unwrap().doc.size.width, 10);
    session.execute("file.close", json!({"document": 0})).unwrap();
    assert_eq!(session.active_index(), None);
    assert!(session.active().is_none());
}
