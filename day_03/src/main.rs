#[derive(Debug, PartialEq)]
enum Token<'a> {
    Mul,
    LParen,
    RParen,
    Comma,
    Digit(&'a str),
    Unk,
    Do,
    Dont,
}
struct Lexer<'a> {
    content: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(content: &'a str) -> Self {
        Self { content }
    }

    fn peek(&self) -> Option<char> {
        let mut chars = self.content.chars();
        let char = chars.next()?;
        Some(char)
    }

    fn take(&mut self) -> Option<char> {
        let mut chars = self.content.chars();
        let char = chars.next()?;
        self.content = chars.as_str();
        Some(char)
    }

    fn take_while(&mut self, predicate: &dyn Fn(&char) -> bool) -> usize {
        let mut count = 0;
        while let Some(ch) = self.peek() {
            match ch {
                ch if predicate(&ch) => count += self.take().expect("Peek checked").len_utf8(),
                _ => break,
            }
        }
        count
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let original_input = self.content;

        let token = match original_input {
            s if s.starts_with("mul") => {
                self.take()?;
                self.take()?;
                self.take()?;
                Some(Token::Mul)
            }
            s if s.starts_with("don't") => {
                self.take()?;
                self.take()?;
                self.take()?;
                self.take()?;
                self.take()?;
                Some(Token::Dont)
            }
            s if s.starts_with("do") => {
                self.take()?;
                self.take()?;

                Some(Token::Do)
            }
            _ => None,
        };

        if token.is_some() {
            return token;
        }

        let ch = self.take()?;

        let token = match ch {
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '0'..='9' => Token::Digit(
                &original_input[..ch.len_utf8() + self.take_while(&|x| x.is_numeric())],
            ),
            _ => Token::Unk,
        };

        Some(token)
    }
}

impl<'a> From<Lexer<'a>> for Vec<Token<'a>> {
    fn from(value: Lexer<'a>) -> Self {
        value.into_iter().collect()
    }
}

#[derive(Debug)]
enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
    Noop,
}

struct Parser<'a> {
    lexer: std::iter::Peekable<Lexer<'a>>,
    current_token: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer: lexer.peekable(),
            current_token: None,
        }
    }

    fn next(&mut self) {
        let token = self.lexer.next();
        self.current_token = token;
    }

    fn parse(&mut self) -> Vec<Instruction> {
        self.next();
        let mut instructions = Vec::new();

        while self.current_token.is_some() {
            let instruction = match self.current_token {
                Some(Token::Mul) => {
                    self.next();
                    self.parse_mul()
                }
                Some(Token::Do) => {
                    self.next();
                    self.parse_do()
                }
                Some(Token::Dont) => {
                    self.next();
                    self.parse_dont()
                }
                _ => {
                    self.next();
                    Instruction::Noop
                }
            };

            instructions.push(instruction);
        }

        instructions
    }

    fn parse_mul(&mut self) -> Instruction {
        if !matches!(self.current_token, Some(Token::LParen)) {
            return Instruction::Noop;
        }
        self.next();

        let Some(Token::Digit(a)) = self.current_token else {
            return Instruction::Noop;
        };
        self.next();

        if !matches!(self.current_token, Some(Token::Comma)) {
            return Instruction::Noop;
        }
        self.next();

        let Some(Token::Digit(b)) = self.current_token else {
            return Instruction::Noop;
        };
        self.next();

        if !matches!(self.current_token, Some(Token::RParen)) {
            return Instruction::Noop;
        }
        self.next();

        Instruction::Mul(a.parse().unwrap(), b.parse().unwrap())
    }

    fn parse_do(&mut self) -> Instruction {
        if !matches!(self.current_token, Some(Token::LParen)) {
            return Instruction::Noop;
        }
        self.next();

        if !matches!(self.current_token, Some(Token::RParen)) {
            return Instruction::Noop;
        }
        self.next();

        Instruction::Do
    }

    fn parse_dont(&mut self) -> Instruction {
        if !matches!(self.current_token, Some(Token::LParen)) {
            return Instruction::Noop;
        }
        self.next();

        if !matches!(self.current_token, Some(Token::RParen)) {
            return Instruction::Noop;
        }
        self.next();

        Instruction::Dont
    }
}

fn main() {
    let sample = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    //let sample = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    //let sample = include_str!("./input.txt");
    let l = Lexer::new(sample);

    let mut p = Parser::new(l);
    let x = p.parse();

    let mut is_disabled = false;
    let mut sum = 0;
    for instruction in x {
        match instruction {
            Instruction::Mul(a, b) => {
                if !is_disabled {
                    sum += a * b;
                }
            }
            Instruction::Do => is_disabled = false,
            Instruction::Dont => is_disabled = true,
            Instruction::Noop => {}
        }
    }

    dbg!(sum);
    //let total: i32 = x
    //    .iter()
    //    .map(|x| match x {
    //        Instruction::Mul(a, b) => a * b,
    //        _ => 0,
    //    })
    //    .sum();
    //dbg!(total);
}
