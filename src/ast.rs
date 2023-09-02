use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub start: Position,
    pub end: Position,
}

pub type Range = (u32, u32);

#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Program {
    pub r#type: String,
    pub loc: Location,
    pub range: Range,
    pub body: Vec<Statement>,
    pub tokens: Vec<Token>,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Statement {
    DebuggerStatement(DebuggerStatement),
    ExpressionStatement(ExpressionStatement),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DebuggerStatement {
    pub loc: Location,
    pub range: Range,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpressionStatement {
    pub loc: Location,
    pub range: Range,
    pub expression: Box<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    Literal(Literal),
    Identifier(Identifier),
    CallExpression(CallExpression),
    MemberExpression(MemberExpression),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Identifier {
    pub loc: Location,
    pub range: Range,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Literal {
    pub loc: Location,
    pub range: Range,
    pub value: Value,
    pub raw: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Value {
    String(String),
    Number(f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CallExpression {
    pub loc: Location,
    pub range: Range,
    pub callee: Box<Expression>,
    pub arguments: Vec<ExprOrSpread>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ExprOrSpread {
    Expr(Expression),
    Spread(SpreadElement),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpreadElement {
    pub loc: Location,
    pub range: Range,
    pub argument: Box<Expression>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemberExpression {
    pub loc: Location,
    pub range: Range,
    pub object: Box<Expression>,
    pub property: Box<Prop>,
    pub computed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Prop {
    Identifier(Identifier),
    // TODO:
    // PrivateName(PrivateName),
    // Computed(ComputedPropName),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Token {
    Identifier(TokenValue),
    Keyword(TokenValue),
    String(TokenValue),
    Punctuator(TokenValue),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenValue {
    pub loc: Location,
    pub range: Range,
    pub value: String,
}
