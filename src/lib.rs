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
}
peg! tokenizer(r##"
use super::Token;

#[pub]
message -> Vec<Token> = token ** space

token -> Token
  = ipv4_token
  / mac_token
  / int_token

ipv4_token -> Token
    = octet "." octet "." octet "." octet { Token::IPv4(match_str.to_string()) }

octet
    = "25" [0-5]
    / "2" [0-4][0-9]
    / "1" [0-9][0-9]
    / [1-9][0-9]
    / [0-9]

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

    fn assert_mac_token_is_valid(message: &str) {
      let expected =  vec![Token::MAC(message.to_string())];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid MAC address");
      assert_eq!(&expected, &token);
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_a_mac_address_then_we_got_the_mac_token() {
        assert_mac_token_is_valid("56:84:7a:fe:97:99");
    }

    #[test]
    fn test_given_tokenizer_when_it_parser_a_cisco_mac_address_then_we_get_the_mac_token() {
        assert_mac_token_is_valid("0011.434A.B862");
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
      let message = "42 56:84:7a:fe:97:99 192.168.0.1";
      let expected = vec![
        Token::Int("42".to_string()),
        Token::MAC("56:84:7a:fe:97:99".to_string()),
        Token::IPv4("192.168.0.1".to_string()),
      ];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid message when it contains spaces");
      assert_eq!(&expected, &token);
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_an_ipv4_address_then_we_get_an_ipv4_token() {
      let message = "127.0.0.1";
      let expected =  vec![Token::IPv4("127.0.0.1".to_string())];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid IPv4 token");
      assert_eq!(&expected, &token);
    }
}
