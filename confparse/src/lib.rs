use crate::parse::Keyword;
use crate::parse::Symbol;
use crate::parse::Token;
use std::fs::read_to_string;
use std::sync::Arc;
use std::vec;
mod parse;

#[derive(Debug, Clone)]
pub struct Task {
    pub name: Arc<str>,
    pub args: Vec<Arc<str>>,
    pub requires: Vec<Arc<str>>,
    pub satisfies: Vec<Arc<str>>,
    pub cycles: u16
}

#[derive(Debug, Clone)]
pub struct Conf {
    pub inports: Vec<Arc<str>>,
    pub outports: Vec<Arc<str>>,
    pub initial: Vec<Arc<str>>,
    pub tasks: Vec<Task>
}

fn parse_keyword(next_token: Token, keyword_type: Keyword) -> Result<(), String> {
    // Errors if keyword_type does not matches
    let Token::Keyword(x) = next_token else {
        Err(format!("Not Keyword : {next_token:?}").to_string())?
    };
    if x != keyword_type {
        Err(format!("Different type keyword, expected : {keyword_type:?} , got: {next_token:?}").to_string())?
    };
    Ok(())
}

fn parse_symbol(next_token: Token, symbol_type: Symbol) -> Result<(), String> {
    // Errors if keyword_type does not matches
    let Token::Symbol(x) = next_token else {
        Err(format!("Not Symbol : {next_token:?}").to_string())?
    };
    if x != symbol_type {
        Err(format!("Different type keyword, expected : {symbol_type:?} , got: {next_token:?}").to_string())?
    };
    Ok(())
}

fn get_token(tokens: &mut std::slice::Iter<'_, Token>) -> Result<Token, String> {
    return tokens
        .next()
        .ok_or("Failed to extract token".to_string())
        .cloned();
}

fn populate( inports: &mut Vec<Arc<str>>, tokens: &mut std::slice::Iter<'_, Token>) -> Result<(), String>{
    let mut token = get_token( tokens)?;
    parse_symbol(token.clone(), Symbol::StartArray)?;

    loop {
        let tmp_token = match get_token(tokens) {
            Ok(token) => token,
            Err(e) => return Err(e), // Directly return on error
        };
    
        match tmp_token {
            Token::Literal(x) => {
                inports.push(x.into());
            }
            _ => {
                token = tmp_token;
                break;
            }
        }
    }
    
    parse_symbol(token.clone(), Symbol::EndArray)?;
    Ok(())
}

fn parse_conf(tokens: &mut std::slice::Iter<'_, Token>) -> Result<Conf, String> {
    let mut config = Conf {
        inports: vec![],
        initial: vec![],
        outports: vec![],
        tasks: vec![]
    };
    
    let mut token = get_token(tokens)?;
    parse_keyword(token.clone(), Keyword::IN)?;
    populate(&mut config.inports, tokens )?;
    
    token = get_token(tokens)?;
    parse_keyword(token.clone(), Keyword::OUT)?;
    populate(&mut config.outports,tokens)?;
    
    token = get_token(tokens)?;
    parse_keyword(token.clone(), Keyword::INIT_CONDITIONS)?;
    populate(&mut config.initial,tokens)?;
    
    Ok(config)
}

fn parse_tasks( tokens: &mut std::slice::Iter<'_, Token>) -> Result<Task, String>{
    let mut task = Task {
        name:"".into(),
        args:vec![],
        requires:vec![],
        satisfies:vec![],
        cycles: 0
    };
    // requires
    // Checking manually so if no token the we return NULL
    let mut token = match get_token(tokens) {
        Ok(token) => token,
        Err(_) => Err("EMPTY")? // TODO: to later convert all thses errors to ENUMS
    };
    parse_keyword(token.clone(), Keyword::REQUIRES)?;
    populate(&mut task.requires, tokens)?;
    //

    token = get_token(tokens)?;
    parse_keyword(token.clone(), Keyword::TASK)?;
    
    // task name
    token = get_token(tokens)?;
    let Token::Literal(x) = token else {
        Err("Task Name Expected")?
    };
    task.name = <String as Into<Arc<str>>>::into(x.to_string());
    populate(&mut task.args, tokens)?;
    // 

    // cycles
    token = get_token(tokens)?;
    let Token::Literal(x) = token else {
        Err("Task Name Expected")?
    };
    task.cycles = x.parse().unwrap();
    //

    // satisfies
    token = get_token(tokens)?;
    parse_keyword(token.clone(), Keyword::SATISFIES)?;
    populate(&mut task.satisfies, tokens)?;
    //
    
    Ok(task)
}

fn coder(tokens: Vec<Token>) -> Result<Conf, String>{
    let mut tokens_iter: std::slice::Iter<'_, Token> = tokens.iter();
    let mut config = parse_conf(&mut tokens_iter)?;
    loop {
        let task = match parse_tasks(&mut tokens_iter) {
            Ok(t) => t,
            Err(e) => match e.as_str() {
                "EMPTY" => {
                    break;
                }
                _ => {
                    Err(e)?
                }
            }
        };
        config.tasks.push(task);
    }
    Ok(config)

}
pub fn get_conf(path: &str) -> Result<Conf, String> {
    let content = read_to_string(path).map_err(|e| e.to_string())?;
    let tokens = parse::parse(&content).map_err(|e| e.to_string())?;
    coder(tokens)
}