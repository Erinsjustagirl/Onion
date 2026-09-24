use logos::Logos;
use winnow::prelude::*;
use winnow::combinator::*;
use winnow::token::*;
use winnow::error::ContextError;
type TokenStream<'a> = &'a [Token];
// ai filled in the lexer because im lazy wehhh - Erin
#[derive(Logos, Debug, PartialEq, Clone)]
enum Token {
    // === KEYWORDS (must come BEFORE Ident) ===
    #[token("num")] KwNum,
    #[token("deci")] KwDeci,
    #[token("string")] KwString,
    #[token("void")] KwVoid,
    #[token("func")] KwFunc,
    #[token("if")] KwIf,
    #[token("else")] KwElse,
    #[token("while")] KwWhile,
    #[token("for")] KwFor,
    #[token("return")] KwReturn,
    #[token("print")] KwPrint,
    #[token("and")] KwAnd,
    #[token("or")] KwOr,
    #[token("bool")] KwBool,
    #[token("set")] KwSet,

    // === IDENTIFIER ===
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),

    // === LITERALS ===
    #[regex(r"[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    Num(i64),

    #[regex(r"[0-9]+\.[0-9]+", |lex| lex.slice().parse::<f64>().unwrap())]
    Deci(f64),

    #[regex(r#""[^"]*""#, |lex| lex.slice().to_string())]
    String(String),

    // === SYMBOLS ===
    #[token("{")] Lbrace,
    #[token("}")] Rbrace,
    #[token("[")] Lbrack,
    #[token("]")] Rbrack,
    #[token("(")] Lparen,
    #[token(")")] Rparen,
    #[token(";")] Semicolon,
    #[token("=")] Equals,
    #[token("+")] Plus,
    #[token("-")] Minus,
    #[token("*")] Mult,
    #[token("/")] Slash,
    #[token("%")] Percent,
    #[token(">")] Gt,
    #[token("<")] Lt,
    #[token("==")] EqEq,
    #[token("!=")] NotEq,
    #[token("<=")] Lte,
    #[token(">=")] Gte,
    #[token("!")] Not,
    #[token(",")] Comma,

    #[regex(r"//.*", logos::skip)]
    _Comment,

    // === SKIP (whitespace + comments) ===
    #[regex(r"[ \t\n\r]+", logos::skip)]
    _Whitespace,
}

// AST - Erin
#[derive(Debug)]
struct Program {
    statements: Vec<Statement>,
}
#[derive(Debug)]
struct VariableDecl {
    kind: Type,
    name: String,
    value: Expression,
}
#[derive(Debug)]
struct FunctionCall {
    func_name: String,
    arguments: Expression,
}
#[derive(Debug)]
struct IfStatement {
    condition: Expression,
    body: Vec<Statement>,
    else_body: Option<Vec<Statement>>,
}
#[derive(Debug)]
struct WhileLoop {
    condition: Expression,
    body: Vec<Statement>,
}
#[derive(Debug)]
struct ReturnStatement {
    value: Expression,
}
#[derive(Debug)]
struct FunctionDecl {
    name: String,
    params: Vec<Param>,
    return_type: Type,
    body: Vec<Statement>,
}
#[derive(Debug)]
struct BinaryOp {
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
}
#[derive(Debug)]
struct Comparison {
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
}
#[derive(Debug)]
struct Assignment {
    name: String,
    value: Expression,
}
#[derive(Debug)]
enum Statement {
    VariableDecl(VariableDecl),
    Assignment(Assignment),
    FunctionCall(FunctionCall),
    IfStatement(IfStatement),
    WhileLoop(WhileLoop),
    ReturnStatement(ReturnStatement),
    FunctionDecl(FunctionDecl),
}
#[derive(Debug)]
enum Expression {
    Num(i64),
    Deci(f64),
    String(String),
    Variable(String),
    BinaryOp(Box<BinaryOp>),
    Comparison(Box<Comparison>),
    Array(Vec<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
enum Type {
    Num,
    Deci,
    String,
    Bool,
    Void,
    Array(Box<Type>),
}
#[derive(Debug)]
struct Param {
    kind: Type,
    name: String,
}

// Parser (the ai wrote some of this sorry) - Erin

fn token_parser(expected: Token) -> impl FnMut(&mut TokenStream) -> PResult<Token> {
    move |input: &mut TokenStream| {
        if input.is_empty() {
            return Err(winnow::error::ErrMode::Backtrack(
                winnow::error::ContextError::new()
            ));
        }
        let actual: Token = input[0].clone();
        if actual == expected {
            *input = &input[1..];
            Ok(actual)
        } else {
            Err(winnow::error::ErrMode::Backtrack(
                winnow::error::ContextError::new()
            ))
        }
    }
}

fn ident_parser(input: &mut TokenStream) -> PResult<String> {
    match input.first() {
        Some(Token::Ident(name)) => {
            let name: String = name.clone();
            *input = &input[1..];
            Ok(name)
        }
        _ => Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new()
        )),
    }
}

fn num_parser(input: &mut TokenStream) -> PResult<i64> {
    match input.first() {
        Some(Token::Num(n)) => {
            let n: i64 = *n;
            *input = &input[1..];
            Ok(n)
        }
        _ => Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new()
        )),
    }
}

fn deci_parser(input: &mut TokenStream) -> PResult<f64> {
    match input.first() {
        Some(Token::Deci(n)) => {
            let n: f64 = *n;
            *input = &input[1..];
            Ok(n)
        }
        _ => Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new()
        )),
    }
}

fn string_parser(input: &mut TokenStream) -> PResult<String> {
    match input.first() {
        Some(Token::String(s)) => {
            let s: String = s.clone();
            *input = &input[1..];
            Ok(s)
        }
        _ => Err(winnow::error::ErrMode::Backtrack(
            winnow::error::ContextError::new()
        )),
    }
}

fn parse_variable_decl(input: &mut TokenStream) -> Result<VariableDecl, winnow::error::ErrMode<ContextError>> {
    let kind: Type = alt((
        token_parser(Token::KwNum).map(|_| Type::Num),
        token_parser(Token::KwDeci).map(|_| Type::Deci),
        token_parser(Token::KwString).map(|_| Type::String),
        token_parser(Token::KwBool).map(|_| Type::Bool),
    )).parse_next(input)?;

    let name: String = ident_parser(input)?;
    token_parser(Token::Equals).parse_next(input)?;
    let value: Expression = alt((
        num_parser.map(Expression::Num),
        deci_parser.map(Expression::Deci),
        string_parser.map(Expression::String),
    )).parse_next(input)?;
    token_parser(Token::Semicolon).parse_next(input)?;

    Ok(VariableDecl { kind, name, value })
}

fn parse_statement(input: &mut TokenStream) -> PResult<Statement> {
    match input.first() {
        Some(Token::KwNum) | Some(Token::KwDeci) | Some(Token::KwString) | Some(Token::KwBool) => {
            let decl: VariableDecl = parse_variable_decl(input)?;
            Ok(Statement::VariableDecl(decl))
        }
        Some(Token::KwSet) => {
            let assignment: Assignment = parse_assignment(input)?;
            Ok(Statement::Assignment(assignment))
        }
        _ => {
            Err(winnow::error::ErrMode::Backtrack(
                winnow::error::ContextError::new()
            ))
        }
    }
}
// ai filled out parse_assignment. it did it for me. AAAAUGH!!! - Erin
fn parse_assignment(input: &mut TokenStream) -> PResult<Assignment> {
    token_parser(Token::KwSet).parse_next(input)?;   // ← consume 'set'
    let name: String = ident_parser(input)?;
    token_parser(Token::Equals).parse_next(input)?;
    let value: Expression = alt((
        num_parser.map(Expression::Num),
        deci_parser.map(Expression::Deci),
        string_parser.map(Expression::String),
    )).parse_next(input)?;
    token_parser(Token::Semicolon).parse_next(input)?;
    Ok(Assignment { name, value })
}

fn parse_program(input: &mut TokenStream) -> PResult<Program> {
    let mut statements: Vec<Statement> = Vec::new();
    while !input.is_empty() {
        let stmt: Statement = parse_statement(input)?;
        statements.push(stmt);
    }
    Ok(Program { statements })
}

fn main() {
    let source: String = std::fs::read_to_string("example.on").unwrap();
    let mut lexer: logos::Lexer<'_, Token> = Token::lexer(&source);
    let tokens: Vec<Token> = lexer.collect::<Result<Vec<_>, _>>().unwrap();

    println!("Tokens: {:?}", tokens);

    let mut slice: &[Token] = &tokens[..];
    let result: Result<Program, winnow::error::ErrMode<ContextError>> = parse_program(&mut slice);
    println!("{:?}", result);
}