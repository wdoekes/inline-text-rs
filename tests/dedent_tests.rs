use inline_text::dedent;

#[test]
fn dedent_spaces() {
    let s = dedent!(
        "\
        line1
        line2
    "
    );
    assert_eq!(s, "line1\nline2\n");
}

#[test]
fn dedent_spaces_4() {
    let s = dedent!(
        keep_ws = 4,
        "
        line1
        line2
    "
    );
    assert_eq!(s, "\n    line1\n    line2\n");
}

#[test]
fn dedent_six_four_two() {
    let s = inline_text::dedent!(
        "
          SIX
        FOUR
      TWO
    "
    );
    assert_eq!(s, "\n    SIX\n  FOUR\nTWO\n");
}

#[test]
fn dedent_hello_world() {
    let s = inline_text::dedent!(
        keep_ws = 4,
        "\
            fn main() {
                println!(\"Hello world 1!\");
            }
    "
    );
    assert_eq!(
        s,
        "    fn main() {\n        println!(\"Hello world 1!\");\n    }\n"
    );
}

#[test]
fn dedent_hello_world_leading_lf() {
    let s = inline_text::dedent!(
        keep_ws = 4,
        "
            fn main() {
                println!(\"Hello world 2!\");
            }
    "
    );
    assert_eq!(
        s,
        "\n    fn main() {\n        println!(\"Hello world 2!\");\n    }\n"
    );
}

#[test]
fn dedent_hello_world_no_trailing_lf() {
    let s = inline_text::dedent!(
        keep_ws = 4,
        "\
            fn main() {
                println!(\"Hello world 3!\");
            }"
    );
    assert_eq!(
        s,
        "    fn main() {\n        println!(\"Hello world 3!\");\n    }"
    );
}

#[test]
fn dedent_tabs() {
    let s = dedent!(
        "\
\t\t\t\tline1
\t\t\t\t
\t\t\t\tline3"
    );
    assert_eq!(s, "line1\n\nline3");
}

#[test]
fn dedent_tabs_2() {
    let s = dedent!(
        keep_ws = 2,
        "
\t\t\t\tline2
\t\t\t\t
\t\t\t\tline4
\t
"
    );
    assert_eq!(s, "\n\t\tline2\n\n\t\tline4\n\n");
}

#[test]
fn detect_mixed_tabs_and_spaces_same_line() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/mixed_indent_same_line.rs");
}

#[test]
fn detect_mixed_tabs_and_spaces_across_lines() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/mixed_indent_across_lines.rs");
}
