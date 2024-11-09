#[test]
fn set_str_to_clipboard() {
    dbg!(&clipboard::set_str_to_clipboard(
        "this message is from rust program!!"
    ));
}
