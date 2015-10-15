use Token;
use super::parse_and_assert_eq;

#[test]
fn test_given_tokenizer_when_it_parses_tokens_separated_with_space_characters_then_we_got_the_tokens
    () {
    let message = "42 56:84:7a:fe:97:99 192.168.0.1";
    let expected = vec![
    int!("42"),
    mac!("56:84:7a:fe:97:99"),
    ipv4!("192.168.0.1"),
  ];
    parse_and_assert_eq(message,
                        expected,
                        "Failed to parse a valid message when it contains spaces");
}

#[test]
fn test_given_tokenizer_when_it_parses_tokens_separated_by_punctuation_marks_then_we_get_the_expected_composite_token
    () {
    let message = "42,0x12:foo bar";
    let expected = vec![
    int!("42"),
    hexstring!("0x12"),
    literal!("foo"),
    literal!("bar"),
  ];
    parse_and_assert_eq(message,
                        expected,
                        "Failed to parse a valid message when the tokens are separated with \
                         punctuation marks");
}

#[test]
fn test_given_tokenizer_when_it_parses_a_log_message_then_we_get_the_expected_tokens() {
    let message = "dhclient: DHCPREQUEST of 10.30.0.97 on eth0 to 255.255.255.255 port 67 \
                   (xid=0x37fe20e3)";
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
fn test_given_tokenizer_when_it_parses_key_value_pairs_in_sequence_then_we_get_the_expected_tokens
    () {
    let message = "foo=bar qux=42 42=42";
    let expected = vec![kvpair!(Box::new(literal!("foo")), Box::new(literal!("bar"))),
                        kvpair!(Box::new(literal!("qux")), Box::new(literal!("42"))),
                        kvpair!(Box::new(literal!("42")), Box::new(literal!("42")))];
    parse_and_assert_eq(message, expected, "Failed to parse a valid key-value pairs");
}

#[test]
fn test_given_tokenizer_when_it_parses_a_key_value_pair_and_the_value_is_not_a_simple_token_then_we_get_the_expected_tokens
    () {
    let message = "msg=audit(1364481363.243:24287)";
    let expected = vec![
    kvpair!(
        Box::new(literal!("msg")),
        Box::new(audit!("1364481363.243", "24287"))
    ),
  ];
    parse_and_assert_eq(message,
                        expected,
                        "Failed to parse a valid key-value pair when the value is a composite \
                         token");
}
