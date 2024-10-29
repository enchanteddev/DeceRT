enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Literal(String)
}


enum Symbol {
    LParen,
    RParen,
    Comma,
    Colon,
    At
}


enum Keyword {
    IN,
    OUT,
    #[allow(non_camel_case_types)]
    INIT_CONDITIONS,
    TASK,
    REQUIRES,
    SATISFIES
}


fn parse(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let lines = input.lines();
    for line in lines.filter(|f| !f.is_empty()) {
        
    }
    tokens
}