// ========================
// ===== QUANTIFIERS ======
// ========================

#[derive(Debug, Clone)]
pub enum QuantifierKind {
    Range { start: u32, end: u32 },
    Some,
    Any,
    Over(u32),
    Option,
    Amount(u32),
}

#[derive(Debug, Clone)]
pub struct Quantifier {
    pub kind: QuantifierKind,
    pub lazy: bool,
    pub expression: Box<Expression>,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Atom(String),
    Symbol(Symbol)
}

// ========================
// ======= SYMBOLS ========
// ========================

#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub negated: bool,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Space,
    Newline,
    Vertical,
    Return,
    Tab,
    Null,
    Whitespace,
    Alphabetic,
    Alphanumeric,
    Char,
    Digit,
    Word,
    Feed,
    Backspace,
    Boundary,
}

// ========================
// ========= AST ==========
// ========================

#[derive(Debug, Clone)]
pub enum AlaniAstNode {
    Quantifier(Quantifier),
    Atom(String),
    Symbol(Symbol),
    Skip,
}

#[derive(Debug, Clone)]
pub enum AlaniAst {
    Root(Vec<AlaniAstNode>),
    Empty,
}