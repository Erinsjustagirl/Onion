use logos::Logos;
// ai filled in the lexer because im lazy wehhh - Erin
#[derive(Logos, Debug, PartialEq)]
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

struct Program {
    statements: Vec<Statement>,
}

struct VariableDecl {
    kind: String,
    name: String,
    value: Expression,
}

struct FunctionCall {
    func_name: String,
    arguments: Expression,
}

struct IfStatement {
    condition: Expression,
    body: Vec<Statement>,
    else_body: Option<Vec<Statement>>,
}

struct WhileLoop {
    condition: Expression,
    body: Vec<Statement>,
}

struct ReturnStatement {
    value: Expression,
}

struct FunctionDecl {
    name: String,
    params: Vec<Param>,
    return_type: String,
    body: Vec<Statement>,
}

struct BinaryOp {
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
}

struct Comparison {
    left: Box<Expression>,
    operator: String,
    right: Box<Expression>,
}

enum Statement {
    VariableDecl(VariableDecl),
    FunctionCall(FunctionCall),
    IfStatement(IfStatement),
    WhileLoop(WhileLoop),
    ReturnStatement(ReturnStatement),
    FunctionDecl(FunctionDecl),
}

enum Expression {
    Num(i64),
    Deci(f64),
    String(String),
    Variable(String),
    BinaryOp(Box<BinaryOp>),
    Comparison(Box<Comparison>),
}

struct Param {
    kind: String,
    name: String,
}

fn main() {
    let source: String = std::fs::read_to_string("example.on").unwrap();
    let mut lexer: logos::Lexer<'_, Token> = Token::lexer(&source);

    while let Some(token) = lexer.next() {
        match token {
            Ok(t) => println!("{:?}", t),
            Err(e) => println!("ERROR at {:?}: {:?}", lexer.span(), e),
        }
    }
}