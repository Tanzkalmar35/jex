#[derive(Debug, PartialEq)]
pub(crate) enum ParserError {
    ExpectedVariable,
    ExpectedAssignment,
    ExpectedValue,
    InvalidValue,
    UnexpectedToken,
}
