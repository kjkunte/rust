use std :: {
    collections::VecDeque,
    fmt,
    io::{self,BufRead},
    str::FromStr,
}
struct Scanner {
    tokens: VecDeque<String>,
}
impl Scanner { // method scanner called on type struct Scanner with variable of the the type deque
    pub fn new() -> Self{
        let stdin = io::stdin();
        let mut tokens = VecDeque::new();
        for line in stdin.lock().lines() {
            for token in line.unwrap().split_ascii_whitespace(){
                tokens.push_back(token.to_owned());
            }
        }
        Self {tokens}
    }
    pub fn next<T: FromStr> (&mut self) -> T
    where
    <T as FromStr>:: Err:fmt:: Debug,
    {
        T::from_str(&self.tokens.pop_front().unwrap()).unwrap()
    }
}

fn main() {
    let mut input = Scanner::new();
    let tests:i32 = input.next();
}


