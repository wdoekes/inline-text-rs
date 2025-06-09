use inline_text::concat;

#[test]
fn concat_like_std() {
    let s = concat!("line1\n", "line2\n", "line3", "line3b",);
    assert_eq!(s, "line1\nline2\nline3line3b");
}

#[test]
fn concat_using_spaces() {
    let s = concat!(
        "line1\n"
        "line2\n"
        "line3"
        "line3b"
    );
    assert_eq!(s, "line1\nline2\nline3line3b");
}

#[test]
fn concat_mixing_both() {
    let s = concat!(
        ,,,
        "line1\n"
        "line2\n",
        "line3"
        "line3b"
    );
    assert_eq!(s, "line1\nline2\nline3line3b");
}
