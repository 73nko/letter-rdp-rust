type NumericalLiteral = u64;

#[derive(Debug)]
pub struct Node {
    kind: String,
    value: NumericalLiteral,
}

pub trait ParserLanguage {
    fn parse(&self) -> Option<Node>;
    fn program(&self) -> Option<Node>;
    fn numerical_literal(&self) -> Option<Node>;
}

#[derive(Debug)]
pub struct Parser {
    pub input: String,
}

impl ParserLanguage for Parser {
    fn parse(&self) -> Option<Node> {
        self.program()
    }

    fn program(&self) -> Option<Node> {
        self.numerical_literal()
    }

    fn numerical_literal(&self) -> Option<Node> {
        let value = self.input.parse::<NumericalLiteral>().unwrap();

        Some(Node {
            kind: String::from("NumericalLiteral"),
            value: value,
        })
    }
}
