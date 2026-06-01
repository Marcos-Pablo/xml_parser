use crate::combinators::{one_or_more, pred, zero_or_more};
use crate::core::{ParseResult, Parser};

pub(crate) fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.starts_with(expected) {
        true => Ok((&input[expected.len()..], ())),
        false => Err(input),
    }
}

pub(crate) fn identifier<'a>(input: &'a str) -> ParseResult<'a, String> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() => matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next() {
        if next.is_alphanumeric() || next == '-' {
            matched.push(next);
            continue;
        }

        break;
    }

    let next_index = matched.len();

    Ok((&input[next_index..], matched))
}

pub(crate) fn any_char<'a>(input: &'a str) -> ParseResult<'a, char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        None => Err(input),
    }
}

pub(crate) fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}

pub(crate) fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

pub(crate) fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal_parser() {
        let parse_joe = match_literal("Hello Joe!");
        assert_eq!(parse_joe.parse("Hello Joe!"), Ok(("", ())));
        assert_eq!(
            parse_joe.parse("Hello Joe! Hello Robert!"),
            Ok((" Hello Robert!", ()))
        );
        assert_eq!(parse_joe.parse("Hello Mike!"), Err("Hello Mike!"));
    }

    #[test]
    fn identifier_parser() {
        assert_eq!(
            identifier("i-am-an-identifier"),
            Ok(("", "i-am-an-identifier".to_string()))
        );

        assert_eq!(
            identifier("not entirely an identifier"),
            Ok((" entirely an identifier", "not".to_string()))
        );

        assert_eq!(
            identifier("!not at all an identifier"),
            Err("!not at all an identifier")
        );
    }
}
