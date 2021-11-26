fn is_letter(character: char) -> bool {
    character.is_ascii_alphabetic()
}
fn is_whitespace(character: char) -> bool {
    character.is_ascii_whitespace()
}
fn is_number(character: char) -> bool {
    character.is_numeric()
}
fn is_open_paren(character: char) -> bool {
    character == '('
}
fn is_close_paren(character: char) -> bool {
    character == ')'
}
fn is_paren(character: char) -> bool {
    is_open_paren(character) || is_close_paren(character)
}
fn is_quote(character: char) -> bool {
    character == '"'
}


#[derive(Debug, Eq, PartialEq)]
enum Token {
    Paren {
        value: char,
    },
    Number {
        value: i32,
    },
    Word {
        value: String,
    },
    String {
        value: String,
    },
}


fn lex(source: String) -> Vec<Token> {
    let source_chars: Vec<char> = source.chars().collect();

    let mut tokens = vec![];
    let mut cursor = 0;

    while cursor < source_chars.len() {
        let character = source_chars[cursor];

        if is_paren(character) {
            tokens.push(Token::Paren { value: character });
            cursor += 1;

            continue;
        }
        if is_whitespace(character) {
            cursor += 1;
            
            continue;
        }
        if is_number(character) {
            let start = cursor;

            cursor += 1;
            while cursor < source_chars.len() && is_number(source_chars[cursor]) {
                cursor += 1;
            }

            let number_string = source_chars[start..cursor].iter().cloned().collect::<String>();
            if let Ok(number) = number_string.parse::<i32>() {
                tokens.push(
                    Token::Number {
                        value: number,
                    }
                );
            } else {
                panic!("Failed to parse number: {}", number_string);
            }
            continue;
        }
        if is_letter(character) {
            let start = cursor;

            cursor += 1;
            while cursor < source_chars.len() && is_letter(source_chars[cursor]) {
                cursor += 1;
            }

            let word_string = source_chars[start..cursor].iter().cloned().collect::<String>();
            tokens.push(Token::Word { value: word_string });
            
            continue;
        }
        if is_quote(character) {
            // Drop the opening "
            cursor += 1;

            let start = cursor;

            while cursor < source_chars.len() && !is_quote(source_chars[cursor]) {
                cursor += 1;
            }

            let string_string = source_chars[start..cursor].iter().cloned().collect::<String>();
            tokens.push(Token::String { value: string_string });

            // Drop the closing "
            cursor += 1;

            continue;
        }

        panic!("Invalid character: {}", character);
    }

    tokens
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn lex_parens() {
        let input = String::from("()");
        let result = vec![
            Token::Paren { value: '(' },
            Token::Paren { value: ')' },
        ];

        assert_eq!(lex(input), result);
    }

    #[test]
    fn ignores_whitespace() {
        let input = String::from("                      ");
        let result = vec![];

        assert_eq!(lex(input), result);
    }

    #[test]
    fn lex_single_digit() {
        let input = String::from("2");
        let result = vec![
            Token::Number { value: 2_i32 },
        ];

        assert_eq!(lex(input), result);
    }

    #[test]
    fn lex_multi_digit() {
        let input = String::from("234");
        let result = vec![
            Token::Number { value: 234_i32 },
        ];

        assert_eq!(lex(input), result);
    }

    #[test]
    fn lex_single_digit_expression() {
        let input = String::from("(1 2)");
        let result = vec![
            Token::Paren { value: '(' },
            Token::Number{ value: 1_i32 },
            Token::Number{ value: 2_i32 },
            Token::Paren { value: ')' },
        ];

        assert_eq!(lex(input), result);
    }

    #[test]
    fn lex_single_letter() {
        let input = String::from("a");
        let result = vec![
            Token::Word { value: String::from("a") },
        ];

        assert_eq!(lex(input), result);
    }

    #[test]
    fn lex_multi_letter() {
        let input = String::from("abc");
        let result = vec![
            Token::Word { value: String::from("abc") },
        ];

        assert_eq!(lex(input), result);
    }

    #[test]
    fn lex_single_letter_expression() {
        let input = String::from("(a b)");
        let result = vec![
            Token::Paren { value: '(' },
            Token::Word{ value: String::from("a") },
            Token::Word{ value: String::from("b") },
            Token::Paren { value: ')' },
        ];

        assert_eq!(lex(input), result);
    }

    #[test]
    fn lex_string() {
        let input = String::from("\"Hello, world!\"");
        let result = vec![
            Token::String { value: String::from("Hello, world!") },
        ];

        assert_eq!(lex(input), result);
    }
}
