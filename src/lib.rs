#![feature(plugin)]
#![plugin(peg_syntax_ext)]

#[macro_use]
mod macros;
#[cfg(test)]
mod tests;

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
