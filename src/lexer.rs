pub enum IntegerToken {
    U64(u64),
}

pub enum SymbolToken {
    Plus,
    Minus,
    Times,
    Divide,
}

pub enum Token<'src> {
    Identifier(&'src str),
    Integer(IntegerToken),
    Symbol(SymbolToken),
}

#[derive(Copy, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

pub struct LocatedToken<'str> {
    token: Token<'str>,
    location: Location,
}

pub struct Lexer<'src> {
    text: &'src str,
    location: Location,
}

impl<'src> Lexer<'src> {
    pub fn new(text: &'src str) -> Lexer<'src> {
        Lexer {
            text,
            location: Location { line: 1, column: 1 },
        }
    }
    pub fn read_while(&mut self, mut pred: impl FnMut(char) -> bool) -> (Location, &'src str) {
        let loc = self.location;
        let index = 0;
        for (i, c) in self.text.char_indices() {
            index = i;
            if !pred(c) {
                break;
            }
        }
        let (first, last) = self.text.split_at(index);
        self.text = last;
        (loc, first)
    }
}

pub enum LexerError<'src> {
    BadToken(&'src str),
}

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<LocatedToken<'src>, LexerError<'src>>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}
