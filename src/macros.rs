macro_rules! brace {
    ($value:expr) => {
        Token::Brace($value)
    }
}

macro_rules! bracket {
    ($value:expr) => {
        Token::Bracket($value)
    }
}

macro_rules! paren {
    ($value:expr) => {
        Token::Paren($value)
    }
}

macro_rules! kvpair {
    ($key:expr, $value:expr) => {
        Token::KVPair($key, $value)
    }
}

macro_rules! audit {
    ($timestamp:expr, $id:expr) => {
        Token::Audit($timestamp.to_string(), $id.to_string())
    }
}

macro_rules! literal {
    ($literal:expr) => {
        Token::Literal($literal.to_string())
    }
}

macro_rules! qliteral {
    ($literal:expr) => {
        Token::QuotedLiteral($literal.to_string())
    }
}

macro_rules! float {
    ($value:expr) => {
        Token::Float($value.to_string())
    }
}

macro_rules! int {
    ($value:expr) => {
        Token::Int($value.to_string())
    }
}

macro_rules! hexstring {
    ($value:expr) => {
        Token::HexString($value.to_string())
    }
}

macro_rules! mac {
    ($value:expr) => {
        Token::MAC($value.to_string())
    }
}

macro_rules! ipv4 {
    ($value:expr) => {
        Token::IPv4($value.to_string())
    }
}

macro_rules! progpid {
    ($program:expr, $pid:expr) => {
        Token::ProgramPid($program.to_string(), $pid.to_string())
    }
}
