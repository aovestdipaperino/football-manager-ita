//! Lexer, AST, and parser for the subset of C64 BASIC V2 used by
//! `footballmanager.txt` (petcat-format plain text).
//!
//! The lexer is deliberately greedy about keywords: `FORAPE=1TO16` is
//! recognised as `FOR APE = 1 TO 16` exactly as the on-ROM tokeniser
//! would have done. String literals embed petcat escapes (`{clr}`,
//! `{$c0}`, colour names, …) which are decoded to PETSCII bytes.

use crate::petscii::name_to_byte;

// ---------------------------------------------------------------------------
// Tokens
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kw {
    Goto,
    Gosub,
    Return,
    If,
    Then,
    For,
    To,
    Step,
    Next,
    Print,
    Input,
    Get,
    Dim,
    Data,
    Read,
    Restore,
    On,
    Rem,
    End,
    Stop,
    Poke,
    Let,
    Run,
    Clr,
    New,
    Tab,
    Spc,
    And,
    Or,
    Not,
    Chr,
    Left,
    Right,
    Mid,
    Str,
    Val,
    Asc,
    Len,
    Int,
    Abs,
    Sgn,
    Sqr,
    Rnd,
    Peek,
    Fre,
    Pos,
}

impl Kw {
    /// Matches a keyword at the start of `s`, returning (keyword, consumed bytes).
    /// Longest-match wins so `STEP` beats `STO` etc.
    fn try_match(s: &[u8]) -> Option<(Kw, usize)> {
        // Ordered by descending length so prefix conflicts resolve correctly.
        const TABLE: &[(&[u8], Kw)] = &[
            (b"RESTORE", Kw::Restore),
            (b"RETURN", Kw::Return),
            (b"GOSUB", Kw::Gosub),
            (b"RIGHT$", Kw::Right),
            (b"LEFT$", Kw::Left),
            (b"INPUT", Kw::Input),
            (b"PRINT", Kw::Print),
            (b"STEP", Kw::Step),
            (b"DATA", Kw::Data),
            (b"READ", Kw::Read),
            (b"THEN", Kw::Then),
            (b"NEXT", Kw::Next),
            (b"POKE", Kw::Poke),
            (b"PEEK", Kw::Peek),
            (b"GOTO", Kw::Goto),
            (b"STOP", Kw::Stop),
            (b"MID$", Kw::Mid),
            (b"CHR$", Kw::Chr),
            (b"STR$", Kw::Str),
            (b"TAB(", Kw::Tab),
            (b"SPC(", Kw::Spc),
            (b"DIM", Kw::Dim),
            (b"FOR", Kw::For),
            (b"GET", Kw::Get),
            (b"REM", Kw::Rem),
            (b"END", Kw::End),
            (b"LET", Kw::Let),
            (b"RUN", Kw::Run),
            (b"CLR", Kw::Clr),
            (b"NEW", Kw::New),
            (b"AND", Kw::And),
            (b"NOT", Kw::Not),
            (b"INT", Kw::Int),
            (b"ABS", Kw::Abs),
            (b"SGN", Kw::Sgn),
            (b"SQR", Kw::Sqr),
            (b"RND", Kw::Rnd),
            (b"LEN", Kw::Len),
            (b"VAL", Kw::Val),
            (b"ASC", Kw::Asc),
            (b"FRE", Kw::Fre),
            (b"POS", Kw::Pos),
            (b"IF", Kw::If),
            (b"TO", Kw::To),
            (b"ON", Kw::On),
            (b"OR", Kw::Or),
        ];
        for (kw, tok) in TABLE {
            if s.len() >= kw.len() && s[..kw.len()].eq_ignore_ascii_case(kw) {
                return Some((*tok, kw.len()));
            }
        }
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    Eq,
    Lt,
    Gt,
    Le,
    Ge,
    Ne,
}

#[derive(Debug, Clone)]
pub enum Token {
    Num(f64),
    Str(Vec<u8>),
    Ident(String), // includes trailing $ or % if present
    Kw(Kw),
    Op(Op),
    LParen,
    RParen,
    Comma,
    Semi,
    Colon,
    Eol,
}

// ---------------------------------------------------------------------------
// Lexer – runs on a single logical line of source
// ---------------------------------------------------------------------------

pub struct Lexer<'a> {
    src: &'a [u8],
    pos: usize,
    after_rem: bool,
    in_data: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        Lexer {
            src,
            pos: 0,
            after_rem: false,
            in_data: false,
        }
    }

    fn peek(&self) -> Option<u8> {
        self.src.get(self.pos).copied()
    }
    fn bump(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.pos += 1;
        Some(b)
    }

    fn skip_spaces(&mut self) {
        while let Some(b) = self.peek() {
            if b == b' ' || b == b'\t' {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    /// Next token. Returns Token::Eol at end of input.
    pub fn next(&mut self) -> Result<Token, String> {
        if self.after_rem {
            self.pos = self.src.len();
            return Ok(Token::Eol);
        }
        if self.in_data {
            return Ok(self.next_data_token());
        }
        self.skip_spaces();
        let b = match self.peek() {
            None => return Ok(Token::Eol),
            Some(b) => b,
        };

        // String literal with petcat escapes.
        if b == b'"' {
            return self.lex_string();
        }

        // Number (leading digit or '.').
        if b.is_ascii_digit()
            || (b == b'.'
                && self
                    .src
                    .get(self.pos + 1)
                    .is_some_and(|c| c.is_ascii_digit()))
        {
            return Ok(self.lex_number());
        }

        // Keyword or identifier – letters only.
        if b.is_ascii_alphabetic() {
            if let Some((kw, n)) = Kw::try_match(&self.src[self.pos..]) {
                self.pos += n;
                if matches!(kw, Kw::Rem) {
                    self.after_rem = true;
                }
                if matches!(kw, Kw::Data) {
                    self.in_data = true;
                }
                return Ok(Token::Kw(kw));
            }
            return Ok(self.lex_ident());
        }

        // Punctuation / operators.
        self.pos += 1;
        Ok(match b {
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b',' => Token::Comma,
            b';' => Token::Semi,
            b':' => Token::Colon,
            b'+' => Token::Op(Op::Plus),
            b'-' => Token::Op(Op::Minus),
            b'*' => Token::Op(Op::Star),
            b'/' => Token::Op(Op::Slash),
            b'^' => Token::Op(Op::Caret),
            b'=' => Token::Op(Op::Eq),
            b'<' => {
                if self.peek() == Some(b'=') {
                    self.pos += 1;
                    Token::Op(Op::Le)
                } else if self.peek() == Some(b'>') {
                    self.pos += 1;
                    Token::Op(Op::Ne)
                } else {
                    Token::Op(Op::Lt)
                }
            }
            b'>' => {
                if self.peek() == Some(b'=') {
                    self.pos += 1;
                    Token::Op(Op::Ge)
                } else {
                    Token::Op(Op::Gt)
                }
            }
            _ => return Err(format!("unexpected byte {:#x} at pos {}", b, self.pos - 1)),
        })
    }

    /// Tokeniser shape used between DATA and the next `:` or EOL.
    /// Emits one Str token per comma-separated item.
    fn next_data_token(&mut self) -> Token {
        // Optional whitespace between items is part of the item unless it's leading.
        self.skip_spaces();
        match self.peek() {
            None => {
                self.in_data = false;
                Token::Eol
            }
            Some(b':') => {
                self.in_data = false;
                self.pos += 1;
                Token::Colon
            }
            Some(b',') => {
                self.pos += 1;
                Token::Comma
            }
            Some(b'"') => self.lex_string().unwrap_or(Token::Str(Vec::new())),
            Some(_) => {
                let start = self.pos;
                while let Some(c) = self.peek() {
                    if c == b',' || c == b':' {
                        break;
                    }
                    self.pos += 1;
                }
                let mut end = self.pos;
                // Trim trailing whitespace.
                while end > start && matches!(self.src[end - 1], b' ' | b'\t') {
                    end -= 1;
                }
                Token::Str(self.src[start..end].to_vec())
            }
        }
    }

    fn lex_number(&mut self) -> Token {
        let start = self.pos;
        while let Some(b) = self.peek() {
            if b.is_ascii_digit() {
                self.pos += 1;
            } else {
                break;
            }
        }
        if self.peek() == Some(b'.') {
            self.pos += 1;
            while let Some(b) = self.peek() {
                if b.is_ascii_digit() {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            while let Some(b) = self.peek() {
                if b.is_ascii_digit() {
                    self.pos += 1;
                } else {
                    break;
                }
            }
        }
        let s = std::str::from_utf8(&self.src[start..self.pos]).unwrap();
        Token::Num(s.parse().unwrap_or(0.0))
    }

    fn lex_ident(&mut self) -> Token {
        // A variable name: 1+ letters, then optional letters/digits. Stops
        // when a keyword prefix is seen so that `IFA` -> IF + A.
        let start = self.pos;
        // Consume at least one leading alpha.
        self.pos += 1;
        while let Some(b) = self.peek() {
            let next_is_alnum = b.is_ascii_alphanumeric();
            if !next_is_alnum {
                break;
            }
            if Kw::try_match(&self.src[self.pos..]).is_some() {
                break;
            }
            self.pos += 1;
        }
        let body_end = self.pos;
        // Optional type suffix.
        let mut suffix = None;
        if matches!(self.peek(), Some(b'$' | b'%')) {
            suffix = Some(self.src[self.pos]);
            self.pos += 1;
        }
        // CBM BASIC V2 only considers the first two characters of a name
        // significant. `GIR`, `GIO`, and `GI` are all the same variable.
        let raw = &self.src[start..body_end];
        let kept = &raw[..raw.len().min(2)];
        let mut name = std::str::from_utf8(kept).unwrap().to_ascii_uppercase();
        if let Some(s) = suffix {
            name.push(s as char);
        }
        Token::Ident(name)
    }

    fn lex_string(&mut self) -> Result<Token, String> {
        assert_eq!(self.bump(), Some(b'"'));
        let mut out = Vec::new();
        while let Some(b) = self.peek() {
            if b == b'"' {
                self.pos += 1;
                return Ok(Token::Str(out));
            }
            if b == b'{' {
                // {name} or {$xx}
                self.pos += 1;
                let start = self.pos;
                while let Some(c) = self.peek() {
                    if c == b'}' {
                        break;
                    }
                    self.pos += 1;
                }
                let tag = &self.src[start..self.pos];
                if self.peek() != Some(b'}') {
                    return Err("unterminated petcat escape".into());
                }
                self.pos += 1;
                let tag_str = std::str::from_utf8(tag).unwrap_or("");
                if let Some(hex) = tag_str.strip_prefix('$') {
                    let byte = u8::from_str_radix(hex.trim(), 16)
                        .map_err(|_| format!("bad hex escape {{{}}}", tag_str))?;
                    out.push(byte);
                } else if let Some(byte) = name_to_byte(tag_str) {
                    out.push(byte);
                } else {
                    return Err(format!("unknown petcat tag {{{}}}", tag_str));
                }
                continue;
            }
            self.pos += 1;
            out.push(b);
        }
        // End-of-line closes an unterminated string (classic CBM behaviour).
        Ok(Token::Str(out))
    }
}

// ---------------------------------------------------------------------------
// AST
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Str(Vec<u8>),
    Var(String),
    Index(String, Vec<Expr>),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
    Call(BuiltinFn, Vec<Expr>),
}

#[derive(Debug, Clone, Copy)]
pub enum UnOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum BuiltinFn {
    Int,
    Abs,
    Sgn,
    Sqr,
    Rnd,
    Chr,
    Asc,
    Len,
    Val,
    Str,
    Left,
    Right,
    Mid,
    Peek,
    Fre,
    Pos,
    Tab,
    Spc,
}

/// Items inside a PRINT statement.
#[derive(Debug, Clone)]
pub enum PrintItem {
    Expr(Expr),
    Tab(Expr),
    Spc(Expr),
    Comma, // 10-column zone advance (simplified to a tab)
    Semi,  // no spacing
}

#[derive(Debug, Clone)]
pub enum LValue {
    Var(String),
    Index(String, Vec<Expr>),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let(LValue, Expr),
    Print(Vec<PrintItem>, bool /* trailing newline */),
    Goto(u32),
    Gosub(u32),
    Return,
    /// If condition is *false*, skip this many subsequent statements on the
    /// same line. A flat layout so FOR/NEXT inside the body resolve to
    /// well-defined (line, stmt_idx) resume points.
    IfFalseSkip(Expr, usize),
    For {
        var: String,
        start: Expr,
        end: Expr,
        step: Option<Expr>,
    },
    Next(Vec<String>),
    Dim(Vec<(String, Vec<Expr>)>),
    Data(Vec<DataItem>),
    Read(Vec<LValue>),
    Restore(Option<u32>),
    Get(LValue),
    Input {
        prompt: Option<Vec<u8>>,
        targets: Vec<LValue>,
    },
    On {
        value: Expr,
        is_gosub: bool,
        targets: Vec<u32>,
    },
    Poke(Expr, Expr),
    End,
    Stop,
    Rem,
    Run,
    Clr,
}

#[derive(Debug, Clone)]
pub enum DataItem {
    Num(f64),
    Str(Vec<u8>),
}

// ---------------------------------------------------------------------------
// Parser – eats a token stream and produces Vec<Stmt>
// ---------------------------------------------------------------------------

pub struct Parser {
    toks: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(toks: Vec<Token>) -> Self {
        Self { toks, pos: 0 }
    }

    fn peek(&self) -> &Token {
        &self.toks[self.pos]
    }
    fn bump(&mut self) -> Token {
        let t = self.toks[self.pos].clone();
        self.pos += 1;
        t
    }

    fn expect(&mut self, cond: impl Fn(&Token) -> bool, what: &str) -> Result<Token, String> {
        if cond(self.peek()) {
            Ok(self.bump())
        } else {
            Err(format!("expected {}, got {:?}", what, self.peek()))
        }
    }

    pub fn parse_line(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        self.parse_stmt_seq(&mut stmts, false)?;
        Ok(stmts)
    }

    /// Parse a sequence of `stmt : stmt : ...` until EOL.
    /// If `in_if_body`, we do not allow a trailing end-of-line issue.
    fn parse_stmt_seq(&mut self, out: &mut Vec<Stmt>, _in_if_body: bool) -> Result<(), String> {
        loop {
            while matches!(self.peek(), Token::Colon) {
                self.bump();
            }
            if matches!(self.peek(), Token::Eol) {
                break;
            }
            if matches!(self.peek(), Token::Kw(Kw::If)) {
                self.bump();
                self.emit_if(out)?;
                // THEN body consumes the rest of the line – nothing more to parse.
                break;
            }
            out.push(self.parse_stmt()?);
            match self.peek() {
                Token::Colon => {
                    self.bump();
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn emit_if(&mut self, out: &mut Vec<Stmt>) -> Result<(), String> {
        let cond = self.parse_expr()?;
        if matches!(self.peek(), Token::Kw(Kw::Then)) {
            self.bump();
        }
        let start = out.len();
        out.push(Stmt::IfFalseSkip(cond, 0)); // patched below
        if matches!(self.peek(), Token::Num(_)) {
            // IF c THEN <line>   -> body is a single GOTO.
            out.push(Stmt::Goto(self.parse_line_no()?));
        } else {
            self.parse_stmt_seq(out, true)?;
        }
        let body_len = out.len() - start - 1;
        if let Stmt::IfFalseSkip(_, n) = &mut out[start] {
            *n = body_len;
        }
        Ok(())
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek().clone() {
            Token::Kw(Kw::Rem) => {
                self.bump();
                // discard everything until end of stream / colon
                while !matches!(self.peek(), Token::Eol) {
                    self.bump();
                }
                Ok(Stmt::Rem)
            }
            Token::Kw(Kw::Data) => {
                self.bump();
                let mut items = Vec::new();
                loop {
                    let item = match self.peek().clone() {
                        Token::Num(n) => {
                            self.bump();
                            DataItem::Num(n)
                        }
                        Token::Op(Op::Minus) => {
                            self.bump();
                            if let Token::Num(n) = self.bump() {
                                DataItem::Num(-n)
                            } else {
                                return Err("bad DATA".into());
                            }
                        }
                        Token::Str(s) => {
                            self.bump();
                            DataItem::Str(s)
                        }
                        Token::Ident(s) => {
                            self.bump();
                            DataItem::Str(s.into_bytes())
                        }
                        Token::Kw(_) => {
                            // unquoted keyword-like word – treat as bare string
                            let t = self.bump();
                            let s = format!("{:?}", t);
                            DataItem::Str(s.into_bytes())
                        }
                        other => return Err(format!("bad DATA item {:?}", other)),
                    };
                    items.push(item);
                    match self.peek() {
                        Token::Comma => {
                            self.bump();
                        }
                        _ => break,
                    }
                }
                Ok(Stmt::Data(items))
            }
            Token::Kw(Kw::Print) => {
                self.bump();
                self.parse_print()
            }
            Token::Kw(Kw::Goto) => {
                self.bump();
                Ok(Stmt::Goto(self.parse_line_no()?))
            }
            Token::Kw(Kw::Gosub) => {
                self.bump();
                Ok(Stmt::Gosub(self.parse_line_no()?))
            }
            Token::Kw(Kw::Return) => {
                self.bump();
                Ok(Stmt::Return)
            }
            Token::Kw(Kw::End) => {
                self.bump();
                Ok(Stmt::End)
            }
            Token::Kw(Kw::Stop) => {
                self.bump();
                Ok(Stmt::Stop)
            }
            Token::Kw(Kw::Run) => {
                self.bump();
                Ok(Stmt::Run)
            }
            Token::Kw(Kw::Clr) => {
                self.bump();
                Ok(Stmt::Clr)
            }
            Token::Kw(Kw::New) => {
                self.bump();
                Ok(Stmt::Clr)
            }
            Token::Kw(Kw::If) => {
                // IF is handled at statement-sequence level so its body can
                // be laid out flat. parse_stmt is only entered for non-IF.
                Err("internal: IF reached parse_stmt".into())
            }
            Token::Kw(Kw::For) => {
                self.bump();
                self.parse_for()
            }
            Token::Kw(Kw::Next) => {
                self.bump();
                self.parse_next()
            }
            Token::Kw(Kw::Dim) => {
                self.bump();
                self.parse_dim()
            }
            Token::Kw(Kw::Read) => {
                self.bump();
                self.parse_read()
            }
            Token::Kw(Kw::Restore) => {
                self.bump();
                let line = if matches!(self.peek(), Token::Num(_)) {
                    Some(self.parse_line_no()?)
                } else {
                    None
                };
                Ok(Stmt::Restore(line))
            }
            Token::Kw(Kw::Get) => {
                self.bump();
                self.parse_get()
            }
            Token::Kw(Kw::Input) => {
                self.bump();
                self.parse_input()
            }
            Token::Kw(Kw::On) => {
                self.bump();
                self.parse_on()
            }
            Token::Kw(Kw::Poke) => {
                self.bump();
                self.parse_poke()
            }
            Token::Kw(Kw::Let) => {
                self.bump();
                self.parse_assign()
            }
            Token::Ident(_) => self.parse_assign(),
            other => Err(format!(
                "unexpected token at start of statement: {:?}",
                other
            )),
        }
    }

    fn parse_line_no(&mut self) -> Result<u32, String> {
        match self.bump() {
            Token::Num(n) => Ok(n as u32),
            other => Err(format!("expected line number, got {:?}", other)),
        }
    }

    fn parse_lvalue(&mut self) -> Result<LValue, String> {
        let name = match self.bump() {
            Token::Ident(n) => n,
            other => return Err(format!("expected variable, got {:?}", other)),
        };
        if matches!(self.peek(), Token::LParen) {
            self.bump();
            let mut idxs = vec![self.parse_expr()?];
            while matches!(self.peek(), Token::Comma) {
                self.bump();
                idxs.push(self.parse_expr()?);
            }
            self.expect(|t| matches!(t, Token::RParen), ")")?;
            Ok(LValue::Index(name, idxs))
        } else {
            Ok(LValue::Var(name))
        }
    }

    fn parse_assign(&mut self) -> Result<Stmt, String> {
        let lv = self.parse_lvalue()?;
        self.expect(|t| matches!(t, Token::Op(Op::Eq)), "=")?;
        let e = self.parse_expr()?;
        Ok(Stmt::Let(lv, e))
    }

    fn parse_print(&mut self) -> Result<Stmt, String> {
        let mut items = Vec::new();
        let mut trailing_nl = true;
        loop {
            match self.peek() {
                Token::Eol | Token::Colon => break,
                Token::Semi => {
                    self.bump();
                    items.push(PrintItem::Semi);
                    trailing_nl = false;
                }
                Token::Comma => {
                    self.bump();
                    items.push(PrintItem::Comma);
                    trailing_nl = false;
                }
                Token::Kw(Kw::Tab) => {
                    self.bump();
                    let e = self.parse_expr()?;
                    self.expect(|t| matches!(t, Token::RParen), ") after TAB")?;
                    items.push(PrintItem::Tab(e));
                    trailing_nl = true;
                }
                Token::Kw(Kw::Spc) => {
                    self.bump();
                    let e = self.parse_expr()?;
                    self.expect(|t| matches!(t, Token::RParen), ") after SPC")?;
                    items.push(PrintItem::Spc(e));
                    trailing_nl = true;
                }
                _ => {
                    let e = self.parse_expr()?;
                    items.push(PrintItem::Expr(e));
                    trailing_nl = true;
                }
            }
        }
        Ok(Stmt::Print(items, trailing_nl))
    }

    fn parse_for(&mut self) -> Result<Stmt, String> {
        let var = match self.bump() {
            Token::Ident(n) => n,
            other => return Err(format!("expected FOR variable, got {:?}", other)),
        };
        self.expect(|t| matches!(t, Token::Op(Op::Eq)), "=")?;
        let start = self.parse_expr()?;
        self.expect(|t| matches!(t, Token::Kw(Kw::To)), "TO")?;
        let end = self.parse_expr()?;
        let step = if matches!(self.peek(), Token::Kw(Kw::Step)) {
            self.bump();
            Some(self.parse_expr()?)
        } else {
            None
        };
        Ok(Stmt::For {
            var,
            start,
            end,
            step,
        })
    }

    fn parse_next(&mut self) -> Result<Stmt, String> {
        let mut names = Vec::new();
        if let Token::Ident(n) = self.peek().clone() {
            self.bump();
            names.push(n);
            while matches!(self.peek(), Token::Comma) {
                self.bump();
                if let Token::Ident(n) = self.bump() {
                    names.push(n);
                } else {
                    return Err("bad NEXT".into());
                }
            }
        }
        Ok(Stmt::Next(names))
    }

    fn parse_dim(&mut self) -> Result<Stmt, String> {
        let mut dims = Vec::new();
        loop {
            let name = match self.bump() {
                Token::Ident(n) => n,
                other => return Err(format!("expected DIM name, got {:?}", other)),
            };
            self.expect(|t| matches!(t, Token::LParen), "(")?;
            let mut sizes = vec![self.parse_expr()?];
            while matches!(self.peek(), Token::Comma) {
                self.bump();
                sizes.push(self.parse_expr()?);
            }
            self.expect(|t| matches!(t, Token::RParen), ")")?;
            dims.push((name, sizes));
            if matches!(self.peek(), Token::Comma) {
                self.bump();
            } else {
                break;
            }
        }
        Ok(Stmt::Dim(dims))
    }

    fn parse_read(&mut self) -> Result<Stmt, String> {
        let mut targets = vec![self.parse_lvalue()?];
        while matches!(self.peek(), Token::Comma) {
            self.bump();
            targets.push(self.parse_lvalue()?);
        }
        Ok(Stmt::Read(targets))
    }

    fn parse_get(&mut self) -> Result<Stmt, String> {
        Ok(Stmt::Get(self.parse_lvalue()?))
    }

    fn parse_input(&mut self) -> Result<Stmt, String> {
        let prompt = if let Token::Str(s) = self.peek().clone() {
            self.bump();
            // prompt then ';'
            if matches!(self.peek(), Token::Semi | Token::Comma) {
                self.bump();
            }
            Some(s)
        } else {
            None
        };
        let mut targets = vec![self.parse_lvalue()?];
        while matches!(self.peek(), Token::Comma) {
            self.bump();
            targets.push(self.parse_lvalue()?);
        }
        Ok(Stmt::Input { prompt, targets })
    }

    fn parse_on(&mut self) -> Result<Stmt, String> {
        let value = self.parse_expr()?;
        let is_gosub = match self.bump() {
            Token::Kw(Kw::Goto) => false,
            Token::Kw(Kw::Gosub) => true,
            other => return Err(format!("expected GOTO or GOSUB after ON, got {:?}", other)),
        };
        let mut lines = vec![self.parse_line_no()?];
        while matches!(self.peek(), Token::Comma) {
            self.bump();
            lines.push(self.parse_line_no()?);
        }
        Ok(Stmt::On {
            value,
            is_gosub,
            targets: lines,
        })
    }

    fn parse_poke(&mut self) -> Result<Stmt, String> {
        let a = self.parse_expr()?;
        self.expect(|t| matches!(t, Token::Comma), ",")?;
        let b = self.parse_expr()?;
        Ok(Stmt::Poke(a, b))
    }

    // ---------- Expressions ----------
    // Precedence (low→high):  OR, AND, NOT, comparisons, +-, */, ^, unary

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_and()?;
        while matches!(self.peek(), Token::Kw(Kw::Or)) {
            self.bump();
            let rhs = self.parse_and()?;
            lhs = Expr::Binary(BinOp::Or, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_not()?;
        while matches!(self.peek(), Token::Kw(Kw::And)) {
            self.bump();
            let rhs = self.parse_not()?;
            lhs = Expr::Binary(BinOp::And, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_not(&mut self) -> Result<Expr, String> {
        if matches!(self.peek(), Token::Kw(Kw::Not)) {
            self.bump();
            let e = self.parse_not()?;
            return Ok(Expr::Unary(UnOp::Not, Box::new(e)));
        }
        self.parse_cmp()
    }

    fn parse_cmp(&mut self) -> Result<Expr, String> {
        let lhs = self.parse_addsub()?;
        let op = match self.peek() {
            Token::Op(Op::Eq) => BinOp::Eq,
            Token::Op(Op::Ne) => BinOp::Ne,
            Token::Op(Op::Lt) => BinOp::Lt,
            Token::Op(Op::Gt) => BinOp::Gt,
            Token::Op(Op::Le) => BinOp::Le,
            Token::Op(Op::Ge) => BinOp::Ge,
            _ => return Ok(lhs),
        };
        self.bump();
        let rhs = self.parse_addsub()?;
        Ok(Expr::Binary(op, Box::new(lhs), Box::new(rhs)))
    }

    fn parse_addsub(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_muldiv()?;
        loop {
            let op = match self.peek() {
                Token::Op(Op::Plus) => BinOp::Add,
                Token::Op(Op::Minus) => BinOp::Sub,
                _ => break,
            };
            self.bump();
            let rhs = self.parse_muldiv()?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_muldiv(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_pow()?;
        loop {
            let op = match self.peek() {
                Token::Op(Op::Star) => BinOp::Mul,
                Token::Op(Op::Slash) => BinOp::Div,
                _ => break,
            };
            self.bump();
            let rhs = self.parse_pow()?;
            lhs = Expr::Binary(op, Box::new(lhs), Box::new(rhs));
        }
        Ok(lhs)
    }

    fn parse_pow(&mut self) -> Result<Expr, String> {
        let lhs = self.parse_unary()?;
        if matches!(self.peek(), Token::Op(Op::Caret)) {
            self.bump();
            let rhs = self.parse_pow()?;
            return Ok(Expr::Binary(BinOp::Pow, Box::new(lhs), Box::new(rhs)));
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if matches!(self.peek(), Token::Op(Op::Minus)) {
            self.bump();
            return Ok(Expr::Unary(UnOp::Neg, Box::new(self.parse_unary()?)));
        }
        if matches!(self.peek(), Token::Op(Op::Plus)) {
            self.bump();
            return self.parse_unary();
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            Token::Num(n) => {
                self.bump();
                Ok(Expr::Num(n))
            }
            Token::Str(s) => {
                self.bump();
                Ok(Expr::Str(s))
            }
            Token::LParen => {
                self.bump();
                let e = self.parse_expr()?;
                self.expect(|t| matches!(t, Token::RParen), ")")?;
                Ok(e)
            }
            Token::Kw(k) => {
                let f = match k {
                    Kw::Int => BuiltinFn::Int,
                    Kw::Abs => BuiltinFn::Abs,
                    Kw::Sgn => BuiltinFn::Sgn,
                    Kw::Sqr => BuiltinFn::Sqr,
                    Kw::Rnd => BuiltinFn::Rnd,
                    Kw::Chr => BuiltinFn::Chr,
                    Kw::Asc => BuiltinFn::Asc,
                    Kw::Len => BuiltinFn::Len,
                    Kw::Val => BuiltinFn::Val,
                    Kw::Str => BuiltinFn::Str,
                    Kw::Left => BuiltinFn::Left,
                    Kw::Right => BuiltinFn::Right,
                    Kw::Mid => BuiltinFn::Mid,
                    Kw::Peek => BuiltinFn::Peek,
                    Kw::Fre => BuiltinFn::Fre,
                    Kw::Pos => BuiltinFn::Pos,
                    Kw::Tab => BuiltinFn::Tab, // only via print item ordinarily
                    Kw::Spc => BuiltinFn::Spc,
                    _ => return Err(format!("keyword {:?} not valid in expression", k)),
                };
                self.bump();
                self.expect(|t| matches!(t, Token::LParen), "( after function")?;
                let mut args = vec![self.parse_expr()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    args.push(self.parse_expr()?);
                }
                self.expect(|t| matches!(t, Token::RParen), ")")?;
                Ok(Expr::Call(f, args))
            }
            Token::Ident(name) => {
                self.bump();
                if matches!(self.peek(), Token::LParen) {
                    self.bump();
                    let mut idxs = vec![self.parse_expr()?];
                    while matches!(self.peek(), Token::Comma) {
                        self.bump();
                        idxs.push(self.parse_expr()?);
                    }
                    self.expect(|t| matches!(t, Token::RParen), ")")?;
                    Ok(Expr::Index(name, idxs))
                } else {
                    Ok(Expr::Var(name))
                }
            }
            other => Err(format!("bad primary expression: {:?}", other)),
        }
    }
}

// ---------------------------------------------------------------------------
// Program loader: consumes a whole source string, produces line_number -> stmts
// ---------------------------------------------------------------------------

use std::collections::BTreeMap;

pub type Program = BTreeMap<u32, Vec<Stmt>>;

pub fn tokenise_line(src: &[u8]) -> Result<Vec<Token>, String> {
    let mut lex = Lexer::new(src);
    let mut toks = Vec::new();
    loop {
        let t = lex.next()?;
        let is_eol = matches!(t, Token::Eol);
        toks.push(t);
        if is_eol {
            break;
        }
    }
    Ok(toks)
}

pub fn load_program(src: &str) -> Result<Program, String> {
    let mut prog = Program::new();
    for (idx, raw) in src.lines().enumerate() {
        let line = raw.trim_end_matches('\r').trim();
        if line.is_empty() {
            continue;
        }
        // Parse line number.
        let mut it = line.bytes();
        let mut num_str = String::new();
        while let Some(b) = it.clone().next() {
            if b.is_ascii_digit() {
                num_str.push(b as char);
                let _ = it.next();
            } else {
                break;
            }
        }
        if num_str.is_empty() {
            return Err(format!("line {} has no line number: {:?}", idx + 1, line));
        }
        let line_no: u32 = num_str.parse().unwrap();
        let rest = &line.as_bytes()[num_str.len()..];
        let rest = trim_leading_spaces(rest);
        let toks = tokenise_line(rest)
            .map_err(|e| format!("line {} (#{}): lex error: {}", idx + 1, line_no, e))?;
        let mut parser = Parser::new(toks);
        let stmts = parser
            .parse_line()
            .map_err(|e| format!("line {} (#{}): parse error: {}", idx + 1, line_no, e))?;
        prog.insert(line_no, stmts);
    }
    Ok(prog)
}

fn trim_leading_spaces(s: &[u8]) -> &[u8] {
    let mut i = 0;
    while i < s.len() && (s[i] == b' ' || s[i] == b'\t') {
        i += 1;
    }
    &s[i..]
}
