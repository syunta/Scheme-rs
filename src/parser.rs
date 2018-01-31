use data::Obj;

pub struct Parser {
    pub unparsed: String
}

impl Parser {
    pub fn parse(&mut self) -> Obj {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Obj {
        loop {
            let ch = self.peek();
            match ch {
                Some(c) => {
                    if c.is_digit(10) {
                        return self.parse_integer()
                    }
                },
                None => return Obj::Nil
            }
        }
    }

    fn parse_integer(&mut self) -> Obj {
        let mut val = 0;
        loop {
            let ch = self.get_char();
            match ch {
                Some(c) => {
                    if c.is_digit(10) {
                        val = val * 10 + c.to_digit(10).unwrap() as i32;
                        continue;
                    }
                    if is_delimiter(c) {
                        return Obj::Int(val)
                    }
                },
                None => return Obj::Int(val)
            }
        }
    }

    fn peek(&self) -> Option<char> {
        if self.unparsed.is_empty() {
            return None
        }
        self.unparsed.chars().nth(0)
    }

    fn get_char(&mut self) -> Option<char> {
        if self.unparsed.is_empty() {
            return None
        }
        Some(self.unparsed.remove(0))
    }
}

fn is_delimiter(c: char) -> bool {
    c == ' ' || c == '\n' || c == '\r' || c == '\t'
}

#[test]
fn it_works() {
    let mut p = Parser { unparsed: "123".to_string() };
    assert_eq!(Obj::Int(123), p.parse());
}
