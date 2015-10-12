#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Other(String),
    Literal(String),
    Float(String),
    Int(String),
    HexString(String),
    MAC(String),
    IPv4(String),
    IPv6(String)
}
peg! tokenizer(r##"
use super::Token;

#[pub]
message -> Vec<Token> = token ** space

token -> Token
  = mac_token
  / int_token

int_token -> Token
  = [0-9]+ { Token::Int(match_str.to_string()) }

mac_token -> Token
  = mac_general { Token::MAC(match_str.to_string()) }
  / mac_cisco { Token::MAC(match_str.to_string()) }

mac_general -> &'input str
  = hex_char2 ":" hex_char2 ":" hex_char2 ":" hex_char2 ":" hex_char2 ":" hex_char2 { match_str }

mac_cisco -> &'input str
  = hex_char4 "." hex_char4 "." hex_char4 { match_str }

hex_char4 -> &'input str
  = hex_char hex_char hex_char hex_char { match_str }

hex_char2 -> &'input str
  = hex_char hex_char { match_str }

hex_char -> &'input str
  = [0-9a-fA-F] { match_str }

space -> &'input str
    = " "+ { match_str }

"##);

#[cfg(test)]
mod tests {
    use tokenizer;
    use Token;

    fn assert_mac_token_eq(message: &str) {
      let expected =  vec![Token::MAC(message.to_string())];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid MAC address");
      assert_eq!(&expected, &token);
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_a_mac_address_then_we_got_the_mac_token() {
        assert_mac_token_eq("56:84:7a:fe:97:99");
    }

    #[test]
    fn test_given_tokenizer_when_it_parser_a_cisco_mac_address_then_we_get_the_mac_token() {
        assert_mac_token_eq("0011.434A.B862");
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_an_integer_then_we_get_the_int_token() {
      let message = "42";
      let expected =  vec![Token::Int("42".to_string())];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid Int token");
      assert_eq!(&expected, &token);
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_tokens_separated_with_space_characters_then_we_got_the_tokens() {
      let message = "42 56:84:7a:fe:97:99";
      let expected = vec![
        Token::Int("42".to_string()),
        Token::MAC("56:84:7a:fe:97:99".to_string()),
      ];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid message when it contains spaces");
      assert_eq!(&expected, &token);
    }
}
