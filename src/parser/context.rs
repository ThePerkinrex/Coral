#[derive(Debug)]
pub enum ContextName {
    Item,
    Fn,
    FnName(String),
    Arguments,
    Block,
    Statement,
    Expression,
    Identifier,
    Type,
}

impl std::fmt::Display for ContextName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl crate::error::ContextName for ContextName {}
