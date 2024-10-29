use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Literal(Arc<str>),
}

#[derive(Debug, Clone, Copy)]
pub enum Symbol {
    StartArray,
    EndArray
}

#[derive(Debug, Clone, Copy)]
pub enum Keyword {
    IN,
    OUT,
    #[allow(non_camel_case_types)]
    INIT_CONDITIONS,
    TASK,
    REQUIRES,
    SATISFIES,
}

pub fn parse(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let lines = input.lines();

    let mut found_in = false;
    let mut found_out = false;
    let mut found_init = false;

    for line in lines.filter(|f| !f.is_empty()).map(|f| f.trim()) {
        if line.starts_with("IN") {
            if found_in {
                Err("Another IN found")?;
            }
            tokens.push(Token::Keyword(Keyword::IN));
            tokens.push(Token::Symbol(Symbol::StartArray));
            tokens.extend(
                line[3..]
                .split(',')
                .map(|f| Token::Literal(f.trim().into())),
            );
            tokens.push(Token::Symbol(Symbol::EndArray));
            found_in = true;

        } else if line.starts_with("OUT") {
            if found_out {
                Err("Another OUT found")?;
            }
            tokens.push(Token::Keyword(Keyword::OUT));
            tokens.push(Token::Symbol(Symbol::StartArray));
            tokens.extend(
                line[4..]
                    .split(',')
                    .map(|f| Token::Literal(f.trim().into())),
            );
            tokens.push(Token::Symbol(Symbol::EndArray));
            found_out = true;
        } else if line.starts_with("INIT_CONDITIONS") {
            if found_init {
                Err("Another INIT_CONDITIONS found")?;
            }
            tokens.push(Token::Keyword(Keyword::INIT_CONDITIONS));
            tokens.push(Token::Symbol(Symbol::StartArray));
            tokens.extend(
                line[16..]
                    .split(',')
                    .map(|f| Token::Literal(f.trim().into())),
            );
            tokens.push(Token::Symbol(Symbol::EndArray));
            found_init = true;
        } else if line.starts_with("@") {
            match &line[1..] {
                "requires" => {
                    tokens.push(Token::Keyword(Keyword::REQUIRES));
                    tokens.push(Token::Symbol(Symbol::StartArray));
                    tokens.extend(
                        line[10..]
                        .split(',')
                        .map(|f| Token::Literal(f.trim().into())),
                    );
                    tokens.push(Token::Symbol(Symbol::EndArray));
                }
                "satisfies" => {
                    tokens.push(Token::Keyword(Keyword::SATISFIES));
                    tokens.push(Token::Symbol(Symbol::StartArray));
                    tokens.extend(
                        line[11..]
                            .split(',')
                            .map(|f| Token::Literal(f.trim().into())),
                    );
                    tokens.push(Token::Symbol(Symbol::EndArray));
                }
                _ => Err("Unknown keyword after @")?,
            }
        } else if line.starts_with("Task") {
            tokens.push(Token::Keyword(Keyword::TASK));
            let (task_name, args_and_cycle) = line[5..].split_once("(").ok_or("Missing '(' after task name")?;
            let (args, cycle_str) = args_and_cycle.split_once(")").ok_or("Missing ')' after task args")?;
            let cycle = cycle_str[1..].parse::<u16>().map_err(|_| "Cycle must be a number")?;
            tokens.push(Token::Literal(task_name.trim().into()));
            tokens.push(Token::Symbol(Symbol::StartArray));
            tokens.extend(
                args
                    .split(',')
                    .map(|f| Token::Literal(f.trim().into())),
            );
            tokens.push(Token::Symbol(Symbol::EndArray));
            tokens.push(Token::Literal(cycle.to_string().into()));
        }
    }
    Ok(tokens)
}
