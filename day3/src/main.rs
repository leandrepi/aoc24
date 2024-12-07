use std::{collections::HashMap, fs};

#[derive(PartialEq, Clone, Debug)]
pub enum Keyword {
    Mult,
    Do,
    Dont,
}

#[derive(Debug)]
pub enum Token {
    Value(u32),
    Keyword(Keyword),
    Punct(char),
    Invalid,
}

pub struct Lexer<'a> {
    mul_str: &'a str,
    cursor: usize,
    do_enabled: bool,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut keyword_map = HashMap::new();
        keyword_map.insert("mul", Keyword::Mult);
        keyword_map.insert("do", Keyword::Do);
        keyword_map.insert("don't", Keyword::Dont);
        while self.cursor < self.mul_str.len() {
            for kw in ["don't", "do", "mul"] {
                let cursor_end = self.cursor + kw.len();
                if cursor_end <= self.mul_str.len() && self.mul_str[self.cursor..cursor_end] == *kw
                {
                    let kw_token = keyword_map.get(kw).expect("Should be in the map");
                    match kw_token {
                        Keyword::Do => {
                            self.do_enabled = true;
                        }
                        Keyword::Dont => {
                            self.do_enabled = false;
                        }
                        _ => (),
                    }
                    self.cursor = cursor_end;
                    return Some(Token::Keyword(kw_token.clone()));
                }
            }
            let mut chars = self.mul_str[self.cursor..].chars();
            let cur = chars.next().expect("Out of bound check already performed");
            self.cursor += 1;
            match cur {
                p @ (',' | '(' | ')') => return Some(Token::Punct(p)),
                c if c.is_digit(10) => {
                    let mut acc = cur.to_digit(10).expect("Should be a digit");
                    while let Some(c) = chars.next() {
                        if c.is_digit(10) {
                            acc = acc * 10 + c.to_digit(10).expect("Should be a digit");
                            self.cursor += 1;
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Value(acc));
                }
                _ => {
                    return Some(Token::Invalid);
                }
            }
        }
        None
    }
}

impl Lexer<'_> {
    fn expect_token(&mut self, token: Token) -> Option<Token> {
        match (self.next(), token) {
            (Some(Token::Value(a)), Token::Value(_)) => Some(Token::Value(a)),
            (Some(Token::Punct(a)), Token::Punct(b)) if a == b => Some(Token::Punct(a)),
            (Some(Token::Keyword(a)), Token::Keyword(b)) if a == b => Some(Token::Keyword(a)),
            _ => None,
        }
    }
}

fn parse_mul(lexer: &mut Lexer) -> Option<u32> {
    let mut acc;
    if let None = lexer.expect_token(Token::Punct('(')) {
        return None;
    }
    match lexer.expect_token(Token::Value(0)) {
        Some(Token::Value(a)) => {
            acc = a;
        }
        _ => return None,
    }

    if let None = lexer.expect_token(Token::Punct(',')) {
        return None;
    }

    match lexer.expect_token(Token::Value(0)) {
        Some(Token::Value(a)) => {
            acc *= a;
        }
        _ => {
            return None;
        }
    }

    if let None = lexer.expect_token(Token::Punct(')')) {
        return None;
    }
    return Some(acc);
}

fn part1(mul_str: &str) -> u32 {
    let mut lexer = Lexer {
        mul_str,
        cursor: 0,
        do_enabled: true,
    };
    let mut res = 0;
    while let Some(token) = lexer.next() {
        match token {
            Token::Keyword(Keyword::Mult) => (),
            _ => continue,
        }
        if let Some(mul_value) = parse_mul(&mut lexer) {
            res += mul_value;
        }
    }
    res
}

fn part2(mul_str: &str) -> u32 {
    let mut lexer = Lexer {
        mul_str,
        cursor: 0,
        do_enabled: true,
    };
    let mut res = 0;
    while let Some(token) = lexer.next() {
        match token {
            Token::Keyword(Keyword::Mult) => {
                if !lexer.do_enabled {
                    continue;
                }
            }
            _ => continue,
        }
        if let Some(mul_value) = parse_mul(&mut lexer) {
            res += mul_value;
        }
    }
    res
}

fn main() {
    let mul_str = fs::read_to_string("input.txt")
        .map_err(|e| eprintln!("ERROR: Failed to read file: {e}"))
        .unwrap();
    let result_part1 = part1(&mul_str);
    println!("Day 3, part 1: {result_part1}");
    let result_part2 = part2(&mul_str);
    println!("Day 3, part 2: {result_part2}");
}
