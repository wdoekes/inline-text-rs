use inline_text::concat;


#[test]
fn concat_test() {
    let s = concat!(
        "line1\n",
        "line2\n",
        "line3",
        "line3b",
    );
    assert_eq!(s, "line1\nline2\nline3line3b");
}
