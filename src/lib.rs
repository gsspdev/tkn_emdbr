//pub mod text_processing;

use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TextError {
    EmptyInput,
    InvalidFormat
}

impl fmt::Display for TextError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TextError::EmptyInput => write!(f, "Input string cannot be empty"),
            TextError::InvalidFormat => write!(f, "input has invalid format"),
        }
    }
}

impl Error for TextError {}

pub fn validate_input(text: &str) -> Result<&str, TextError> {
    if text.is_empty() { return Err(TextError::EmptyInput); }
    Ok(text)
}

pub fn make_lowercase(text: &str) -> String {
    text.to_lowercase().to_string()
}

pub fn remove_commas(text: &str) -> String {
    text.to_string().replace(",", "")
}

pub fn remove_double_spaces(text: &str) -> String {
    text.to_string().replace("  ", " ") // TODO: implement code for if there are three or more spaces;
}

pub fn double_punc_paragraph(text: &str) -> String {
    text.to_string().replace("\n", ".")
}

pub fn c_to_ch(text: &str) -> String {
    text.to_string().replace("ch", "c")
}

//use text_processing::*;

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_validate_input() {
        let input = "";
        assert_eq!(Err(TextError::EmptyInput), validate_input(input));
    }

    #[test]
    fn test_make_lowercase() {
        let input = "mAke tHIS ALL lowercase";
        let expected = "make this all lowercase";
        assert_eq!(make_lowercase(input), expected);
    }

    #[test]
    fn test_remove_commas() {
        let input = "so ,,man,y c,omm,as";
        let expected = "so many commas";
        assert_eq!(remove_commas(input), expected);
    }

    #[test]
    fn test_remove_double_spaces() {
        let input = "some  extra  spaces  here";
        let expected = "some extra spaces here";
        assert_eq!(remove_double_spaces(input), expected);
    }

    #[test]
    fn test_double_punc_paragraph() {
        let input = "This is a test.\n There should be a double period directly preceeding this sentence";
        let expected = "This is a test.. There should be a double period directly preceeding this sentence";
        assert_eq!(double_punc_paragraph(input), expected);
    }

    #[test]
    fn test_c_to_ch() {
        let input = "each chew";
        let expected = "eac cew";
        assert_eq!(c_to_ch(input), expected);
    }
}

