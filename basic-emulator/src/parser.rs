use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Print(PrintStatement),
    Input(InputStatement),
    Let(LetStatement),
    If(IfStatement),
    Goto(u32),
    Gosub(u32),
    Return,
    For(ForStatement),
    Next(Option<String>),
    Dim(Vec<DimDeclaration>),
    Data(Vec<String>),
    Read(Vec<String>),
    Poke(Box<Expr>, Box<Expr>),
    End,
    Rem(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrintStatement {
    pub items: Vec<PrintItem>,
    pub trailing_semicolon: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrintItem {
    Expr(Expr),
    Tab(Box<Expr>),
    Spc(Box<Expr>),
    Comma,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputStatement {
    pub prompt: Option<String>,
    pub variable: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub variable: String,
    pub index: Option<Vec<Expr>>,
    pub value: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfStatement {
    pub condition: Expr,
    pub then_branch: Vec<Statement>,
    pub then_line: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForStatement {
    pub variable: String,
    pub start: Expr,
    pub end: Expr,
    pub step: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DimDeclaration {
    pub name: String,
    pub dimensions: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(f64),
    String(String),
    Variable(String),
    ArrayAccess(String, Vec<Expr>),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    UnaryOp(UnOp, Box<Expr>),
    FunctionCall(String, Vec<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOp {
    Neg,
    Not,
}

pub struct Program {
    pub lines: BTreeMap<u32, Vec<Statement>>,
}

impl Program {
    pub fn new() -> Self {
        Program {
            lines: BTreeMap::new(),
        }
    }
}

pub struct Parser {
    input: String,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let normalized = Self::normalize_keywords(&input.to_uppercase());
        Parser {
            input: normalized,
            pos: 0,
        }
    }

    fn normalize_keywords(input: &str) -> String {
        // First pass: normalize statement keywords
        // CRITICAL: Must skip string literals! Keywords in strings should NOT be normalized
        let statement_keywords = [
            "RETURN", "GOSUB", "INPUT", "PRINT", "GOTO", "THEN", "NEXT",
            "DATA", "READ", "POKE", "FOR", "DIM", "END", "REM", "IF",
        ];

        let mut result = String::new();
        let mut i = 0;
        let chars: Vec<char> = input.chars().collect();
        let mut in_string = false;

        while i < chars.len() {
            // Track string literal boundaries
            if chars[i] == '"' {
                in_string = !in_string;
                result.push('"');
                i += 1;
                continue;
            }

            // If we're in a string, just copy characters
            if in_string {
                result.push(chars[i]);
                i += 1;
                continue;
            }

            let mut matched = false;

            // Try to match statement keywords first
            for keyword in &statement_keywords {
                let keyword_len = keyword.len();
                if i + keyword_len <= chars.len() {
                    let slice: String = chars[i..i + keyword_len].iter().collect();
                    if slice == *keyword {
                        // Always match keywords - they take precedence
                        // This handles both THENPRINT and I>ZTHEN correctly
                        result.push_str(keyword);
                        i += keyword_len;

                        // Add space if next char would make this look like part of an identifier
                        if i < chars.len() {
                            let next = chars[i];
                            if next.is_ascii_alphanumeric() || next == '$' || next == '%' {
                                result.push(' ');
                            }
                        }

                        matched = true;
                        break;
                    }
                }
            }

            if !matched {
                result.push(chars[i]);
                i += 1;
            }
        }

        // Second pass: normalize OR and AND in expressions
        // CRITICAL: Must skip string literals! OR/AND in strings should NOT be normalized
        // Example: "[BORDERS]" should NOT become "[B OR DERS]"
        let mut second_pass = String::new();
        let result_chars: Vec<char> = result.chars().collect();
        let mut i = 0;
        let mut in_string = false;

        while i < result_chars.len() {
            let ch = result_chars[i];

            // Track string literal boundaries
            if ch == '"' {
                in_string = !in_string;
                second_pass.push(ch);
                i += 1;
                continue;
            }

            // If we're in a string, just copy characters
            if in_string {
                second_pass.push(ch);
                i += 1;
                continue;
            }

            let mut matched = false;

            // Try to match AND first (3 chars) to avoid matching the 'AN' part as something else
            if i + 3 <= result_chars.len() {
                let slice: String = result_chars[i..i + 3].iter().collect();
                if slice == "AND" {
                    let prev_char = if i > 0 { Some(result_chars[i - 1]) } else { None };
                    let next_char = if i + 3 < result_chars.len() { Some(result_chars[i + 3]) } else { None };

                    // Consider $ and % as part of identifiers (BASIC variable suffixes)
                    let is_identifier_char = |c: char| c.is_alphanumeric() || c == '$' || c == '%';
                    let prev_not_id = prev_char.map_or(true, |c| !is_identifier_char(c));
                    let next_not_id = next_char.map_or(true, |c| !is_identifier_char(c));

                    // Normalize when:
                    // 1. Both sides are boundaries (isolated keyword)
                    // 2. Both sides are non-boundaries (embedded like HZANDQZ)
                    // 3. Mixed, but prev is NOT an identifier char and next IS (like 8ANDXZ or "ANDA$)
                    let should_normalize = (prev_not_id && next_not_id) ||
                                          (!prev_not_id && !next_not_id) ||
                                          (prev_not_id && !next_not_id);

                    if should_normalize {
                        second_pass.push_str(" AND ");
                        i += 3;
                        matched = true;
                    }
                }
            }

            // Try to match OR (2 chars)
            if !matched && i + 2 <= result_chars.len() {
                let slice: String = result_chars[i..i + 2].iter().collect();
                if slice == "OR" {
                    let prev_char = if i > 0 { Some(result_chars[i - 1]) } else { None };
                    let next_char = if i + 2 < result_chars.len() { Some(result_chars[i + 2]) } else { None };

                    // Consider $ and % as part of identifiers (BASIC variable suffixes)
                    let is_identifier_char = |c: char| c.is_alphanumeric() || c == '$' || c == '%';
                    let prev_not_id = prev_char.map_or(true, |c| !is_identifier_char(c));
                    let next_not_id = next_char.map_or(true, |c| !is_identifier_char(c));

                    // Normalize when:
                    // 1. Both sides are boundaries (isolated keyword)
                    // 2. Both sides are non-boundaries (embedded like HZORQZ)
                    // 3. Mixed, but prev is NOT an identifier char and next IS (like 8ORXZ or "ORA$)
                    let should_normalize = (prev_not_id && next_not_id) ||
                                          (!prev_not_id && !next_not_id) ||
                                          (prev_not_id && !next_not_id);

                    if should_normalize {
                        second_pass.push_str(" OR ");
                        i += 2;
                        matched = true;
                    }
                }
            }

            if !matched {
                second_pass.push(result_chars[i]);
                i += 1;
            }
        }

        // Third pass: normalize "TO" in FOR loops
        // Look for pattern: FOR var=expr TO expr
        // NOTE: This runs AFTER OR/AND normalization
        // Since we're only normalizing OR/AND at word boundaries, HZTOHZ will still contain TO
        let mut final_result = String::new();
        let second_chars: Vec<char> = second_pass.chars().collect();
        let mut i = 0;
        let mut in_for_loop = false;
        let mut seen_equals = false;

        while i < second_chars.len() {
            // Check if we're starting a FOR loop
            if i + 4 <= second_chars.len() {
                let slice: String = second_chars[i..i + 4].iter().collect();
                if slice == "FOR " {
                    in_for_loop = true;
                    seen_equals = false;
                }
            }

            // Check for '=' in FOR loop
            if in_for_loop && second_chars[i] == '=' {
                seen_equals = true;
            }

            // Check for ':' or newline (end of statement)
            if second_chars[i] == ':' || second_chars[i] == '\n' {
                in_for_loop = false;
                seen_equals = false;
            }

            // Try to match "TO" pattern in FOR loop after =
            if in_for_loop && seen_equals && i + 2 <= second_chars.len() {
                let slice: String = second_chars[i..i + 2].iter().collect();
                if slice == "TO" {
                    // In FOR loop context after '=', TO is unconditionally the keyword
                    final_result.push_str(" TO ");
                    i += 2;
                    in_for_loop = false; // Only one TO per FOR
                    continue;
                }
            }

            final_result.push(second_chars[i]);
            i += 1;
        }

        final_result
    }

    pub fn parse_program(source: &str) -> Result<Program, String> {
        let mut program = Program::new();

        for line in source.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let mut parser = Parser::new(line);
            let (line_num, statements) = parser.parse_line()?;
            program.lines.insert(line_num, statements);
        }

        Ok(program)
    }

    fn parse_line(&mut self) -> Result<(u32, Vec<Statement>), String> {
        self.skip_whitespace();

        // Parse line number
        let line_num = self.parse_number()? as u32;
        self.skip_whitespace();

        // Parse statements (separated by :)
        let mut statements = Vec::new();
        loop {
            self.skip_whitespace();

            if self.is_at_end() {
                break;
            }

            // Skip empty statements (consecutive colons)
            if self.peek() == Some(':') {
                self.advance();
                continue;
            }

            statements.push(self.parse_statement()?);

            self.skip_whitespace();
            if self.peek() == Some(':') {
                self.advance();
            } else {
                break;
            }
        }

        Ok((line_num, statements))
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        // Try to match keywords by checking if input starts with them
        // Check longer keywords first to avoid partial matches
        self.skip_whitespace();
        let remaining = &self.input[self.pos..];

        // Helper to check if a character is part of an identifier (not $ or other special chars)
        let is_keyword_boundary = |c: char| !c.is_ascii_alphabetic() && c != '_';

        if remaining.starts_with("RETURN") && remaining.chars().nth(6).map_or(true, is_keyword_boundary) {
            self.consume_word("RETURN");
            return Ok(Statement::Return);
        }
        if remaining.starts_with("PRINT") && remaining.chars().nth(5).map_or(true, is_keyword_boundary) {
            return self.parse_print();
        }
        if remaining.starts_with("INPUT") && remaining.chars().nth(5).map_or(true, is_keyword_boundary) {
            return self.parse_input();
        }
        if remaining.starts_with("GOSUB") && remaining.chars().nth(5).map_or(true, is_keyword_boundary) {
            return self.parse_gosub();
        }
        if remaining.starts_with("GOTO") && remaining.chars().nth(4).map_or(true, is_keyword_boundary) {
            return self.parse_goto();
        }
        if remaining.starts_with("FOR") && remaining.chars().nth(3).map_or(true, is_keyword_boundary) {
            return self.parse_for();
        }
        if remaining.starts_with("NEXT") && remaining.chars().nth(4).map_or(true, is_keyword_boundary) {
            return self.parse_next();
        }
        if remaining.starts_with("DATA") && remaining.chars().nth(4).map_or(true, is_keyword_boundary) {
            return self.parse_data();
        }
        if remaining.starts_with("READ") && remaining.chars().nth(4).map_or(true, is_keyword_boundary) {
            return self.parse_read();
        }
        if remaining.starts_with("POKE") && remaining.chars().nth(4).map_or(true, is_keyword_boundary) {
            return self.parse_poke();
        }
        if remaining.starts_with("DIM") && remaining.chars().nth(3).map_or(true, is_keyword_boundary) {
            return self.parse_dim();
        }
        if remaining.starts_with("END") && remaining.chars().nth(3).map_or(true, is_keyword_boundary) {
            self.consume_word("END");
            return Ok(Statement::End);
        }
        if remaining.starts_with("REM") && remaining.chars().nth(3).map_or(true, is_keyword_boundary) {
            return self.parse_rem();
        }
        if remaining.starts_with("IF") && remaining.chars().nth(2).map_or(true, is_keyword_boundary) {
            return self.parse_if();
        }

        // Default to LET statement (variable assignment)
        self.parse_let()
    }

    fn parse_print(&mut self) -> Result<Statement, String> {
        self.consume_word("PRINT");
        let mut items = Vec::new();
        let mut trailing_semicolon = false;

        loop {
            self.skip_whitespace();

            if self.is_at_end() || self.peek() == Some(':') {
                break;
            }

            if self.peek() == Some(';') {
                self.advance();
                trailing_semicolon = true;
                continue;
            }

            if self.peek() == Some(',') {
                self.advance();
                items.push(PrintItem::Comma);
                trailing_semicolon = false;
                continue;
            }

            // Check for TAB
            if self.peek_word() == "TAB" {
                self.consume_word("TAB");
                self.skip_whitespace();
                self.expect('(')?;
                let expr = self.parse_expression()?;
                self.expect(')')?;
                items.push(PrintItem::Tab(Box::new(expr)));
                trailing_semicolon = false;
                continue;
            }

            items.push(PrintItem::Expr(self.parse_expression()?));
            trailing_semicolon = false;
        }

        Ok(Statement::Print(PrintStatement {
            items,
            trailing_semicolon,
        }))
    }

    fn parse_input(&mut self) -> Result<Statement, String> {
        self.consume_word("INPUT");
        self.skip_whitespace();

        let mut prompt = None;

        // Check for prompt string
        if self.peek() == Some('"') {
            prompt = Some(self.parse_string_literal()?);
            self.skip_whitespace();
            if self.peek() == Some(';') {
                self.advance();
                self.skip_whitespace();
            }
        }

        let variable = self.parse_identifier()?;

        Ok(Statement::Input(InputStatement { prompt, variable }))
    }

    fn parse_if(&mut self) -> Result<Statement, String> {
        self.consume_word("IF");
        let condition = self.parse_expression()?;

        self.skip_whitespace();

        // THEN is optional in C64 BASIC (e.g., "IF X=1 GOTO 100" or "IF X=1 THEN 100")
        let remaining = &self.input[self.pos..];
        if remaining.starts_with("THEN") {
            self.consume_word("THEN");
            self.skip_whitespace();
        }

        // Check if followed by line number, GOTO, or statements
        if self.peek().map(|c| c.is_ascii_digit()).unwrap_or(false) {
            let line_num = self.parse_number()? as u32;
            Ok(Statement::If(IfStatement {
                condition,
                then_branch: vec![],
                then_line: Some(line_num),
            }))
        } else {
            let mut then_branch = Vec::new();
            loop {
                if self.is_at_end() || self.peek() == Some(':') {
                    break;
                }
                then_branch.push(self.parse_statement()?);
                self.skip_whitespace();
                if self.peek() == Some(':') {
                    self.advance();
                } else {
                    break;
                }
            }
            Ok(Statement::If(IfStatement {
                condition,
                then_branch,
                then_line: None,
            }))
        }
    }

    fn parse_goto(&mut self) -> Result<Statement, String> {
        self.consume_word("GOTO");
        let line_num = self.parse_number()? as u32;
        Ok(Statement::Goto(line_num))
    }

    fn parse_gosub(&mut self) -> Result<Statement, String> {
        self.consume_word("GOSUB");
        let line_num = self.parse_number()? as u32;
        Ok(Statement::Gosub(line_num))
    }

    fn parse_for(&mut self) -> Result<Statement, String> {
        self.consume_word("FOR");
        let variable = self.parse_identifier()?;
        self.skip_whitespace();
        self.expect('=')?;
        let start = self.parse_expression()?;
        self.skip_whitespace();
        self.consume_word("TO");
        let end = self.parse_expression()?;

        let mut step = None;
        self.skip_whitespace();
        if self.peek_word() == "STEP" {
            self.consume_word("STEP");
            step = Some(self.parse_expression()?);
        }

        Ok(Statement::For(ForStatement {
            variable,
            start,
            end,
            step,
        }))
    }

    fn parse_next(&mut self) -> Result<Statement, String> {
        self.consume_word("NEXT");
        self.skip_whitespace();

        let variable = if self.is_at_end() || self.peek() == Some(':') {
            None
        } else {
            Some(self.parse_identifier()?)
        };

        Ok(Statement::Next(variable))
    }

    fn parse_dim(&mut self) -> Result<Statement, String> {
        self.consume_word("DIM");
        let mut declarations = Vec::new();

        loop {
            self.skip_whitespace();
            let name = self.parse_identifier()?;
            self.expect('(')?;

            let mut dimensions = Vec::new();
            loop {
                dimensions.push(self.parse_expression()?);
                self.skip_whitespace();
                if self.peek() == Some(',') {
                    self.advance();
                } else {
                    break;
                }
            }

            self.expect(')')?;
            declarations.push(DimDeclaration { name, dimensions });

            self.skip_whitespace();
            if self.peek() == Some(',') {
                self.advance();
            } else {
                break;
            }
        }

        Ok(Statement::Dim(declarations))
    }

    fn parse_data(&mut self) -> Result<Statement, String> {
        self.consume_word("DATA");
        let mut values = Vec::new();

        loop {
            self.skip_whitespace();
            // Read until comma or end of line
            let mut value = String::new();
            while !self.is_at_end() && self.peek() != Some(',') && self.peek() != Some(':') {
                value.push(self.advance().unwrap());
            }
            values.push(value.trim().to_string());

            self.skip_whitespace();
            if self.peek() == Some(',') {
                self.advance();
            } else {
                break;
            }
        }

        Ok(Statement::Data(values))
    }

    fn parse_read(&mut self) -> Result<Statement, String> {
        self.consume_word("READ");
        let mut variables = Vec::new();

        loop {
            self.skip_whitespace();
            variables.push(self.parse_identifier()?);
            self.skip_whitespace();
            if self.peek() == Some(',') {
                self.advance();
            } else {
                break;
            }
        }

        Ok(Statement::Read(variables))
    }

    fn parse_poke(&mut self) -> Result<Statement, String> {
        self.consume_word("POKE");
        let address = self.parse_expression()?;
        self.skip_whitespace();
        self.expect(',')?;
        let value = self.parse_expression()?;
        Ok(Statement::Poke(Box::new(address), Box::new(value)))
    }

    fn parse_rem(&mut self) -> Result<Statement, String> {
        self.consume_word("REM");
        let rest = self.input[self.pos..].to_string();
        self.pos = self.input.len();
        Ok(Statement::Rem(rest))
    }

    fn parse_let(&mut self) -> Result<Statement, String> {
        // LET is optional in BASIC
        if self.peek_word() == "LET" {
            self.consume_word("LET");
        }

        let variable = self.parse_identifier()?;
        let mut index = None;

        // Check for array access
        self.skip_whitespace();
        if self.peek() == Some('(') {
            self.advance();
            let mut indices = Vec::new();
            loop {
                indices.push(self.parse_expression()?);
                self.skip_whitespace();
                if self.peek() == Some(',') {
                    self.advance();
                } else {
                    break;
                }
            }
            self.expect(')')?;
            index = Some(indices);
        }

        self.skip_whitespace();
        self.expect('=')?;
        let value = self.parse_expression()?;

        Ok(Statement::Let(LetStatement {
            variable,
            index,
            value,
        }))
    }

    fn parse_expression(&mut self) -> Result<Expr, String> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;

        while self.peek_word() == "OR" {
            self.consume_word("OR");
            let right = self.parse_and()?;
            left = Expr::BinaryOp(Box::new(left), BinOp::Or, Box::new(right));
        }

        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_comparison()?;

        while self.peek_word() == "AND" {
            self.consume_word("AND");
            let right = self.parse_comparison()?;
            left = Expr::BinaryOp(Box::new(left), BinOp::And, Box::new(right));
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_addition()?;

        self.skip_whitespace();
        if let Some(op) = self.match_comparison_op() {
            let right = self.parse_addition()?;
            left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
        }

        Ok(left)
    }

    fn match_comparison_op(&mut self) -> Option<BinOp> {
        self.skip_whitespace();

        if self.peek() == Some('<') {
            self.advance();
            if self.peek() == Some('>') {
                self.advance();
                return Some(BinOp::Ne);
            } else if self.peek() == Some('=') {
                self.advance();
                return Some(BinOp::Le);
            }
            return Some(BinOp::Lt);
        }

        if self.peek() == Some('>') {
            self.advance();
            if self.peek() == Some('=') {
                self.advance();
                return Some(BinOp::Ge);
            }
            return Some(BinOp::Gt);
        }

        if self.peek() == Some('=') {
            self.advance();
            return Some(BinOp::Eq);
        }

        None
    }

    fn parse_addition(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_multiplication()?;

        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('+') => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    left = Expr::BinaryOp(Box::new(left), BinOp::Add, Box::new(right));
                }
                Some('-') => {
                    self.advance();
                    let right = self.parse_multiplication()?;
                    left = Expr::BinaryOp(Box::new(left), BinOp::Sub, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;

        loop {
            self.skip_whitespace();
            match self.peek() {
                Some('*') => {
                    self.advance();
                    let right = self.parse_power()?;
                    left = Expr::BinaryOp(Box::new(left), BinOp::Mul, Box::new(right));
                }
                Some('/') => {
                    self.advance();
                    let right = self.parse_power()?;
                    left = Expr::BinaryOp(Box::new(left), BinOp::Div, Box::new(right));
                }
                _ => break,
            }
        }

        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;

        self.skip_whitespace();
        if self.peek() == Some('^') {
            self.advance();
            let right = self.parse_power()?; // Right associative
            left = Expr::BinaryOp(Box::new(left), BinOp::Pow, Box::new(right));
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();

        if self.peek() == Some('-') {
            self.advance();
            return Ok(Expr::UnaryOp(UnOp::Neg, Box::new(self.parse_unary()?)));
        }

        if self.peek_word() == "NOT" {
            self.consume_word("NOT");
            return Ok(Expr::UnaryOp(UnOp::Not, Box::new(self.parse_unary()?)));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        self.skip_whitespace();

        // Parenthesized expression
        if self.peek() == Some('(') {
            self.advance();
            let expr = self.parse_expression()?;
            self.expect(')')?;
            return Ok(expr);
        }

        // String literal
        if self.peek() == Some('"') {
            return Ok(Expr::String(self.parse_string_literal()?));
        }

        // Number (including .5 format)
        if self.peek().map(|c| c.is_ascii_digit() || c == '.').unwrap_or(false) {
            return Ok(Expr::Number(self.parse_number()?));
        }

        // Function call or variable
        let name = self.parse_identifier()?;
        self.skip_whitespace();

        if self.peek() == Some('(') {
            // Function call or array access
            self.advance();
            let mut args = Vec::new();

            if self.peek() != Some(')') {
                loop {
                    args.push(self.parse_expression()?);
                    self.skip_whitespace();
                    if self.peek() == Some(',') {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }

            self.expect(')')?;

            // Check if it's a function
            if is_function(&name) {
                Ok(Expr::FunctionCall(name, args))
            } else {
                Ok(Expr::ArrayAccess(name, args))
            }
        } else {
            Ok(Expr::Variable(name))
        }
    }

    fn parse_identifier(&mut self) -> Result<String, String> {
        self.skip_whitespace();
        let start = self.pos;

        if let Some(c) = self.peek() {
            if !c.is_alphabetic() {
                return Err(format!("Expected identifier, found '{}'", c));
            }
        } else {
            return Err("Unexpected end of input".to_string());
        }

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '$' || c == '%' {
                self.advance();
            } else {
                break;
            }
        }

        Ok(self.input[start..self.pos].to_string())
    }

    fn parse_number(&mut self) -> Result<f64, String> {
        self.skip_whitespace();
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                self.advance();
            } else {
                break;
            }
        }

        self.input[start..self.pos]
            .parse()
            .map_err(|_| "Invalid number".to_string())
    }

    fn parse_string_literal(&mut self) -> Result<String, String> {
        self.expect('"')?;
        let start = self.pos;

        while let Some(c) = self.peek() {
            if c == '"' {
                break;
            }
            self.advance();
        }

        let result = self.input[start..self.pos].to_string();
        self.expect('"')?;
        Ok(result)
    }

    fn peek_word(&self) -> String {
        let mut parser = self.clone();
        parser.skip_whitespace();

        let start = parser.pos;
        while let Some(c) = parser.peek() {
            if c.is_alphabetic() {
                parser.advance();
            } else {
                break;
            }
        }

        parser.input[start..parser.pos].to_string()
    }

    fn consume_word(&mut self, word: &str) {
        self.skip_whitespace();
        let word_upper = word.to_uppercase();

        for expected_char in word_upper.chars() {
            if let Some(c) = self.advance() {
                if c != expected_char {
                    let context_start = self.pos.saturating_sub(20);
                    let context_end = (self.pos + 20).min(self.input.len());
                    let context = &self.input[context_start..context_end];
                    panic!(
                        "Expected '{}', found '{}' while consuming '{}'. Context: ...{}...",
                        expected_char, c, word, context
                    );
                }
            } else {
                panic!("Expected '{}', found end of input while consuming '{}'", expected_char, word);
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        c
    }

    fn expect(&mut self, expected: char) -> Result<(), String> {
        self.skip_whitespace();
        if let Some(c) = self.advance() {
            if c == expected {
                Ok(())
            } else {
                Err(format!("Expected '{}', found '{}'", expected, c))
            }
        } else {
            Err(format!("Expected '{}', found end of input", expected))
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.input.len()
    }
}

impl Clone for Parser {
    fn clone(&self) -> Self {
        Parser {
            input: self.input.clone(),
            pos: self.pos,
        }
    }
}

fn is_function(name: &str) -> bool {
    matches!(
        name,
        "INT" | "RND" | "CHR$" | "ASC" | "VAL" | "STR$" | "MID$" | "LEN" | "LEFT$" | "RIGHT$"
    )
}
