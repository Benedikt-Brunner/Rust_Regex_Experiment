use std::str::Chars;

fn main() {
    let binding: String = "a*vb{5}ggg{5,}".to_string();
    println!("{}", regex(binding)("aaaaaaavbbvbgggggggggg".to_string()));
}

#[derive(Debug, PartialEq, Clone)]
enum Token {
    Star(Vec<char>),
    Plus(Vec<char>),
    Question(Vec<char>),
    OR(char, char),
    TimesEx(u32, Vec<char>),
    Timesleast(u32, Vec<char>),
}

struct Parser{
    rules: Vec<Token>,
    cur_rule: usize,
    accept: bool,
    count: u32,
}

impl Parser{
    fn new(rules: Vec<Token>) -> Self{
        println!("{:?}", rules);
        Parser{
            rules,
            cur_rule: 0,
            accept: false,
            count: 0,
        }
    }
    fn parse(&mut self, cur_char: char){
        println!("{} {} {}", cur_char, self.cur_rule, self.count);
        if self.accept{
            return;
        }

        match &self.rules[self.cur_rule]{
            Token::Star(a) => {
                    if a.contains(&cur_char){
                        self.count += 1;
                        return;
                    }
                    if self.cur_rule + 1 == self.rules.len(){
                        self.accept = true;
                        return;
                    }
               match &self.rules[self.cur_rule + 1]{
                        Token::Star(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        }, 
                        Token::Plus(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::Question(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::OR(a, b) => {
                            if a == &cur_char || b == &cur_char{
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::TimesEx(_, b) => {
                                if b.contains(&cur_char){
                                    self.cur_rule += 1;
                                    self.count = 1;
                                    return;
                                }
                        },
                        Token::Timesleast(_, b) => {
                            if b.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 1;
                                return;
                            }
                        }
                    }
            },
            Token::Plus(a) => {
                if self.count < 1{
                    if a.contains(&cur_char){
                        self.count += 1;
                        return;
                    }else{
                        self.cur_rule = 0;
                        self.count = 0;
                        return;
                    }
                }
                if self.cur_rule + 1 == self.rules.len(){
                    self.accept = true;
                    return;
                }
               match &self.rules[self.cur_rule + 1]{
                        Token::Star(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        }, 
                        Token::Plus(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::Question(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::OR(a, b) => {
                            if a == &cur_char || b == &cur_char{
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::TimesEx(_, b) => {
                                if b.contains(&cur_char){
                                    self.cur_rule += 1;
                                    self.count = 1;
                                    return;
                                }
                        },
                        Token::Timesleast(_, b) => {
                            if b.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 1;
                                return;
                            }
                        }
                    }
                    if a.contains(&cur_char){
                        self.count += 1;
                        return;
                    }else{
                        self.cur_rule = 0;
                        self.count = 0;
                        return;
                    }
            },
            Token::Question(a) => {
                if 2 > self.count{
                    if a.contains(&cur_char){
                        self.count += 1;
                        return;
                    }
                }
                if self.cur_rule + 1 == self.rules.len(){
                    self.accept = true;
                    return;
                }
               match &self.rules[self.cur_rule + 1]{
                        Token::Star(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        }, 
                        Token::Plus(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::Question(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::OR(a, b) => {
                            if a == &cur_char || b == &cur_char{
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::TimesEx(_, b) => {
                                if b.contains(&cur_char){
                                    self.cur_rule += 1;
                                    self.count = 1;
                                    return;
                                }
                        },
                        Token::Timesleast(_, b) => {
                            if b.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 1;
                                return;
                            }
                        }
                    }
                        self.cur_rule = 0;
                        self.count = 0;
                        return;
            },
            Token::OR(a, b) => {
                if a == &cur_char || b == &cur_char{
                    self.cur_rule += 1;
                    self.count = 0;
                    return;
                }else{
                    self.cur_rule = 0;
                    self.count = 0;
                    return;
                }
            },
            Token::TimesEx(a, b) => {
                if a > &self.count{
                    if b.contains(&cur_char){
                        self.count += 1;
                        return;
                    }else{
                        self.cur_rule = 0;
                        self.count = 0;
                        return;
                    }
                }else if a < &self.count{
                        self.cur_rule = 0;
                        self.count = 0;
                        return;
                    }
                    if self.cur_rule + 1 == self.rules.len(){
                        self.accept = true;
                        return;
                    }
               match &self.rules[self.cur_rule + 1]{
                        Token::Star(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        }, 
                        Token::Plus(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::Question(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::OR(a, b) => {
                            if a == &cur_char || b == &cur_char{
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::TimesEx(_, b) => {
                                if b.contains(&cur_char){
                                    self.cur_rule += 1;
                                    self.count = 1;
                                    return;
                                }
                        },
                        Token::Timesleast(_, b) => {
                            if b.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 1;
                                return;
                            }
                        }
                    }
                    if b.contains(&cur_char){
                        self.count += 1;
                        return;
                    }else{
                        self.cur_rule = 0;
                        self.count = 0;
                        return;
                    }
            },
            Token::Timesleast(a, b) => {
                if a > &self.count{
                    if b.contains(&cur_char){
                        self.count += 1;
                        return;
                    }else{
                        self.cur_rule = 0;
                        self.count = 0;
                        return;
                    }
                }
                if self.cur_rule + 1 == self.rules.len(){
                    self.accept = true;
                    return;
                }
               match &self.rules[self.cur_rule + 1]{
                        Token::Star(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        }, 
                        Token::Plus(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::Question(a) => {
                            if a.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::OR(a, b) => {
                            if a == &cur_char || b == &cur_char{
                                self.cur_rule += 1;
                                self.count = 0;
                                return;
                            }
                        },
                        Token::TimesEx(_, b) => {
                                if b.contains(&cur_char){
                                    self.cur_rule += 1;
                                    self.count = 1;
                                    return;
                                }
                        },
                        Token::Timesleast(_, b) => {
                            if b.contains(&cur_char){
                                self.cur_rule += 1;
                                self.count = 1;
                                return;
                            }
                        }
                    }
                    if b.contains(&cur_char){
                        self.count += 1;
                        return;
                    }else{
                        self.cur_rule = 0;
                        self.count = 0;
                        return;
                    }
                }
            }
        }
    }





fn regex(reg: String) -> Box<impl Fn(String) -> bool> {
    let rules = parsereg(None, vec![], reg.chars(), vec![]);

    return Box::new(move |str: String| {
        return parser(str.chars(), rules.clone());
    });
}

fn parser(chain: Chars, rules: Vec<Token>) -> bool{
    if rules.len() == 0{
        return true;
    }
    let mut parser = Parser::new(rules);


    for char in chain{
        parser.parse(char);
    }

    return parser.accept;
}

fn parsereg(
    prev_tok: Option<Token>,
    mut char_store: Vec<char>,
    mut it: Chars,
    mut tokens: Vec<Token>,
) -> Vec<Token> {
    let nextchar;
    match it.next() {
        Some(char) => nextchar = char,
        None => {
            if let Some(_) = prev_tok {
                panic!("Regex isnt formated correctly!")
            }
            if char_store.len() != 0 {
                panic!("Regex isnt formated correctly!")
            }
            return tokens;
        }
    }
    //if prevtok then parse further to populate,
    if let Some(tok) = prev_tok {
        match tok {
            Token::Plus(_) => panic!("Regex isnt formated correctly!"),
            Token::Star(_) => panic!("Regex isnt formated correctly!"),
            Token::Question(_) => panic!("Regex isnt formated correctly!"),
            Token::OR(a, _) => {
                tokens.push(Token::OR(a, nextchar));
                return parsereg(None, char_store, it, tokens);
            }
            Token::TimesEx(a, b) => {
                if a == 0 {
                    return parsereg(
                        Some(Token::TimesEx(nextchar.to_digit(10).unwrap(), b)),
                        char_store,
                        it,
                        tokens,
                    );
                } else if nextchar == ',' {
                    tokens.push(Token::Timesleast(a, b));
                    return parsereg(None, char_store, it, tokens);
                } else {
                    tokens.push(Token::TimesEx(a, b));
                    return parsereg(None, char_store, it, tokens);
                }
            }
            Token::Timesleast(_, _) => panic!("Regex isnt formated correctly!"),
        }
    } else {
        match nextchar {
            nextchar if nextchar.is_alphabetic() => {
                char_store.push(nextchar);
                return parsereg(prev_tok, char_store, it, tokens);
            }
            '*' => {
                tokens.push(Token::Star(char_store));
                char_store = vec![];
                return parsereg(prev_tok, char_store, it, tokens);
            }
            '+' => {
                tokens.push(Token::Plus(char_store));
                char_store = vec![];
                return parsereg(prev_tok, char_store, it, tokens);
            }
            '?' => {
                tokens.push(Token::Question(char_store));
                char_store = vec![];
                return parsereg(prev_tok, char_store, it, tokens);
            }
            '{' => {
                let temp = Some(Token::TimesEx(0, char_store));
                char_store = vec![];
                return parsereg(temp, char_store, it, tokens);
            }
            '}' => {
                return parsereg(prev_tok, char_store, it, tokens);
            }
            '|' => {
                if char_store.len() != 1 {
                    panic!("Regex isnt formated correctly!")
                } else {
                    let temp = Some(Token::OR(char_store[0], ' '));
                    char_store = vec![];
                    return parsereg(temp, char_store, it, tokens);
                }
            }
            _ => panic!("Regex isnt formated correctly!"),
        }
    }
}
