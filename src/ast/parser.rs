use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "ast/alani.pest"]
pub struct AlaniParser;