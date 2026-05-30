trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input).and_then(|(next_input, result1)| {
            parser2
                .parse(next_input)
                .map(|(last_input, result2)| (last_input, (result1, result2)))
        })
    }
}

fn left<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(left, _right)| left)
}

fn right<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    map(pair(parser1, parser2), |(_left, right)| right)
}

fn one_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_item)) = parser.parse(input) {
            input = next_input;
            result.push(first_item);
        } else {
            return Err(input);
        }

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

fn zero_or_more<'a, P, A>(parser: P) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_item)) = parser.parse(input) {
            input = next_input;
            result.push(next_item);
        }

        Ok((input, result))
    }
}

fn match_literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.starts_with(expected) {
        true => Ok((&input[expected.len()..], ())),
        false => Err(input),
    }
}

fn identifier<'a>(input: &'a str) -> ParseResult<'a, String> {
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

fn any_char(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some(next) => Ok((&input[next.len_utf8()..], next)),
        None => Err(input),
    }
}

fn pred<'a, P, A, F>(parser: P, predicate: F) -> impl Parser<'a, A>
where
    P: Parser<'a, A>,
    F: Fn(&A) -> bool,
{
    move |input| {
        if let Ok((next_input, value)) = parser.parse(input) {
            if predicate(&value) {
                return Ok((next_input, value));
            }
        }

        Err(input)
    }
}

fn whitespace_char<'a>() -> impl Parser<'a, char> {
    pred(any_char, |c| c.is_whitespace())
}

fn space1<'a>() -> impl Parser<'a, Vec<char>> {
    one_or_more(whitespace_char())
}

fn space0<'a>() -> impl Parser<'a, Vec<char>> {
    zero_or_more(whitespace_char())
}

fn quoted_string<'a>() -> impl Parser<'a, String> {
    map(
        right(
            match_literal("\""),
            left(
                zero_or_more(pred(any_char, |c| *c != '"')),
                match_literal("\""),
            ),
        ),
        |chars| chars.into_iter().collect(),
    )
}

fn attribute_pair<'a>() -> impl Parser<'a, (String, String)> {
    pair(identifier, right(match_literal("="), quoted_string()))
}

fn attributes<'a>() -> impl Parser<'a, Vec<(String, String)>> {
    zero_or_more(right(space1(), attribute_pair()))
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

    #[test]
    fn pair_combinator() {
        let tag_opener = pair(match_literal("<"), identifier);

        assert_eq!(
            tag_opener.parse("<my-first-element/>"),
            Ok(("/>", ((), "my-first-element".to_string())))
        );

        assert_eq!(tag_opener.parse("oops"), Err("oops"));
        assert_eq!(tag_opener.parse("<!oops"), Err("!oops"));
    }

    #[test]
    fn right_combinator() {
        let tag_opener = right(match_literal("<"), identifier);

        assert_eq!(
            tag_opener.parse("<my-first-element/>"),
            Ok(("/>", "my-first-element".to_string()))
        );

        assert_eq!(tag_opener.parse("oops"), Err("oops"));
        assert_eq!(tag_opener.parse("<!oops"), Err("!oops"));
    }

    #[test]
    fn zero_or_more_combinator() {
        let parser = zero_or_more(match_literal("ha"));

        assert_eq!(parser.parse("hahaha"), Ok(("", vec![(), (), ()])));
        assert_eq!(parser.parse("ahah"), Ok(("ahah", vec![])));
        assert_eq!(parser.parse(""), Ok(("", vec![])));
    }

    #[test]
    fn one_or_more_combinator() {
        let parser = one_or_more(match_literal("ha"));

        assert_eq!(parser.parse("hahaha"), Ok(("", vec![(), (), ()])));
        assert_eq!(parser.parse("ahah"), Err("ahah"));
        assert_eq!(parser.parse(""), Err(""));
    }

    #[test]
    fn predicate_combinator() {
        let parser = pred(any_char, |c| *c == 'o');
        assert_eq!(Ok(("mg", 'o')), parser.parse("omg"));
        assert_eq!(Err("lol"), parser.parse("lol"));
    }

    #[test]
    fn quoted_strings_parser() {
        let parser = quoted_string();

        assert_eq!(
            Ok(("", "Hello Joe!".to_string())),
            parser.parse("\"Hello Joe!\"")
        );
    }
}
