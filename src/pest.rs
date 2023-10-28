use std::{error::Error, io::Write};

use pest::iterators::Pair;

#[derive(pest_derive::Parser)]
#[grammar = "pest/crs.pest"]
struct CRSParser;

pub trait Serialize {
    type Error: Error;

    fn serialize<W: Write>(&self, out: W) -> Result<(), Self::Error>;
}

pub trait Deserialize: Sized {
    const RULE: Rule;
    type Error: Error;

    fn deserialize(input: Pair<Rule>) -> Result<Self, Self::Error>;
}
