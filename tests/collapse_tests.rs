use inline_text::collapse;


#[test]
fn collapse_test() {
    let s = collapse!("

        Blah blah \t blah
        foo   blah.

          --

    ");
    assert_eq!(s, "Blah blah blah foo blah. --");
}
