use std::sync::Arc;
use crate::parse::Token;
use crate::parse::Keyword;
use crate::parse::Symbol;
mod parse;


struct Task {
    name: Arc<str>,
    args: Vec<Arc<str>>,
    requires: Vec<Arc<str>>,
    satisfies: Vec<Arc<str>>,
    cycles: u16
}


struct Conf {
    inports: Vec<Arc<str>>,
    outports: Vec<Arc<str>>,
    initial: Vec<Arc<str>>,
    tasks: Vec<Task>
}


fn parse_keyword(next_token:&Token,keyword_type:Keyword) -> bool{
    // return true if successfull
    return true;
}

fn parse_identifier(next_token:&Token) -> bool {
    // returns true if identifier
    return true;
}

fn parse_identifier(next_token:&Token,symbol_type:Symbol) -> bool {
    // returns true if identifier
    return true;
}

fn getToken(tokens:&mut std::slice::Iter<'_, Token>) -> Result<&Token, String> {
    return tokens.next().ok_or("Failed to extract token");
}

fn parse_conf( mut tokens:std::slice::Iter<'_, Token>) -> Result<(), String>{
    let mut token= getToken(&mut tokens)?;
    parse_keyword(token, Keyword::IN);
    while (parse_identifier(token)){
        token = getToken(&mut tokens)?;
    }
    Ok(())
}

fn parse_tasks(tokens:std::slice::Iter<'_, Token>) {

}

pub fn coder(tokens:Vec<Token>) {
    let tokens_iter: std::slice::Iter<'_, Token>  = tokens.iter();
    parse_conf(tokens_iter);   
}

