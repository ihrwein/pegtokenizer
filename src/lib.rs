#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#[macro_use]
mod macros;

#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Brace(Vec<Token>),
    Bracket(Vec<Token>),
    Paren(Vec<Token>),
    KVPair(Box<Token>, Box<Token>),
    Audit(String, String),
    ProgramPid(String, String),
    Literal(String),
    QuotedLiteral(String),
    Float(String),
    Int(String),
    HexString(String),
    MAC(String),
    IPv4(String),
}

use tokenizer::ParseError;

pub fn tokenize(message: &str) -> Result<Vec<Token>, ParseError> {
    tokenizer::message(message)
}

peg! tokenizer(r##"
use super::Token;

#[pub]
message -> Vec<Token>
    = token_seq

token_seq -> Vec<Token>
    = token_expr+

separators
    = separator+

separator -> &'input str
    = s:space { s }
    / p:punctuation { p }

token_expr -> Token
    = separators? token:token separators? { token }

token -> Token
    = token:composite_token { token }
    / token:simple_token  { token }

composite_token -> Token
    = token:brace_token { token }
  / token:bracket_token { token }
  / token:paren_token { token }
  / token:kvpair_token { token }
  / token:program_pid_token { token }

program_pid_token -> Token
    = program:key "[" pid:int "]" { Token::ProgramPid(program.to_string(), pid.to_string()) }

kvpair_token -> Token
    = key:kvpair_token_key "=" value:kvpair_token_value {
        Token::KVPair(Box::new(key), Box::new(value))
     }

kvpair_token_key -> Token
    = key { Token::Literal(match_str.to_string()) }

key -> &'input str
    = [a-zA-Z0-9_-]+ { match_str }

kvpair_token_value -> Token
    = t:audit_token { t }
    / l:quoted_literal_token { l }
    / (!"}" !"]" !")" !" " .)+ { Token::Literal(match_str.to_string()) }
    / (!" " .)+ { Token::Literal(match_str.to_string()) }

audit_token -> Token
    = "audit" "(" timestamp:float ":" id:int ")" { Token::Audit(timestamp.to_string(), id.to_string()) }

simple_token -> Token
    = hex_token
  / ipv4_token
  / mac_token
  / float_token
  / int_token
  / quoted_literal_token
  / literal_token

quoted_literal_token -> Token
    = "\"" (!"\"" .)+ "\"" { Token::QuotedLiteral(match_str.to_string()) }
    / "'" (!"'" .)+ "'" { Token::QuotedLiteral(match_str.to_string()) }

literal_token -> Token
    = (!"{" !"}" !"(" !")" !separators !"=" !BRACKET_OPEN  !BRACKET_CLOSE  .)+ { Token::Literal(match_str.to_string()) }

BRACKET_OPEN
    = "["

BRACKET_CLOSE
    = ']'

brace_token -> Token
    = "{" tokens:token_seq "}" { Token::Brace(tokens) }

bracket_token -> Token
    = "[" separators ? tokens:token_seq "]" { Token::Bracket(tokens) }

paren_token -> Token
    = "(" tokens:token_seq ")" { Token::Paren(tokens) }

punctuation -> &'input str
    = ";" { match_str }
    / ":" { match_str }
    / "," { match_str }

float_token -> Token
    = f:float { Token::Float(f.to_string()) }

float -> &'input str
    = [-+]? [0-9]* "."? [0-9]+ ([eE][-+]?[0-9]+)? { match_str }

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
    = i:int { Token::Int(i.to_string()) }

int -> &'input str
  = [0-9]+ { match_str }

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
    = " " { match_str }

"##);

#[cfg(test)]
mod tests {
    use tokenizer;
    use Token;

    fn parse_and_assert_eq(message: &str, expected: Vec<Token>, error_message: &str) {
      let result = tokenizer::message(message);
      println!("{:?}", &result);
      let token = result.ok().expect(error_message);
      assert_eq!(&expected, &token);
    }

    fn assert_mac_token_is_valid(message: &str) {
      let expected =  vec![mac!(message)];
      parse_and_assert_eq(message, expected, "Failed to parse a valid MAC address");
    }

    fn assert_hex_string_token_is_valid(message: &str) {
      let expected =  vec![hexstring!(message)];
      parse_and_assert_eq(message, expected, "Failed to parse a valid HexString address");
    }

    fn assert_float_token_is_valid(message: &str) {
      let expected =  vec![float!(message)];
      parse_and_assert_eq(message, expected, "Failed to parse a valid Float address");
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
      let expected =  vec![int!(message)];
      parse_and_assert_eq(message, expected, "Failed to parse a valid Int address");
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_tokens_separated_with_space_characters_then_we_got_the_tokens() {
      let message = "42 56:84:7a:fe:97:99 192.168.0.1";
      let expected = vec![
        int!("42"),
        mac!("56:84:7a:fe:97:99"),
        ipv4!("192.168.0.1"),
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a valid message when it contains spaces");
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_an_ipv4_address_then_we_get_an_ipv4_token() {
      let message = "127.0.0.1";
      let expected =  vec![ipv4!("127.0.0.1")];
      parse_and_assert_eq(message, expected, "Failed to parse a valid IPv4 token");
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
        let expected =  vec![literal!("foo")];
        parse_and_assert_eq(message, expected, "Failed to parse a valid literal token");
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_tokens_in_braces_then_we_get_the_expected_composite_token() {
      let message = "{42 0x12}";
      let expected = vec![
        brace!(vec![
            int!("42"),
            hexstring!("0x12"),
        ])
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a valid message when it contains braces");
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_tokens_in_brackets_then_we_get_the_expected_composite_token() {
      let message = "[42 0x12]";
      let expected = vec![
        bracket!(vec![
            int!("42"),
            hexstring!("0x12"),
        ])
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a valid message when it contains brackets");
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_tokens_in_parentheses_then_we_get_the_expected_composite_token() {
      let message = "(42 0x12)";
      let expected = vec![
        paren!(vec![
            int!("42"),
            hexstring!("0x12"),
        ])
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a valid message when it contains parentheses");
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_tokens_separated_by_punctuation_marks_then_we_get_the_expected_composite_token() {
      let message = "42,0x12:foo bar";
      let expected = vec![
        int!("42"),
        hexstring!("0x12"),
        literal!("foo"),
        literal!("bar"),
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a valid message when the tokens are separated with punctuation marks");
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_tokens_in_parens_then_we_get_the_expected_composite_token() {
      let message = "(xid=0x37fe20e3)";
      let expected = vec![
        paren!(
            vec![
                kvpair!(
                    Box::new(literal!("xid")),
                    Box::new(literal!("0x37fe20e3"))
                )
            ]
        )
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a valid message when the tokens are in parens");
    }

    #[test]
    fn test_given_tokenizer_when_it_parses_a_log_message_then_we_get_the_expected_tokens() {
      let message = "dhclient: DHCPREQUEST of 10.30.0.97 on eth0 to 255.255.255.255 port 67 (xid=0x37fe20e3)";
      let expected = vec![
        literal!("dhclient"),
        literal!("DHCPREQUEST"),
        literal!("of"),
        ipv4!("10.30.0.97"),
        literal!("on"),
        literal!("eth0"),
        literal!("to"),
        ipv4!("255.255.255.255"),
        literal!("port"),
        int!("67"),
        paren!(vec![
            kvpair!(
                Box::new(literal!("xid")),
                Box::new(literal!("0x37fe20e3"))
            )]
        ),
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a valid log message");
  }

  #[test]
  fn test_given_tokenizer_when_it_parses_key_value_pairs_in_sequence_then_we_get_the_expected_tokens() {
      let message = "foo=bar qux=42 42=42";
      let expected = vec![
        kvpair!(
            Box::new(literal!("foo")),
            Box::new(literal!("bar"))
        ),
        kvpair!(
            Box::new(literal!("qux")),
            Box::new(literal!("42"))
        ),
        kvpair!(
            Box::new(literal!("42")),
            Box::new(literal!("42"))
        )
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a valid key-value pairs");
  }

  #[test]
  fn test_given_tokenizer_when_it_parses_a_key_value_pair_and_the_value_is_not_a_simple_token_then_we_get_the_expected_tokens() {
      let message = "msg=audit(1364481363.243:24287)";
      let expected = vec![
        kvpair!(
            Box::new(literal!("msg")),
            Box::new(audit!("1364481363.243", "24287"))
        ),
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a valid key-value pair when the value is a composite token");
  }

  #[test]
  fn test_given_tokenizer_when_it_parses_a_quoted_string_then_we_get_the_expected_token() {
      let message = r#"exe="/bin/cat""#;
      let expected = vec![
        kvpair!(
            Box::new(literal!("exe")),
            Box::new(qliteral!(r#""/bin/cat""#))
        ),
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a valid message when it contains \" quoted string");
  }

  #[test]
  fn test_given_tokenizer_when_it_parses_a_quoted_string_with_apostrophe_then_we_get_the_expected_token() {
      let message = r#"exe='/bin/cat'"#;
      let expected = vec![
        kvpair!(
            Box::new(literal!("exe")),
            Box::new(qliteral!(r#"'/bin/cat'"#))
        ),
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a valid message when it contains \" quoted string");
  }

  #[test]
  fn test_given_tokenizer_when_it_parses_a_key_value_pair_and_the_value_consists_of_more_than_one_tokens_then_we_get_the_expected_tokens() {
    let message = "dev=fd:00";
    let expected = vec![
        kvpair!(
            Box::new(literal!("dev")),
            Box::new(literal!("fd:00"))
        )
    ];
    parse_and_assert_eq(message, expected, "Failed to parse key-value pair when the value ");
  }

  #[test]
  fn test_given_tokenizer_when_it_parses_auditd_log_then_we_get_the_expected_token() {
      let message = r#"type=PATH msg=audit(1364481363.243:24287): item=0 name="/etc/ssh/sshd_config" inode=409248 dev=fd:00 mode=0100600 ouid=0 ogid=0 rdev=00:00 obj=system_u:object_r:etc_t:s0"#;
      let expected = vec![
        kvpair!(
            Box::new(literal!("type")),
            Box::new(literal!("PATH"))
        ),
        kvpair!(
            Box::new(literal!("msg")),
            Box::new(audit!("1364481363.243", "24287"))
        ),
        kvpair!(
            Box::new(literal!("item")),
            Box::new(literal!("0"))
        ),
        kvpair!(
            Box::new(literal!("name")),
            Box::new(qliteral!(r#""/etc/ssh/sshd_config""#))
        ),
        kvpair!(
            Box::new(literal!("inode")),
            Box::new(literal!("409248"))
        ),
        kvpair!(
            Box::new(literal!("dev")),
            Box::new(literal!("fd:00"))
        ),
        kvpair!(
            Box::new(literal!("mode")),
            Box::new(literal!("0100600"))
        ),
        kvpair!(
            Box::new(literal!("ouid")),
            Box::new(literal!("0"))
        ),
        kvpair!(
            Box::new(literal!("ogid")),
            Box::new(literal!("0"))
        ),
        kvpair!(
            Box::new(literal!("rdev")),
            Box::new(literal!("00:00"))
        ),
        kvpair!(
            Box::new(literal!("obj")),
            Box::new(literal!("system_u:object_r:etc_t:s0"))
        ),
      ];
      parse_and_assert_eq(message, expected, "Failed to parse auditd log");
  }

  #[test]
  fn test_given_tokenizer_when_it_parses_a_message_which_contains_programname_and_pid_then_we_get_the_expected_tokens() {
      let message = "localhost bluetoothd[723]: Starting SDP server";
      let expected = vec![
        literal!("localhost"),
        progpid!("bluetoothd", "723"),
        literal!("Starting"),
        literal!("SDP"),
        literal!("server"),
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a log message containing program[PID]");
  }

  #[test]
  fn test_given_tokenizer_when_it_parses_adjacent_separators_then_we_get_the_expected_result() {
      let message = " a";
      let expected = vec![
            literal!("a")
      ];
      parse_and_assert_eq(message, expected, "Failed to parse adjacent separators");
  }

  #[test]
  fn test_given_tokenizer_when_it_parses_a_kernel_log_message_then_we_get_the_expected_tokens() {
      let message = "kernel: [    0.000000] Initializing";
      let expected = vec![
        literal!("kernel"),
        bracket!(vec![
            float!("0.000000")
        ]),
        literal!("Initializing"),
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a kernel log");
  }

  #[test]
  fn test_given_tokenizer_when_it_parses_separators_in_brackets_then_we_get_the_expected_tokens() {
      let message = "[ w0: C-E ]";
      let expected = vec![
        bracket!(vec![
            literal!("w0"),
            literal!("C-E"),
        ]),
      ];
      parse_and_assert_eq(message, expected, "Failed to parse separators in brackets");
  }

  #[test]
  fn test_given_tokenizer_when_it_parses_a_message_where_the_last_characters_are_separators_then_we_get_the_expected_tokens() {
      let message = "a  ";
      let expected = vec![
        literal!("a"),
      ];
      parse_and_assert_eq(message, expected, "Failed to parse a message where the last tokens are separators");
  }
}
