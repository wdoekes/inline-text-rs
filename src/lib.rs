use proc_macro::TokenStream;

mod collapse;
mod concat;
mod dedent;


/// `collapse!` is a compile-time macro that normalizes whitespace in a string literal.
///
/// It removes all leading and trailing ASCII whitespace (`' '`, `\t`, `\n`, `\r`)
/// and replaces any internal sequences of whitespace with a single space.
///
/// # Example
/// ```
/// let s = inline_text::collapse!("
///     SELECT t.* FROM table t
///     INNER JOIN jtable j ON j.t_id = t.id
///     WHERE j.status IN (
///       'yes', 'maybe'
///     );
/// ");
/// assert_eq!(s, "SELECT t.* FROM table t INNER JOIN jtable j ON j.t_id = t.id WHERE j.status IN ( 'yes', 'maybe' );");
/// ```
///
/// Only string literals are accepted as input. Use for formatting static strings at compile time.
#[proc_macro]
pub fn collapse(input: TokenStream) -> TokenStream {
    collapse::collapse_impl(input)
}


/// Concatenates adjacent string literals at compile time.
///
/// It joins the string arguments together.
///
/// # Example
/// ```
/// let s = inline_text::concat!(
///   "This is a text ",
///   "that might span ",
///   "multiple lines.\n",
///   "But only if you ",
///   "manually add the ",
///   "LFs.\n"
/// );
/// assert_eq!(s, "This is a text that might span multiple lines.\nBut only if you manually add the LFs.\n");
/// ```
///
/// Only string literals are accepted as input. Use for formatting static strings at compile time.
#[proc_macro]
pub fn concat(input: TokenStream) -> TokenStream {
    concat::concat_impl(input)
}


/// Removes uniform leading indentation and optional blank lines from a multi-line string literal.
///
/// It unindents/dedents inline string text. For instance inline SQL
///
/// # Example
/// ```
/// let s = inline_text::dedent!("
///       SIX
///     FOUR
///   TWO");
/// assert_eq!(s, "\n    SIX\n  FOUR\nTWO");
///
/// let s2 = inline_text::dedent!(keep_ws=4, "\
///         fn main() {
///             println!(\"Hello world!\");
///         }
/// ");
/// assert_eq!(s2, "    fn main() {\n        println!(\"Hello world!\");\n    }\n");
/// ```
///
/// Only string literals are accepted as input. Use for formatting static strings at compile time.
#[proc_macro]
pub fn dedent(input: TokenStream) -> TokenStream {
    dedent::dedent_impl(input)
}
