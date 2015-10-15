use Token;
use tokenizer;

#[cfg(test)]
mod atomic;

#[cfg(test)]
mod complex;

fn parse_and_assert_eq(message: &str, expected: Vec<Token>, error_message: &str) {
    let result = tokenizer::message(message);
    println!("{:?}", &result);
    let token = result.ok().expect(error_message);
    assert_eq!(&expected, &token);
}
