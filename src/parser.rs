use crate::types;
use types::Expression;

struct Parser {
    // Need an Expression here
    index: usize,         // Where it is currently!
    asf: Vec<Expression>, // Abstract syntax forest!
}

impl Parser {
    fn new() -> Parser {
        Parser {
            index: 0,
            asf: Vec::new(),
        }
    }

    fn push_expr(&mut self, e: Expression) {
        self.asf.push(e);
    }
}

fn is_number(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }

    s.parse::<f64>().is_ok()
}

fn parse_list(tokens: &Vec<String>, parser: &mut Parser) -> Expression {
    let mut l: Vec<Expression> = Vec::new();
    while parser.index < tokens.len() && tokens[parser.index].as_str() != ")" {
        l.push(parse_expression(tokens, parser));
    }
    parser.index += 1;
    if l.is_empty() || parser.index > tokens.len() {
        Expression::Nil
    } else {
        Expression::List(l)
    }
}

fn parse_expression(tokens: &Vec<String>, parser: &mut Parser) -> Expression {
    if parser.index >= tokens.len() {
        return Expression::Nil;
    }

    let input = tokens[parser.index].as_str();
    match input {
        // Parse a LIST here
        "(" => {
            parser.index += 1;
            parse_list(tokens, parser)
        }
        // Parse a quote here
        "'" => {
            parser.index += 1;
            let e = Expression::Quote(Box::new(parse_expression(tokens, parser)));
            e
        }
        // Parse an unquote here
        "," => {
            parser.index += 1;
            let e = Expression::Unquote(Box::new(parse_expression(tokens, parser)));
            e
        }
        // Parse a boolean here
        "true" | "t" => {
            let e = Expression::Boolean(true);
            parser.index += 1;
            e
        }
        "false" | "f" => {
            let e = Expression::Boolean(false);
            parser.index += 1;
            e
        }

        "nil" => {
            parser.index += 1;
            let e = Expression::Nil;
            e
        }
        // Parse a string here!
        s if s.starts_with('\"') && s.ends_with('\"') => {
            parser.index += 1;
            let text = &s[1..s.len() - 1];
            let e = Expression::String(text.to_string());
            e
        }
        // Parse a number here!
        s if is_number(s) => {
            parser.index += 1;
            let num = s.parse::<f64>().unwrap();
            let e = Expression::Number(num);
            e
        }
        // Parse a symbol here!
        s if (s.chars().next().is_some_and(|c| c.is_alphabetic())
            && s.chars().all(|c| c.is_alphanumeric()))
            || (s.len() == 1 && "+-*/%=".contains(s)) =>
        {
            parser.index += 1;
            let e = Expression::Symbol(s.to_string());
            e
        }
        _ => {
            parser.index += 1;
            let e = Expression::Nil;
            e
        }
    }
}

pub fn parse(tokens: Vec<String>) -> Vec<Expression> {
    if tokens.is_empty() {
        return vec![Expression::Nil];
    }

    let mut parser = Parser::new();

    while parser.index < tokens.len() {
        let e = parse_expression(&tokens, &mut parser);
        parser.push_expr(e);
    }

    return parser.asf;
}
