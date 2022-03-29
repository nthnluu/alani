use super::constants::{LAZY, NOT};
use super::parser::{AlaniParser, Rule};
use super::types::*;
use super::utils::{
    alphabetic_first_char, first_inner, first_last_inner_str, last_inner, nth_inner, to_char,
    unquote_escape_literal, unquote_escape_raw,
};
use crate::errors::CompilerError;
use anyhow::Result;
use pest::iterators::Pairs;
use pest::{iterators::Pair, Parser};
use std::collections::HashMap;

// Converts the source file (as a string) to an AST
pub fn to_ast(source: &str) -> Result<AlaniAst> {
    if source.is_empty() {
        return Ok(AlaniAst::Empty);
    }

    let mut pairs = AlaniParser::parse(Rule::root, source)?;

    let root_statements = pairs.next().ok_or(CompilerError::MissingRootNode)?;

    // This hashmap is used as an environment for variables as we traverse the tokens
    let mut env: HashMap<String, AlaniAst> = HashMap::new();

    pairs_to_ast(root_statements.into_inner(), &mut env)
}

// Converts a set of tokens into an AST
pub fn pairs_to_ast(pairs: Pairs<Rule>, env: &mut HashMap<String, AlaniAst>) -> Result<AlaniAst> {
    let mut nodes = Vec::new();

    // Iterate through tokens and create AST nodes
    for pair in pairs {
        let node = create_ast_node(pair, env)?;
        nodes.push(node);
    }

    Ok(AlaniAst::Root(nodes))
}

// Converts a token into an AST node
fn create_ast_node(pair: Pair<Rule>, env: &mut HashMap<String, AlaniAst>) -> Result<AlaniAstNode> {
    let node = match pair.as_rule() {
        Rule::raw => AlaniAstNode::Atom(unquote_escape_raw(&pair)),
        Rule::literal => AlaniAstNode::Atom(unquote_escape_literal(&pair)),
        Rule::symbol => parse_symbol(pair)?,
        // Rule::range => range(pair)?, NOT YET IMPLEMENTED
        Rule::quantifier => parse_quantifier(pair, env)?,
        // Rule::group => group(pair, env)?, NOT YET IMPLEMENTED
        // Rule::assertion => assertion(pair, env)?, NOT YET IMPLEMENTED
        // Rule::negative_char_class => negative_char_class(&pair)?, NOT YET IMPLEMENTED
        // Rule::variable_invocation => variable_invocation(&pair, env)?, NOT YET IMPLEMENTED
        // Rule::variable_declaration => variable_declaration(pair, env)?, NOT YET IMPLEMENTED
        Rule::EOI => AlaniAstNode::Skip,
        _ => return Err(CompilerError::UnrecognizedSyntax.into()),
    };

    Ok(node)
}

// Converts a symbol token into a symbol AST node
fn parse_symbol(pair: Pair<Rule>) -> Result<AlaniAstNode> {
    let (negated, symbol) = first_last_inner_str(pair)?;

    let negated = negated == NOT;

    // The special 'start' and 'stop' symbols must not be negated; thus we error check
    if negated {
        match symbol {
            "start" => return Err(CompilerError::NegativeStartNotAllowed.into()),
            "end" => return Err(CompilerError::NegativeEndNotAllowed.into()),
            _ => {}
        }
    }

    let symbol_node = match symbol {
        "space" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Space,
            negated,
        }),
        "newline" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Newline,
            negated,
        }),
        "vertical" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Vertical,
            negated,
        }),
        "word" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Word,
            negated,
        }),
        "digit" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Digit,
            negated,
        }),
        "whitespace" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Whitespace,
            negated,
        }),
        "boundary" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Boundary,
            negated,
        }),
        "alphabetic" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Alphabetic,
            negated,
        }),
        "alphanumeric" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Alphanumeric,
            negated,
        }),
        "return" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Return,
            negated,
        }),
        "tab" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Tab,
            negated,
        }),
        "null" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Null,
            negated,
        }),
        "feed" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Feed,
            negated,
        }),
        "char" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Char,
            negated,
        }),
        "backspace" => AlaniAstNode::Symbol(Symbol {
            kind: SymbolKind::Backspace,
            negated,
        }),

        // "start" => AlaniAstNode::SpecialSymbol(SpecialSymbol::Start),
        // "end" => AlaniAstNode::SpecialSymbol(SpecialSymbol::End),
        _ => return Err(CompilerError::UnrecognizedSymbol.into()),
    };

    Ok(symbol_node)
}

// Converts a quantifier token into a quantifer AST node
fn parse_quantifier(
    pair: Pair<Rule>,
    variables: &mut HashMap<String, AlaniAst>,
) -> Result<AlaniAstNode> {
    let quantity = first_inner(pair.clone())?;
    let kind = first_inner(quantity.clone())?;
    let expression = create_ast_node(last_inner(pair)?, variables)?;

    let expression = match expression {
        // AlaniAstNode::Group(group) => Expression::Group(group),
        AlaniAstNode::Atom(atom) => Expression::Atom(atom),
        // AlaniAstNode::Range(range) => Expression::Range(range),
        AlaniAstNode::Symbol(symbol) => Expression::Symbol(symbol),
        // AlaniAstNode::NegativeCharClass(class) => Expression::NegativeCharClass(class),

        // unexpected nodes
        // AlaniAstNode::SpecialSymbol(_) => {
        //     return Err(CompilerError::UnexpectedSpecialSymbolInQuantifier.into())
        // }
        AlaniAstNode::Quantifier(_) => {
            return Err(CompilerError::UnexpectedQuantifierInQuantifier.into())
        }
        // AlaniAstNode::Assertion(_) => {
        //     return Err(CompilerError::UnexpectedAssertionInQuantifier.into())
        // }
        // AlaniAstNode::VariableInvocation(_) => {
        //     return Err(CompilerError::UnexpectedVariableInvocationInQuantifier.into())
        // }
        AlaniAstNode::Skip => return Err(CompilerError::UnexpectedSkippedNodeInQuantifier.into()),
    };

    let lazy = quantity.as_str().starts_with(LAZY);


}
