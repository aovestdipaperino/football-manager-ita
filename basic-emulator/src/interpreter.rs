use crate::parser::{BinOp, DimDeclaration, Expr, ForStatement, IfStatement, LetStatement, Parser, PrintItem, PrintStatement, Program, Statement, UnOp};
use crate::screen::Screen;
use crate::value::Value;
use rand::Rng;
use std::collections::HashMap;

struct ForLoop {
    variable: String,
    end_value: f64,
    step: f64,
    line_num: u32,
    statement_idx: usize,
}

pub struct Interpreter {
    program: Program,
    variables: HashMap<String, Value>,
    arrays: HashMap<String, Vec<Value>>,
    array_dimensions: HashMap<String, Vec<usize>>,
    gosub_stack: Vec<(u32, usize)>,
    for_stack: Vec<ForLoop>,
    data_values: Vec<String>,
    data_pointer: usize,
    current_line: Option<u32>,
    statement_idx: usize,
    screen: Screen,
    input_buffer: String,
    waiting_for_input: bool,
    input_variable: Option<String>,
    rng: rand::rngs::ThreadRng,
    ended: bool,
}

impl Interpreter {
    pub fn new(screen: Screen) -> Self {
        Interpreter {
            program: Program::new(),
            variables: HashMap::new(),
            arrays: HashMap::new(),
            array_dimensions: HashMap::new(),
            gosub_stack: Vec::new(),
            for_stack: Vec::new(),
            data_values: Vec::new(),
            data_pointer: 0,
            current_line: None,
            statement_idx: 0,
            screen,
            input_buffer: String::new(),
            waiting_for_input: false,
            input_variable: None,
            rng: rand::thread_rng(),
            ended: false,
        }
    }

    pub fn load_program(&mut self, source: &str) -> Result<(), String> {
        self.program = Parser::parse_program(source)?;

        // Collect DATA values
        for (_, statements) in &self.program.lines {
            for statement in statements {
                if let Statement::Data(values) = statement {
                    self.data_values.extend(values.clone());
                }
            }
        }

        // Start at first line
        if let Some(&first_line) = self.program.lines.keys().next() {
            self.current_line = Some(first_line);
        }

        Ok(())
    }

    pub fn is_waiting_for_input(&self) -> bool {
        self.waiting_for_input
    }

    pub fn handle_input_char(&mut self, c: char) {
        self.input_buffer.push(c);
        let bytes = c.to_string().into_bytes();
        self.screen.print(&bytes);
    }

    pub fn handle_input_backspace(&mut self) {
        if !self.input_buffer.is_empty() {
            self.input_buffer.pop();
            // TODO: Handle backspace in screen
        }
    }

    pub fn handle_input_enter(&mut self) {
        if let Some(var_name) = &self.input_variable {
            let value = if var_name.ends_with('$') {
                Value::String(self.input_buffer.clone().into_bytes())
            } else {
                Value::Number(self.input_buffer.parse().unwrap_or(0.0))
            };

            self.variables.insert(var_name.clone(), value);
            self.input_buffer.clear();
            self.waiting_for_input = false;
            self.input_variable = None;
            self.screen.println(b"");
        }
    }

    pub fn step(&mut self) -> Result<bool, String> {
        if self.ended {
            return Ok(false);
        }

        // If waiting for input, don't advance
        if self.waiting_for_input {
            return Ok(true);
        }

        let current_line = match self.current_line {
            Some(line) => line,
            None => return Ok(false), // Program ended
        };

        let statements = self.program.lines.get(&current_line)
            .ok_or_else(|| format!("Line {} not found", current_line))?
            .clone();

        if self.statement_idx >= statements.len() {
            // Move to next line
            self.advance_to_next_line();
            return Ok(true);
        }

        let statement = statements[self.statement_idx].clone();
        self.statement_idx += 1;

        self.execute_statement(&statement)?;

        Ok(true)
    }

    fn execute_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Print(stmt) => self.exec_print(stmt),
            Statement::Input(stmt) => self.exec_input(stmt),
            Statement::Get(var) => self.exec_get(var),
            Statement::Let(stmt) => self.exec_let(stmt),
            Statement::If(stmt) => self.exec_if(stmt),
            Statement::Goto(line) => self.exec_goto(*line),
            Statement::Gosub(line) => self.exec_gosub(*line),
            Statement::Return => self.exec_return(),
            Statement::Run => self.exec_run(),
            Statement::For(stmt) => self.exec_for(stmt),
            Statement::Next(var) => self.exec_next(var),
            Statement::Dim(decls) => self.exec_dim(decls),
            Statement::Data(_) => Ok(()), // DATA is processed at load time
            Statement::Read(vars) => self.exec_read(vars),
            Statement::Poke(addr, val) => self.exec_poke(addr, val),
            Statement::End => {
                self.ended = true;
                Ok(())
            }
            Statement::Rem(_) => Ok(()), // Comments are ignored
        }
    }

    fn exec_print(&mut self, stmt: &PrintStatement) -> Result<(), String> {
        let mut column = 0;

        for item in &stmt.items {
            match item {
                PrintItem::Expr(expr) => {
                    let value = self.eval_expr(expr)?;
                    let bytes = value.as_petscii_bytes();
                    self.screen.print(&bytes);

                    // C64 adds a trailing space after numbers (but not strings)
                    let add_space = matches!(value, Value::Number(_));
                    if add_space {
                        self.screen.print(b" ");
                        column += bytes.len() + 1;
                    } else {
                        column += bytes.len();
                    }
                }
                PrintItem::Tab(expr) => {
                    let col = self.eval_expr(expr)?.as_int()? as usize;
                    self.screen.tab_to(col);
                    column = col;
                }
                PrintItem::Comma => {
                    // Tab to next zone (every 10 columns)
                    let next_zone = ((column / 10) + 1) * 10;
                    self.screen.tab_to(next_zone);
                    column = next_zone;
                }
                PrintItem::Spc(expr) => {
                    let count = self.eval_expr(expr)?.as_int()? as usize;
                    for _ in 0..count {
                        self.screen.print(b" ");
                    }
                    column += count;
                }
            }
        }

        if !stmt.trailing_semicolon {
            self.screen.println(b"");
        }

        Ok(())
    }

    fn exec_input(&mut self, stmt: &crate::parser::InputStatement) -> Result<(), String> {
        // If we're already waiting for input, don't do anything
        // (this shouldn't happen with the step() check, but defensive)
        if self.waiting_for_input {
            return Ok(());
        }

        if let Some(prompt) = &stmt.prompt {
            self.screen.print(prompt);
        } else {
            self.screen.print(b"? ");
        }

        self.waiting_for_input = true;
        self.input_variable = Some(stmt.variable.clone());
        // Don't decrement statement_idx - step() will handle waiting

        Ok(())
    }

    fn exec_get(&mut self, variable: &str) -> Result<(), String> {
        // GET reads a single character without waiting for Enter
        // For our TUI implementation, we'll treat it like INPUT for simplicity
        if self.waiting_for_input {
            return Ok(());
        }

        // On C64, GET doesn't wait - it reads immediately or gets empty string
        // For our TUI implementation, we'll wait for input like INPUT
        self.waiting_for_input = true;
        self.input_variable = Some(variable.to_string());
        // Don't decrement statement_idx - step() will handle waiting

        Ok(())
    }

    fn exec_let(&mut self, stmt: &LetStatement) -> Result<(), String> {
        let value = self.eval_expr(&stmt.value)?;

        if let Some(indices) = &stmt.index {
            // Array assignment
            let index_values: Result<Vec<usize>, String> = indices
                .iter()
                .map(|e| Ok(self.eval_expr(e)?.as_int()? as usize))
                .collect();
            let index_values = index_values?;

            // C64 BASIC auto-dimensions arrays on first use (0-10)
            if !self.array_dimensions.contains_key(&stmt.variable) {
                self.auto_dimension_array(&stmt.variable, indices.len())?;
            }

            let dims = self.array_dimensions.get(&stmt.variable)
                .ok_or_else(|| format!("Array {} not dimensioned", stmt.variable))?.clone();

            let flat_index = self.calculate_flat_index(&index_values, &dims)?;

            let array = self.arrays.get_mut(&stmt.variable)
                .ok_or_else(|| format!("Array {} not dimensioned", stmt.variable))?;

            if flat_index >= array.len() {
                return Err(format!("Array index out of bounds"));
            }

            array[flat_index] = value;
        } else {
            // Simple variable assignment
            self.variables.insert(stmt.variable.clone(), value);
        }

        Ok(())
    }

    fn exec_if(&mut self, stmt: &IfStatement) -> Result<(), String> {
        let condition = self.eval_expr(&stmt.condition)?;

        if condition.is_truthy() {
            if let Some(line) = stmt.then_line {
                self.exec_goto(line)?;
            } else {
                for statement in &stmt.then_branch {
                    self.execute_statement(statement)?;
                }
            }
        }

        Ok(())
    }

    fn exec_goto(&mut self, line: u32) -> Result<(), String> {
        self.current_line = Some(line);
        self.statement_idx = 0;
        Ok(())
    }

    fn exec_gosub(&mut self, line: u32) -> Result<(), String> {
        self.gosub_stack.push((self.current_line.unwrap(), self.statement_idx));
        self.exec_goto(line)
    }

    fn exec_return(&mut self) -> Result<(), String> {
        let (line, idx) = self.gosub_stack.pop()
            .ok_or_else(|| "RETURN without GOSUB".to_string())?;

        self.current_line = Some(line);
        self.statement_idx = idx;
        Ok(())
    }

    fn exec_run(&mut self) -> Result<(), String> {
        // RUN restarts the program from the beginning
        // Clear all variables and stacks
        self.variables.clear();
        self.arrays.clear();
        self.array_dimensions.clear();
        self.gosub_stack.clear();
        self.for_stack.clear();
        self.data_pointer = 0;

        // Start at first line
        if let Some(&first_line) = self.program.lines.keys().next() {
            self.current_line = Some(first_line);
            self.statement_idx = 0;
        }

        Ok(())
    }

    fn exec_for(&mut self, stmt: &ForStatement) -> Result<(), String> {
        let start = self.eval_expr(&stmt.start)?.as_number()?;
        let end = self.eval_expr(&stmt.end)?.as_number()?;
        let step = if let Some(step_expr) = &stmt.step {
            self.eval_expr(step_expr)?.as_number()?
        } else {
            1.0
        };

        self.variables.insert(stmt.variable.clone(), Value::Number(start));

        self.for_stack.push(ForLoop {
            variable: stmt.variable.clone(),
            end_value: end,
            step,
            line_num: self.current_line.unwrap(),
            statement_idx: self.statement_idx,
        });

        Ok(())
    }

    fn exec_next(&mut self, _var: &Option<String>) -> Result<(), String> {
        let for_loop = self.for_stack.last_mut()
            .ok_or_else(|| "NEXT without FOR".to_string())?;

        // Increment loop variable
        let current_value = self.variables.get(&for_loop.variable)
            .ok_or_else(|| format!("Loop variable {} not found", for_loop.variable))?
            .as_number()?;

        let new_value = current_value + for_loop.step;

        // Check if loop should continue
        let should_continue = if for_loop.step > 0.0 {
            new_value <= for_loop.end_value
        } else {
            new_value >= for_loop.end_value
        };

        if should_continue {
            self.variables.insert(for_loop.variable.clone(), Value::Number(new_value));
            self.current_line = Some(for_loop.line_num);
            self.statement_idx = for_loop.statement_idx;
        } else {
            self.for_stack.pop();
        }

        Ok(())
    }

    fn exec_dim(&mut self, decls: &[DimDeclaration]) -> Result<(), String> {
        for decl in decls {
            let mut dims = Vec::new();
            let mut total_size = 1;

            for dim_expr in &decl.dimensions {
                let size = self.eval_expr(dim_expr)?.as_int()? as usize + 1; // +1 because BASIC arrays are 0-indexed
                dims.push(size);
                total_size *= size;
            }

            let default_value = if decl.name.ends_with('$') {
                Value::String(Vec::new())
            } else {
                Value::Number(0.0)
            };

            self.arrays.insert(decl.name.clone(), vec![default_value; total_size]);
            self.array_dimensions.insert(decl.name.clone(), dims);
        }

        Ok(())
    }

    fn exec_read(&mut self, targets: &[crate::parser::ReadTarget]) -> Result<(), String> {
        for target in targets {
            if self.data_pointer >= self.data_values.len() {
                return Err("OUT OF DATA".to_string());
            }

            let data_str = &self.data_values[self.data_pointer];
            self.data_pointer += 1;

            let value = if target.variable.ends_with('$') {
                Value::String(data_str.clone().into_bytes())
            } else {
                Value::Number(data_str.parse().unwrap_or(0.0))
            };

            // Handle array or simple variable assignment
            if let Some(indices) = &target.index {
                // Array assignment
                let index_values: Result<Vec<usize>, String> = indices
                    .iter()
                    .map(|e| Ok(self.eval_expr(e)?.as_int()? as usize))
                    .collect();
                let index_values = index_values?;

                // C64 BASIC auto-dimensions arrays on first use (0-10)
                if !self.array_dimensions.contains_key(&target.variable) {
                    self.auto_dimension_array(&target.variable, indices.len())?;
                }

                let dims = self.array_dimensions.get(&target.variable)
                    .ok_or_else(|| format!("Array {} not dimensioned", target.variable))?.clone();

                let flat_index = self.calculate_flat_index(&index_values, &dims)?;

                let array = self.arrays.get_mut(&target.variable)
                    .ok_or_else(|| format!("Array {} not dimensioned", target.variable))?;

                if flat_index >= array.len() {
                    return Err(format!("Array index out of bounds"));
                }

                array[flat_index] = value;
            } else {
                // Simple variable assignment
                self.variables.insert(target.variable.clone(), value);
            }
        }

        Ok(())
    }

    fn exec_poke(&mut self, addr_expr: &Expr, val_expr: &Expr) -> Result<(), String> {
        let addr = self.eval_expr(addr_expr)?.as_int()? as u32;
        let value = self.eval_expr(val_expr)?.as_int()? as u8;

        match addr {
            650 => {}, // Keyboard repeat - ignore
            1690 => {}, // Unknown - ignore
            53272 => {}, // Character set control - ignore
            53280 => self.screen.set_border_color(value),
            53281 => self.screen.set_background_color(value),
            _ => {}, // Ignore other POKE addresses
        }

        Ok(())
    }

    fn advance_to_next_line(&mut self) {
        if let Some(current) = self.current_line {
            self.current_line = self.program.lines
                .keys()
                .find(|&&k| k > current)
                .copied();
            self.statement_idx = 0;
        }
    }

    fn auto_dimension_array(&mut self, name: &str, num_dimensions: usize) -> Result<(), String> {
        // C64 BASIC auto-dimensions arrays to (0-10) on first use
        // This gives 11 elements per dimension
        let default_size = 11;

        let dims = vec![default_size; num_dimensions];
        let total_size = dims.iter().product();

        // Create array filled with default values (0 for numbers, "" for strings)
        let default_value = if name.ends_with('$') {
            Value::String(Vec::new())
        } else {
            Value::Number(0.0)
        };

        let array = vec![default_value; total_size];

        self.arrays.insert(name.to_string(), array);
        self.array_dimensions.insert(name.to_string(), dims);

        Ok(())
    }

    fn calculate_flat_index(&self, indices: &[usize], dims: &[usize]) -> Result<usize, String> {
        if indices.len() != dims.len() {
            return Err("Array dimension mismatch".to_string());
        }

        let mut flat_index = 0;
        let mut multiplier = 1;

        for i in (0..indices.len()).rev() {
            if indices[i] >= dims[i] {
                return Err("Array index out of bounds".to_string());
            }
            flat_index += indices[i] * multiplier;
            multiplier *= dims[i];
        }

        Ok(flat_index)
    }

    fn eval_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Variable(name) => {
                self.variables.get(name)
                    .cloned()
                    .or_else(|| {
                        // Return default value if not found
                        if name.ends_with('$') {
                            Some(Value::String(Vec::new()))
                        } else {
                            Some(Value::Number(0.0))
                        }
                    })
                    .ok_or_else(|| format!("Variable {} not found", name))
            }
            Expr::ArrayAccess(name, indices) => {
                let index_values: Result<Vec<usize>, String> = indices
                    .iter()
                    .map(|e| Ok(self.eval_expr(e)?.as_int()? as usize))
                    .collect();
                let index_values = index_values?;

                // C64 BASIC auto-dimensions arrays on first use (0-10)
                if !self.array_dimensions.contains_key(name) {
                    self.auto_dimension_array(name, indices.len())?;
                }

                let array = self.arrays.get(name)
                    .ok_or_else(|| format!("Array {} not dimensioned", name))?;

                let dims = self.array_dimensions.get(name)
                    .ok_or_else(|| format!("Array {} not dimensioned", name))?;

                let flat_index = self.calculate_flat_index(&index_values, dims)?;

                array.get(flat_index)
                    .cloned()
                    .ok_or_else(|| "Array index out of bounds".to_string())
            }
            Expr::BinaryOp(left, op, right) => {
                let left_val = self.eval_expr(left)?;
                let right_val = self.eval_expr(right)?;
                self.eval_binop(&left_val, *op, &right_val)
            }
            Expr::UnaryOp(op, expr) => {
                let val = self.eval_expr(expr)?;
                self.eval_unop(*op, &val)
            }
            Expr::FunctionCall(name, args) => self.eval_function(name, args),
        }
    }

    fn eval_binop(&mut self, left: &Value, op: BinOp, right: &Value) -> Result<Value, String> {
        match op {
            BinOp::Add => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => {
                    let mut result = a.clone();
                    result.extend_from_slice(b);
                    Ok(Value::String(result))
                }
                _ => Err("Type mismatch in addition".to_string()),
            },
            BinOp::Sub => Ok(Value::Number(left.as_number()? - right.as_number()?)),
            BinOp::Mul => Ok(Value::Number(left.as_number()? * right.as_number()?)),
            BinOp::Div => Ok(Value::Number(left.as_number()? / right.as_number()?)),
            BinOp::Pow => Ok(Value::Number(left.as_number()?.powf(right.as_number()?))),
            BinOp::Eq => Ok(Value::Number(if left == right { -1.0 } else { 0.0 })),
            BinOp::Ne => Ok(Value::Number(if left != right { -1.0 } else { 0.0 })),
            BinOp::Lt => Ok(Value::Number(if left.as_number()? < right.as_number()? { -1.0 } else { 0.0 })),
            BinOp::Le => Ok(Value::Number(if left.as_number()? <= right.as_number()? { -1.0 } else { 0.0 })),
            BinOp::Gt => Ok(Value::Number(if left.as_number()? > right.as_number()? { -1.0 } else { 0.0 })),
            BinOp::Ge => Ok(Value::Number(if left.as_number()? >= right.as_number()? { -1.0 } else { 0.0 })),
            BinOp::And => {
                let a = left.as_int()? as i64;
                let b = right.as_int()? as i64;
                Ok(Value::Number((a & b) as f64))
            }
            BinOp::Or => {
                let a = left.as_int()? as i64;
                let b = right.as_int()? as i64;
                Ok(Value::Number((a | b) as f64))
            }
        }
    }

    fn eval_unop(&mut self, op: UnOp, val: &Value) -> Result<Value, String> {
        match op {
            UnOp::Neg => Ok(Value::Number(-val.as_number()?)),
            UnOp::Not => Ok(Value::Number(if val.is_truthy() { 0.0 } else { -1.0 })),
        }
    }

    fn eval_function(&mut self, name: &str, args: &[Expr]) -> Result<Value, String> {
        match name {
            "INT" => {
                if args.len() != 1 {
                    return Err("INT requires 1 argument".to_string());
                }
                Ok(Value::Number(self.eval_expr(&args[0])?.as_number()?.floor()))
            }
            "RND" => {
                // C64 RND ignores argument, just returns random 0-1
                Ok(Value::Number(self.rng.gen()))
            }
            "CHR$" => {
                if args.len() != 1 {
                    return Err("CHR$ requires 1 argument".to_string());
                }
                let code = self.eval_expr(&args[0])?.as_int()? as u8;
                Ok(Value::String(vec![code]))
            }
            "ASC" => {
                if args.len() != 1 {
                    return Err("ASC requires 1 argument".to_string());
                }
                let s = self.eval_expr(&args[0])?.as_string_bytes()?;
                Ok(Value::Number(s.get(0).copied().unwrap_or(0) as f64))
            }
            "VAL" => {
                if args.len() != 1 {
                    return Err("VAL requires 1 argument".to_string());
                }
                let s = self.eval_expr(&args[0])?.as_string_bytes()?;
                let s_str = String::from_utf8_lossy(&s);
                Ok(Value::Number(s_str.trim().parse().unwrap_or(0.0)))
            }
            "STR$" => {
                if args.len() != 1 {
                    return Err("STR$ requires 1 argument".to_string());
                }
                let n = self.eval_expr(&args[0])?.as_number()?;
                let formatted = format!(" {}", n);
                Ok(Value::String(formatted.into_bytes()))
            }
            "MID$" => {
                if args.len() < 2 || args.len() > 3 {
                    return Err("MID$ requires 2 or 3 arguments".to_string());
                }
                let s = self.eval_expr(&args[0])?.as_string_bytes()?;
                let start = (self.eval_expr(&args[1])?.as_int()? - 1).max(0) as usize;

                let result = if args.len() == 3 {
                    let length = self.eval_expr(&args[2])?.as_int()? as usize;
                    s.iter().skip(start).take(length).copied().collect()
                } else {
                    s.iter().skip(start).copied().collect()
                };

                Ok(Value::String(result))
            }
            "LEN" => {
                if args.len() != 1 {
                    return Err("LEN requires 1 argument".to_string());
                }
                let s = self.eval_expr(&args[0])?.as_string_bytes()?;
                Ok(Value::Number(s.len() as f64))
            }
            "LEFT$" => {
                if args.len() != 2 {
                    return Err("LEFT$ requires 2 arguments".to_string());
                }
                let s = self.eval_expr(&args[0])?.as_string_bytes()?;
                let length = self.eval_expr(&args[1])?.as_int()? as usize;
                let result: Vec<u8> = s.iter().take(length).copied().collect();
                Ok(Value::String(result))
            }
            "RIGHT$" => {
                if args.len() != 2 {
                    return Err("RIGHT$ requires 2 arguments".to_string());
                }
                let s = self.eval_expr(&args[0])?.as_string_bytes()?;
                let length = self.eval_expr(&args[1])?.as_int()? as usize;
                let skip = s.len().saturating_sub(length);
                let result: Vec<u8> = s.iter().skip(skip).copied().collect();
                Ok(Value::String(result))
            }
            _ => Err(format!("Unknown function: {}", name)),
        }
    }
}
