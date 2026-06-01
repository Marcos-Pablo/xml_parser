# xml_parser

A handwritten XML parser in Rust, built from scratch using **parser combinators**.

## Why this exists

I do [Advent of Code](https://adventofcode.com) every year, and I have to admit: my favorite part is often **parsing the input**. There's something deeply satisfying about turning a wall of raw text into a structured representation of the problem. Sometimes I find it more exciting than the actual puzzle solution.

That curiosity led me down a rabbit hole. I started wondering: how do people actually build *real* parsers? Not `split()` and `parse::<i32>()` hacks, but proper, composable parsing machinery. I stumbled across a tutorial on **parser combinators** — the idea of building complex parsers by combining small, reusable ones — and decided to implement one myself.

This project is the result: a fully functional XML parser built entirely from first principles, using nothing but functions and traits.

## What are parser combinators?

Parser combinators are a technique for building parsers by combining simpler parsers. Instead of writing one monolithic parsing function, you write small primitives (like "match this literal string" or "parse an identifier") and then combine them using combinators like:

- **Sequence**: parse A, then B (`pair`)
- **Choice**: try A, or if that fails, try B (`either`)
- **Map**: parse A, then transform the result (`map`)
- **Repeat**: parse zero or more of A (`zero_or_more`)
- **Predicate**: parse A, but only if it satisfies a condition (`pred`)

The neat thing is that these combinators are just functions. A parser is anything that implements `Fn(&str) -> Result<(&str, Output), &str>` — take a string, consume some of it, and return the parsed value along with the remaining input.

## Quick start

```rust
use xml_parser::{element, Element, Parser};

let input = r#"
    <top label="Top">
        <semi-bottom label="Bottom"/>
        <middle>
            <bottom label="Another bottom"/>
        </middle>
    </top>
"#;

let result = element().parse(input);

assert!(result.is_ok());
```

## Project structure

The project is organized into four focused modules:

| Module | Purpose |
|--------|---------|
| `core` | The `Parser` trait, `ParseResult` type alias, and `BoxedParser` for type-erased parsers |
| `combinators` | Generic parser combinators (`map`, `pair`, `either`, `and_then`, etc.) and the `ParserExt` trait for method chaining |
| `primitives` | Base parsers: `match_literal`, `identifier`, `any_char`, whitespace handling |
| `xml` | XML-specific types and parsers (`Element`, `element()`, attribute parsing, nested element support) |

## Features

- **Self-closing tags**: `<div class="float"/>`
- **Nested elements**: proper parent/child relationships
- **Attributes**: `name="value"` pairs with quoted strings
- **Whitespace tolerance**: ignores whitespace around elements
- **Error reporting**: tells you where parsing failed
- **Method chaining**: fluent API via `ParserExt` (`.map()`, `.pred()`, `.and_then()`)

## A note on the tutorial

This project was built while following a tutorial on parser combinators in Rust. The original single-file implementation has been refactored into a cleaner, modular structure with a focused public API.

## License

MIT
