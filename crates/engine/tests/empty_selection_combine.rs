use photocraft_engine::Session;
use serde_json::json;

fn session() -> Session {
    let mut session = Session::new();
    session.execute("file.new", json!({"width": 32, "height": 24})).unwrap();
    session
}

#[test]
fn subtract_and_intersect_never_create_a_selection_from_none() {
    for mode in ["subtract", "intersect"] {
        for feather in [0.0, 2.0] {
            let mut session = session();
            session.execute("select.rect", json!({
                "x": 4, "y": 5, "width": 12, "height": 10,
                "mode": mode, "feather": feather
            })).unwrap();
            assert!(session.active().unwrap().doc.selection.is_none(), "mode={mode}, feather={feather}");
        }
    }
}

#[test]
fn replace_and_add_still_create_selections_from_none() {
    for mode in ["replace", "add"] {
        let mut session = session();
        session.execute("select.rect", json!({
            "x": 4, "y": 5, "width": 12, "height": 10, "mode": mode
        })).unwrap();
        let selection = session.active().unwrap().doc.selection.as_ref().unwrap();
        assert!(!selection.content_bounds().is_empty(), "mode={mode}");
    }
}

#[test]
fn intersect_with_an_existing_selection_keeps_the_overlap() {
    let mut session = session();
    session.execute("select.rect", json!({"x": 4, "y": 5, "width": 12, "height": 10})).unwrap();
    session.execute("select.rect", json!({
        "x": 8, "y": 9, "width": 12, "height": 10, "mode": "intersect"
    })).unwrap();
    let selection = session.active().unwrap().doc.selection.as_ref().unwrap();
    let mut inside = [0.0];
    let mut outside = [0.0];
    selection.read_pixel(8, 9, &mut inside);
    selection.read_pixel(4, 5, &mut outside);
    assert!(inside[0] > 0.0);
    assert_eq!(outside[0], 0.0);
}
