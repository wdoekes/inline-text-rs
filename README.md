# inline-text

**Procedural macros for inline string formatting in Rust.**

This crate provides compile-time utilities for manipulating multiline
string literals, including removing common indentation, collapsing
whitespace, and concatenating fragments.

This is inspired by the discussions at [rust-lang RFCs PR
3830](https://github.com/rust-lang/rfcs/pull/3830) where a new
`d"`-string is discussed, for detenting.

---

## ✨ Macros

### [`collapse!`]

Removes leading/trailing whitespace and collapses all internal sequences
of whitespace (`[ \t\r\n]+`) into a single space.

```rust
use inline_text::collapse;

let s = collapse!("
    SELECT t.* FROM table t
    INNER JOIN jtable j ON j.t_id = t.id
    WHERE j.status IN (
      'yes', 'maybe'
    );
");
assert_eq!(s, "SELECT t.* FROM table t INNER JOIN jtable j ON j.t_id = t.id WHERE j.status IN ( 'yes', 'maybe' );");
```


### [`concat!`]

Concatenates adjacent string literals at compile time. Useful when
you're dealing with long strings and want to break early to avoid
exceeding the source line length.

```rust
use inline_text::concat;

let s = concat!(
    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\n",
    "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB",
    "BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB\n",
);
//assert_eq!(s, "AAA...AAA\nBBB...BBB\n");
assert_eq!(s, format!("{}\n{}\n", "A".repeat(144), "B".repeat(144)));
```

*NOTE: This is a dumber version of the `std::concat!` macro. Only useful
if expanded with more capabilities, like adding LFs.*


### [`dedent!`]

Removes uniform leading indentation from a multiline string. Mandates
that the leading whitespace consistently uses either spaces or tabs.

```rust
use inline_text::dedent;

let s = dedent!("\
    SELECT t.* FROM table t
    INNER JOIN jtable j ON j.t_id = t.id
    WHERE j.status IN (
      'yes', 'maybe'
    );
");
assert_eq!(s, "\
SELECT t.* FROM table t
INNER JOIN jtable j ON j.t_id = t.id
WHERE j.status IN (
  'yes', 'maybe'
);
");
```

**Optional:** preserve some indentation:
```rust
let s = dedent!(keep_ws=2, "\
	    CREATE TABLE student (
	      id INT PRIMARY KEY,

	      -- firstname + lastname
	      name TEXT
	    );
");
assert_eq!(s, "  CREATE TABLE student (\n    id INT PRIMARY KEY,\n\n    -- firstname + lastname\n    name TEXT\n  );\n");
```


## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
inline-text = "0.1"
```
