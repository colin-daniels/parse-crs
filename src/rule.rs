use pest::iterators::Pair;
use thiserror::Error;

use crate::{
    action::{Action, ActionParseError},
    input::{Input, InputParseError},
    operator::{Operator, OperatorParseError},
    pest::{Deserialize, Rule},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SecRule {
    pub inputs: Vec<Input>,
    pub op: Operator,
    pub actions: Vec<Action>,
}

#[derive(Error, Debug)]
pub enum SecRuleParseError {
    #[error("invalid rule {0:?}")]
    UnexpectedRule(Rule),
    #[error(transparent)]
    Input(#[from] InputParseError),
    #[error(transparent)]
    Operator(#[from] OperatorParseError),
    #[error(transparent)]
    Action(#[from] ActionParseError),
}

impl Deserialize for SecRule {
    const RULE: Rule = Rule::sec_rule;
    type Error = SecRuleParseError;

    fn deserialize(input: Pair<Rule>) -> Result<Self, Self::Error> {
        if input.as_rule() != Self::RULE {
            return Err(SecRuleParseError::UnexpectedRule(input.as_rule()));
        }

        let mut inner = input.into_inner();
        let inputs = Deserialize::deserialize(inner.next().unwrap())?;
        let op = Deserialize::deserialize(inner.next().unwrap())?;
        let actions = Deserialize::deserialize(inner.next().unwrap())?;

        Ok(Self {
            inputs,
            op,
            actions,
        })
    }
}
