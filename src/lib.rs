#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Brace(Vec<Token>),
    Bracket(Vec<Token>),
    Paren(Vec<Token>),
    Punc(String),
    Literal(String),
    Float(String),
    Int(String),
    HexString(String),
    MAC(String),
    IPv4(String),
    Space
}
peg! tokenizer(r##"
use super::Token;

#[pub]
message -> Vec<Token>
    = token+

token -> Token
    = token:composite_token { token }
    / token:simple_token { token }

composite_token -> Token
    = token:brace_token { token }
  / token:bracket_token { token }
  / token:paren_token { token }

simple_token -> Token
    = hex_token
  / ipv4_token
  / mac_token
  / float_token
  / int_token
  / punctuation_token
  / space_token
  / literal_token

space_token -> Token
    = " " { Token::Space }

literal_token -> Token
    = (![{()}] !"[" !"]" !punctuation_token .)+ { Token::Literal(match_str.to_string()) }

brace_token -> Token
    = "{" tokens:token+ "}" { Token::Brace(tokens) }

bracket_token -> Token
    = "[" tokens:token+ "]" { Token::Bracket(tokens) }

paren_token -> Token
    = "(" tokens:token+ ")" { Token::Paren(tokens) }

punctuation_token -> Token
    = punctuations { Token::Punc(match_str.to_string()) }

punctuations
    = ";"
    / ":"
    / ","

float_token -> Token
    = [-+]? [0-9]* "."? [0-9]+ ([eE][-+]?[0-9]+)? { Token::Float(match_str.to_string()) }

hex_token -> Token
    = hex_prefix hex_char+ { Token::HexString(match_str.to_string()) }

hex_prefix
    = "0" [xX]

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
  = mac_general_token { Token::MAC(match_str.to_string()) }
  / mac_cisco_token { Token::MAC(match_str.to_string()) }

mac_general_token -> &'input str
  = &(mac_general) t:mac_general{ t }

mac_general -> &'input str
    = hex_char2 ":" hex_char2 ":" hex_char2 ":" hex_char2 ":" hex_char2 ":" hex_char2 { match_str }

mac_cisco_token -> &'input str
  = &mac_cisco t:mac_cisco { t }

mac_cisco -> &'input str
    = hex_char4 "." hex_char4 "." hex_char4 { match_str }

hex_char4 -> &'input str
  = &(hex_char2 hex_char2) hex_char2 hex_char2 { match_str }

hex_char2 -> &'input str
  = &(hex_char hex_char) hex_char hex_char { match_str }

hex_char -> &'input str
  = &[0-9a-fA-F] [0-9a-fA-F] { match_str }

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

    fn assert_hex_string_token_is_valid(message: &str) {
      let expected =  vec![Token::HexString(message.to_string())];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid HexString token");
      assert_eq!(&expected, &token);
    }

    fn assert_float_token_is_valid(message: &str) {
      let expected =  vec![Token::Float(message.to_string())];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid Float token");
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
        Token::Space,
        Token::MAC("56:84:7a:fe:97:99".to_string()),
        Token::Space,
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

    #[test]
    fn test_given_tokenizer_when_it_parses_a_hex_string_with_0x_prefix_then_we_get_the_hex_string_token() {
        assert_hex_string_token_is_valid("0xff034");
    }

    #[test]
    #[allow(non_snake_case)]
    fn test_given_tokenizer_when_it_parses_a_hex_string_with_0X_prefix_then_we_het_the_hex_string_token() {
        assert_hex_string_token_is_valid("0Xff034");
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_a_float_token_then_we_get_the_float_token() {
        assert_float_token_is_valid("3.14");
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_a_float_token_with_exponent_then_we_get_the_float_token() {
        assert_float_token_is_valid("3.14e0");
    }

    #[test]
    fn test_given_tokenizer_when_there_is_no_other_higher_precedence_match_it_creates_literal_tokens() {
        let message = "foo";
        let expected =  vec![Token::Literal("foo".to_string())];
        let result = tokenizer::message(message);
        println!("{:?}", &result);
        let token = result.ok().expect("Failed to parse a valid literal token");
        assert_eq!(&expected, &token);
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_tokens_in_braces_then_we_get_the_expected_composite_token() {
      let message = "{42 0x12}";
      let expected = vec![
        Token::Brace(vec![
            Token::Int("42".to_string()),
            Token::Space,
            Token::HexString("0x12".to_string()),
        ])
      ];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid message when it contains braces");
      assert_eq!(&expected, &token);
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_tokens_in_brackets_then_we_get_the_expected_composite_token() {
      let message = "[42 0x12]";
      let expected = vec![
        Token::Bracket(vec![
            Token::Int("42".to_string()),
            Token::Space,
            Token::HexString("0x12".to_string()),
        ])
      ];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid message when it contains brackets");
      assert_eq!(&expected, &token);
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_tokens_in_parentheses_then_we_get_the_expected_composite_token() {
      let message = "(42 0x12)";
      let expected = vec![
        Token::Paren(vec![
            Token::Int("42".to_string()),
            Token::Space,
            Token::HexString("0x12".to_string()),
        ])
      ];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid message when it contains parentheses");
      assert_eq!(&expected, &token);
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_tokens_separated_by_punctuation_marks_then_we_get_the_expected_composite_token() {
      let message = "42,0x12:foo;bar";
      let expected = vec![
        Token::Int("42".to_string()),
        Token::Punc(",".to_string()),
        Token::HexString("0x12".to_string()),
        Token::Punc(":".to_string()),
        Token::Literal("foo".to_string()),
        Token::Punc(";".to_string()),
        Token::Literal("bar".to_string()),
      ];
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect("Failed to parse a valid message when the tokens are separated with punctuation marks");
      assert_eq!(&expected, &token);
    }
}
