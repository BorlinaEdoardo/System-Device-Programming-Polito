use line_editor::editor::{LineEditor, };

#[test]
fn editor_creation() {
    let ed = LineEditor::new(String::from("Hello\nworld\n!"));
    assert_eq!(ed.all_lines(), ["Hello", "world", "!"]);
}