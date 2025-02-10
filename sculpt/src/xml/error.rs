use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
/// Defines a problem that was found while trying to parse Xml into a type
pub enum XmlWriteError {
    /// An error from quick-xml
    #[error("An error occurred while parsing the xml: {0}")]
    ParserError(String),

    /// Implies that the name is invalid and cannot be mapped to any doc node
    #[error("The name {0} is invalid")]
    InvalidName(String),

    /// Recieving this error means that there is an attribute on a valid name that cannot be used
    #[error("The attribute {0} is invalid")]
    InvalidAttribute(String),

    /// Text exists in the xml where it should not
    #[error("The text {0} is invalid")]
    EmptyText(String),

    /// if an element has an invalid child. For example, `<p>` can only contain text.
    #[error("The children of {0} are invalid")]
    InvalidChildren(String),

    /// Something went horrendously wrong and cannot be described by any other error. This should be removed eventually
    #[error("An unknown error occurred: {0}")]
    TotalFailure(String),
}

impl From<quick_xml::Error> for XmlWriteError {
    fn from(value: quick_xml::Error) -> Self {
        XmlWriteError::ParserError(value.to_string())
    }
}

impl From<std::io::Error> for XmlWriteError {
    fn from(value: std::io::Error) -> Self {
        XmlWriteError::ParserError(value.to_string())
    }
}
