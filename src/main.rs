use parser::parser::*;

fn main() {
    let text = Parser {
        input: String::from("42"),
    };

    println!("{:?}", Parser::parse(&text));
}
