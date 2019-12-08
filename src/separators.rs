use super::*;
use std::str::FromStr;

/// A helper struct to store the separator (delimiter) characters used to parse this message.
/// Note that HL7 allows each _message_ to define it's own separators, although most messages
/// use a default set (available from `Separators::default()`)
pub struct Separators {
    /// constant value, spec fixed to '\r' (ASCII 13, 0x0D)
    segment: char,
    field: char,
    repeat: char,
    component: char,
    subcomponent: char,

    escape_char: char,
}

impl Separators {
    /// Create a Separator with the default (most common) HL7 values
    pub fn default() -> Separators {
        Separators {
            segment: '\r',
            field: '|',
            repeat: '~',
            component: '^',
            subcomponent: '&',
            escape_char: '\\',
        }
    }

    // Create a Separators with the values provided in the message.
    // This assumes the message starts with `MSH|^~\&|` or equiv for custom Separators
    fn new(message: &str) -> Result<Separators, Hl7ParseError> {
        //assuming we have a valid message
        let mut chars = message.char_indices();

        if Some((0, 'M')) != chars.next()
            || Some((1, 'S')) != chars.next()
            || Some((2, 'H')) != chars.next()
        {
            return Err(Hl7ParseError::Msh1Msh2 {
                error: "Message doesn't start with 'MSH'".to_string(),
            });
        }

        Ok(Separators {
            segment: '\r',
            field: chars.next().unwrap().1,
            component: chars.next().unwrap().1,
            repeat: chars.next().unwrap().1,
            escape_char: chars.next().unwrap().1,
            subcomponent: chars.next().unwrap().1,
        })
    }
}

/// Expects to receive a full message (or at least a MSH segment) in order to parse
/// out the separator chars.
impl FromStr for Separators {
    type Err = Hl7ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        return Separators::new(input);
    }
}

#[cfg(test)]
mod tests {
    use super::separators::Separators;
    use super::*;

    #[test]
    fn ensure_separators_load_correctly() {
        let expected = Separators::default();
        let actual = Separators::new("MSH|^~\\&|CATH|StJohn|AcmeHIS|StJohn|20061019172719||ACK^O01|MSGID12349876|P|2.3\rMSA|AA|MSGID12349876");
        match actual {
            Ok(actual) => {
                assert_eq!(expected.component, actual.component);
                assert_eq!(expected.escape_char, actual.escape_char);
                assert_eq!(expected.field, actual.field);
                assert_eq!(expected.repeat, actual.repeat);
                assert_eq!(expected.segment, actual.segment);
                assert_eq!(expected.subcomponent, actual.subcomponent);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn ensure_separators_load_from_string() {
        let expected = Separators::default();
        let actual = str::parse::<Separators>("MSH|^~\\&|CATH|StJohn|AcmeHIS|StJohn|20061019172719||ACK^O01|MSGID12349876|P|2.3\rMSA|AA|MSGID12349876");

        match actual {
            Ok(actual) => {
                assert_eq!(expected.component, actual.component);
                assert_eq!(expected.escape_char, actual.escape_char);
                assert_eq!(expected.field, actual.field);
                assert_eq!(expected.repeat, actual.repeat);
                assert_eq!(expected.segment, actual.segment);
                assert_eq!(expected.subcomponent, actual.subcomponent);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn ensure_missing_msh_causes_error() {
        //note the missing M
        if let Err(Hl7ParseError::Msh1Msh2{error: _}) = Separators::new("SH|^~\\&|CATH|StJohn|AcmeHIS|StJohn|20061019172719||ACK^O01|MSGID12349876|P|2.3\rMSA|AA|MSGID12349876")
        {
              //nop, we're good
        } else {
            assert!(false); //should have failed here
        }
    }
}
