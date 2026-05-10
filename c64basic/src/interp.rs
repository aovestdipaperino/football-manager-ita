//! Tree-walking BASIC interpreter over the AST from `lang.rs`.
//! Control flow uses a flat (line, stmt_index) program counter so that
//! FOR/NEXT and IF-bodies all address well-defined resume points.

use crate::lang::*;
use crate::screen::{Screen, COLS, ROWS};
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum Value {
    Num(f64),
    Str(Vec<u8>),
}

impl Value {
    pub fn to_num(&self) -> f64 {
        match self {
            Value::Num(n) => *n,
            Value::Str(s) => {
                let txt: String = s.iter().map(|&b| b as char).collect();
                txt.trim().parse::<f64>().unwrap_or(0.0)
            }
        }
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Value::Str(s) => s.clone(),
            Value::Num(n) => fmt_number(*n).into_bytes(),
        }
    }
}

/// C64-style number formatting: leading space for non-negative, integer form
/// when value has no fractional part, and a trailing space.
pub fn fmt_number(n: f64) -> String {
    let mut s = if n == n.trunc() && n.abs() < 1e16 {
        format!("{}", n as i64)
    } else {
        format!("{}", n)
    };
    if !s.starts_with('-') {
        s.insert(0, ' ');
    }
    s.push(' ');
    s
}

fn coerce(is_str: bool, v: Value) -> Value {
    match (is_str, v) {
        (true, Value::Num(n)) => Value::Str(fmt_number(n).into_bytes()),
        (false, Value::Str(s)) => {
            let txt: String = s.iter().map(|&b| b as char).collect();
            Value::Num(txt.trim().parse().unwrap_or(0.0))
        }
        (_, v) => v,
    }
}

fn compare(l: &Value, r: &Value) -> i32 {
    match (l, r) {
        (Value::Str(a), Value::Str(b)) => {
            if a < b {
                -1
            } else if a > b {
                1
            } else {
                0
            }
        }
        _ => {
            let a = l.to_num();
            let b = r.to_num();
            if a < b {
                -1
            } else if a > b {
                1
            } else {
                0
            }
        }
    }
}

struct Array {
    dims: Vec<usize>, // inclusive max index per dimension
    data: Vec<Value>,
}

impl Array {
    fn new(dims: Vec<usize>, is_str: bool) -> Self {
        let total: usize = dims.iter().map(|d| d + 1).product();
        let fill = if is_str {
            Value::Str(Vec::new())
        } else {
            Value::Num(0.0)
        };
        Array {
            dims,
            data: vec![fill; total],
        }
    }
    fn offset(&self, idx: &[usize]) -> Result<usize, String> {
        if idx.len() != self.dims.len() {
            return Err(format!(
                "array rank mismatch: expected {}, got {}",
                self.dims.len(),
                idx.len()
            ));
        }
        let mut off = 0;
        for (i, &x) in idx.iter().enumerate() {
            if x > self.dims[i] {
                return Err(format!(
                    "array index {} out of bounds (max {})",
                    x, self.dims[i]
                ));
            }
            off = off * (self.dims[i] + 1) + x;
        }
        Ok(off)
    }
    fn get(&self, idx: &[usize]) -> Result<Value, String> {
        Ok(self.data[self.offset(idx)?].clone())
    }
    fn set(&mut self, idx: &[usize], v: Value) -> Result<(), String> {
        let o = self.offset(idx)?;
        self.data[o] = v;
        Ok(())
    }
}

#[derive(Copy, Clone)]
struct Pc {
    line: u32,
    stmt: usize,
}

struct ForFrame {
    var: String,
    end: f64,
    step: f64,
    resume: Pc,
}

pub enum InputMode {
    Normal,
    AwaitingLine {
        targets: Vec<LValue>,
        buffer: Vec<u8>,
    },
}

pub struct Interp {
    prog: Program,
    pc: Pc,
    next_pc: Option<Pc>,
    vars: HashMap<String, Value>,
    arrays: HashMap<String, Array>,
    for_stack: Vec<ForFrame>,
    call_stack: Vec<Pc>,
    data: Vec<DataItem>,
    data_line_starts: Vec<(u32, usize)>, // (line, starting index in flat data)
    data_ptr: usize,
    pending_chars: Vec<u8>,
    pub halted: bool,
    pub screen: Screen,
    pub input_mode: InputMode,
    rng: rand::rngs::ThreadRng,
}

impl Interp {
    pub fn new(prog: Program, screen: Screen) -> Result<Self, String> {
        let mut data = Vec::new();
        let mut data_line_starts = Vec::new();
        for (&ln, stmts) in &prog {
            let before = data.len();
            for s in stmts {
                if let Stmt::Data(items) = s {
                    data.extend(items.iter().cloned());
                }
            }
            if data.len() != before {
                data_line_starts.push((ln, before));
            }
        }
        let first_line = *prog.keys().next().ok_or("empty program")?;
        Ok(Self {
            prog,
            pc: Pc {
                line: first_line,
                stmt: 0,
            },
            next_pc: None,
            vars: HashMap::new(),
            arrays: HashMap::new(),
            for_stack: Vec::new(),
            call_stack: Vec::new(),
            data,
            data_line_starts,
            data_ptr: 0,
            pending_chars: Vec::new(),
            halted: false,
            screen,
            input_mode: InputMode::Normal,
            rng: rand::thread_rng(),
        })
    }

    /// Current BASIC line number (for status display).
    pub fn current_line(&self) -> u32 {
        self.pc.line
    }

    /// The host calls this for each keystroke captured from crossterm.
    pub fn push_char(&mut self, b: u8) {
        match &mut self.input_mode {
            InputMode::AwaitingLine { buffer, .. } => {
                match b {
                    0x0D => {
                        // ENTER – commit the line.
                        self.screen.put_byte(0x0D);
                        let line_bytes = std::mem::take(buffer);
                        let mode = std::mem::replace(&mut self.input_mode, InputMode::Normal);
                        if let InputMode::AwaitingLine { targets, .. } = mode {
                            let _ = self.deliver_input(line_bytes, targets);
                        }
                    }
                    0x14 => {
                        if buffer.pop().is_some() {
                            self.screen.put_byte(0x14);
                        }
                    }
                    _ => {
                        buffer.push(b);
                        self.screen.put_byte(b);
                    }
                }
            }
            InputMode::Normal => {
                self.pending_chars.push(b);
                // Cap queue length – we don't need a huge backlog.
                if self.pending_chars.len() > 64 {
                    self.pending_chars.remove(0);
                }
            }
        }
    }

    /// Execute up to `budget` statements, then return so the host can render
    /// and poll the keyboard. Returns true when halted.
    pub fn run_slice(&mut self, mut budget: u32) -> Result<bool, String> {
        while budget > 0 && !self.halted && matches!(self.input_mode, InputMode::Normal) {
            let line = self.pc.line;
            let stmt_idx = self.pc.stmt;
            let stmts = match self.prog.get(&line) {
                Some(v) => v,
                None => {
                    self.halted = true;
                    return Ok(true);
                }
            };
            if stmt_idx >= stmts.len() {
                let next = self.prog.range((line + 1)..).next().map(|(k, _)| *k);
                match next {
                    Some(n) => self.pc = Pc { line: n, stmt: 0 },
                    None => {
                        self.halted = true;
                        return Ok(true);
                    }
                }
                continue;
            }
            let stmt = stmts[stmt_idx].clone();
            self.next_pc = None;
            self.exec(&stmt)?;
            if self.halted {
                return Ok(true);
            }
            match self.next_pc.take() {
                Some(pc) => self.pc = pc,
                None => self.pc.stmt += 1,
            }
            budget -= 1;
        }
        Ok(self.halted)
    }

    fn exec(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Rem | Stmt::Data(_) => Ok(()),
            Stmt::End | Stmt::Stop => {
                self.halted = true;
                Ok(())
            }
            Stmt::Run | Stmt::Clr => {
                // RUN/CLR mid-program: reset variables but keep running.
                self.vars.clear();
                self.arrays.clear();
                self.for_stack.clear();
                self.call_stack.clear();
                Ok(())
            }
            Stmt::Let(lv, e) => {
                let v = self.eval(e)?;
                self.assign(lv, v)
            }
            Stmt::Print(items, trailing_nl) => self.do_print(items, *trailing_nl),
            Stmt::Goto(n) => {
                self.next_pc = Some(Pc { line: *n, stmt: 0 });
                Ok(())
            }
            Stmt::Gosub(n) => {
                let ret = Pc {
                    line: self.pc.line,
                    stmt: self.pc.stmt + 1,
                };
                self.call_stack.push(ret);
                self.next_pc = Some(Pc { line: *n, stmt: 0 });
                Ok(())
            }
            Stmt::Return => {
                let pc = self.call_stack.pop().ok_or("RETURN without GOSUB")?;
                self.next_pc = Some(pc);
                Ok(())
            }
            Stmt::IfFalseSkip(cond, skip) => {
                let truthy = self.eval(cond)?.to_num() != 0.0;
                if !truthy {
                    self.next_pc = Some(Pc {
                        line: self.pc.line,
                        stmt: self.pc.stmt + 1 + *skip,
                    });
                }
                Ok(())
            }
            Stmt::For {
                var,
                start,
                end,
                step,
            } => {
                let s = self.eval(start)?.to_num();
                let e = self.eval(end)?.to_num();
                let st = match step {
                    Some(x) => self.eval(x)?.to_num(),
                    None => 1.0,
                };
                self.vars.insert(var.clone(), Value::Num(s));
                let resume = Pc {
                    line: self.pc.line,
                    stmt: self.pc.stmt + 1,
                };
                self.for_stack.push(ForFrame {
                    var: var.clone(),
                    end: e,
                    step: st,
                    resume,
                });
                Ok(())
            }
            Stmt::Next(names) => {
                let list: Vec<Option<String>> = if names.is_empty() {
                    vec![None]
                } else {
                    names.iter().map(|n| Some(n.clone())).collect()
                };
                for target in list {
                    let frame_idx = match &target {
                        Some(n) => self
                            .for_stack
                            .iter()
                            .rposition(|f| &f.var == n)
                            .ok_or_else(|| format!("NEXT without matching FOR for {}", n))?,
                        None => self
                            .for_stack
                            .len()
                            .checked_sub(1)
                            .ok_or("NEXT without FOR")?,
                    };
                    self.for_stack.truncate(frame_idx + 1);
                    let frame = self.for_stack.last().unwrap();
                    let var = frame.var.clone();
                    let end = frame.end;
                    let step = frame.step;
                    let resume = frame.resume;
                    let cur = self.vars.get(&var).map(|v| v.to_num()).unwrap_or(0.0);
                    let next_val = cur + step;
                    self.vars.insert(var.clone(), Value::Num(next_val));
                    let done = if step >= 0.0 {
                        next_val > end
                    } else {
                        next_val < end
                    };
                    if !done {
                        self.next_pc = Some(resume);
                        return Ok(());
                    }
                    self.for_stack.pop();
                }
                Ok(())
            }
            Stmt::Dim(dims) => {
                for (name, sizes) in dims {
                    let is_str = name.ends_with('$');
                    let mut d = Vec::with_capacity(sizes.len());
                    for s in sizes {
                        d.push(self.eval(s)?.to_num() as usize);
                    }
                    self.arrays.insert(name.clone(), Array::new(d, is_str));
                }
                Ok(())
            }
            Stmt::Read(targets) => {
                for t in targets {
                    if self.data_ptr >= self.data.len() {
                        return Err("OUT OF DATA".into());
                    }
                    let item = self.data[self.data_ptr].clone();
                    self.data_ptr += 1;
                    let is_str_target = match t {
                        LValue::Var(n) => n.ends_with('$'),
                        LValue::Index(n, _) => n.ends_with('$'),
                    };
                    let v = match item {
                        DataItem::Num(n) => {
                            if is_str_target {
                                Value::Str(fmt_number(n).trim().as_bytes().to_vec())
                            } else {
                                Value::Num(n)
                            }
                        }
                        DataItem::Str(s) => {
                            if is_str_target {
                                Value::Str(s)
                            } else {
                                let t: String = s.iter().map(|&b| b as char).collect();
                                Value::Num(t.trim().parse().unwrap_or(0.0))
                            }
                        }
                    };
                    self.assign(t, v)?;
                }
                Ok(())
            }
            Stmt::Restore(opt) => {
                self.data_ptr = match opt {
                    Some(line) => self
                        .data_line_starts
                        .iter()
                        .find(|(ln, _)| *ln >= *line)
                        .map(|(_, idx)| *idx)
                        .unwrap_or(self.data.len()),
                    None => 0,
                };
                Ok(())
            }
            Stmt::Get(target) => self.do_get(target),
            Stmt::Input { prompt, targets } => {
                if let Some(p) = prompt {
                    for &b in p {
                        self.screen.put_byte(b);
                    }
                }
                self.screen.put_byte(b'?');
                self.screen.put_byte(b' ');
                self.input_mode = InputMode::AwaitingLine {
                    targets: targets.clone(),
                    buffer: Vec::new(),
                };
                Ok(())
            }
            Stmt::On {
                value,
                is_gosub,
                targets,
            } => {
                let n = self.eval(value)?.to_num() as i64;
                if n < 1 || (n as usize) > targets.len() {
                    return Ok(());
                }
                let line = targets[n as usize - 1];
                if *is_gosub {
                    let ret = Pc {
                        line: self.pc.line,
                        stmt: self.pc.stmt + 1,
                    };
                    self.call_stack.push(ret);
                }
                self.next_pc = Some(Pc { line, stmt: 0 });
                Ok(())
            }
            Stmt::Poke(addr, val) => {
                let a = self.eval(addr)?.to_num() as u64;
                let v = self.eval(val)?.to_num() as u8;
                self.do_poke(a, v);
                Ok(())
            }
        }
    }

    fn assign(&mut self, lv: &LValue, v: Value) -> Result<(), String> {
        match lv {
            LValue::Var(name) => {
                let v = coerce(name.ends_with('$'), v);
                self.vars.insert(name.clone(), v);
                Ok(())
            }
            LValue::Index(name, idx_exprs) => {
                let mut idxs = Vec::with_capacity(idx_exprs.len());
                for e in idx_exprs {
                    idxs.push(self.eval(e)?.to_num() as usize);
                }
                let is_str = name.ends_with('$');
                let v = coerce(is_str, v);
                let arr = self
                    .arrays
                    .entry(name.clone())
                    .or_insert_with(|| Array::new(vec![10; idxs.len()], is_str));
                arr.set(&idxs, v)
            }
        }
    }

    fn eval(&mut self, e: &Expr) -> Result<Value, String> {
        Ok(match e {
            Expr::Num(n) => Value::Num(*n),
            Expr::Str(s) => Value::Str(s.clone()),
            Expr::Var(name) => {
                if let Some(v) = self.vars.get(name) {
                    v.clone()
                } else if name.ends_with('$') {
                    Value::Str(Vec::new())
                } else {
                    Value::Num(0.0)
                }
            }
            Expr::Index(name, idx_exprs) => {
                let mut idxs = Vec::with_capacity(idx_exprs.len());
                for ex in idx_exprs {
                    idxs.push(self.eval(ex)?.to_num() as usize);
                }
                let is_str = name.ends_with('$');
                let arr = self
                    .arrays
                    .entry(name.clone())
                    .or_insert_with(|| Array::new(vec![10; idxs.len()], is_str));
                arr.get(&idxs)?
            }
            Expr::Unary(op, x) => {
                let v = self.eval(x)?;
                match op {
                    UnOp::Neg => Value::Num(-v.to_num()),
                    UnOp::Not => Value::Num(if v.to_num() == 0.0 { -1.0 } else { 0.0 }),
                }
            }
            Expr::Binary(op, l, r) => {
                let lv = self.eval(l)?;
                let rv = self.eval(r)?;
                self.apply_binop(*op, lv, rv)?
            }
            Expr::Call(f, args) => self.call_builtin(*f, args)?,
        })
    }

    fn apply_binop(&self, op: BinOp, l: Value, r: Value) -> Result<Value, String> {
        use BinOp::*;
        match op {
            Add => match (&l, &r) {
                (Value::Str(a), Value::Str(b)) => {
                    let mut c = a.clone();
                    c.extend_from_slice(b);
                    Ok(Value::Str(c))
                }
                _ => Ok(Value::Num(l.to_num() + r.to_num())),
            },
            Sub => Ok(Value::Num(l.to_num() - r.to_num())),
            Mul => Ok(Value::Num(l.to_num() * r.to_num())),
            Div => {
                let d = r.to_num();
                if d == 0.0 {
                    return Err("division by zero".into());
                }
                Ok(Value::Num(l.to_num() / d))
            }
            Pow => Ok(Value::Num(l.to_num().powf(r.to_num()))),
            And => {
                let a = l.to_num() as i64;
                let b = r.to_num() as i64;
                Ok(Value::Num((a & b) as f64))
            }
            Or => {
                let a = l.to_num() as i64;
                let b = r.to_num() as i64;
                Ok(Value::Num((a | b) as f64))
            }
            Eq | Ne | Lt | Gt | Le | Ge => {
                let cmp = compare(&l, &r);
                let res = match op {
                    Eq => cmp == 0,
                    Ne => cmp != 0,
                    Lt => cmp < 0,
                    Gt => cmp > 0,
                    Le => cmp <= 0,
                    Ge => cmp >= 0,
                    _ => unreachable!(),
                };
                Ok(Value::Num(if res { -1.0 } else { 0.0 }))
            }
        }
    }

    fn call_builtin(&mut self, f: BuiltinFn, args: &[Expr]) -> Result<Value, String> {
        use BuiltinFn::*;
        let need = |n: usize| -> Result<(), String> {
            if args.len() < n {
                Err(format!("{:?} needs {} args", f, n))
            } else {
                Ok(())
            }
        };
        match f {
            Int => {
                need(1)?;
                let x = self.eval(&args[0])?.to_num();
                Ok(Value::Num(x.floor()))
            }
            Abs => {
                need(1)?;
                Ok(Value::Num(self.eval(&args[0])?.to_num().abs()))
            }
            Sgn => {
                need(1)?;
                let x = self.eval(&args[0])?.to_num();
                Ok(Value::Num(if x > 0.0 {
                    1.0
                } else if x < 0.0 {
                    -1.0
                } else {
                    0.0
                }))
            }
            Sqr => {
                need(1)?;
                Ok(Value::Num(self.eval(&args[0])?.to_num().sqrt()))
            }
            Rnd => {
                need(1)?;
                let _ = self.eval(&args[0])?.to_num();
                Ok(Value::Num(self.rng.gen::<f64>()))
            }
            Chr => {
                need(1)?;
                let n = self.eval(&args[0])?.to_num() as i64 & 0xFF;
                Ok(Value::Str(vec![n as u8]))
            }
            Asc => {
                need(1)?;
                let s = self.eval(&args[0])?.to_bytes();
                Ok(Value::Num(s.first().copied().unwrap_or(0) as f64))
            }
            Len => {
                need(1)?;
                let s = self.eval(&args[0])?.to_bytes();
                Ok(Value::Num(s.len() as f64))
            }
            Val => {
                need(1)?;
                let s = self.eval(&args[0])?.to_bytes();
                let txt: String = s.iter().map(|&b| b as char).collect();
                Ok(Value::Num(txt.trim().parse().unwrap_or(0.0)))
            }
            Str => {
                need(1)?;
                let n = self.eval(&args[0])?.to_num();
                Ok(Value::Str(fmt_number(n).into_bytes()))
            }
            Left => {
                need(2)?;
                let s = self.eval(&args[0])?.to_bytes();
                let n = self.eval(&args[1])?.to_num() as usize;
                Ok(Value::Str(s.into_iter().take(n).collect()))
            }
            Right => {
                need(2)?;
                let s = self.eval(&args[0])?.to_bytes();
                let n = self.eval(&args[1])?.to_num() as usize;
                let skip = s.len().saturating_sub(n);
                Ok(Value::Str(s[skip..].to_vec()))
            }
            Mid => {
                need(2)?;
                let s = self.eval(&args[0])?.to_bytes();
                let start = (self.eval(&args[1])?.to_num() as usize).saturating_sub(1);
                let len = if args.len() > 2 {
                    self.eval(&args[2])?.to_num() as usize
                } else {
                    s.len().saturating_sub(start)
                };
                let s_len = s.len();
                let end = (start + len).min(s_len);
                Ok(Value::Str(s[start.min(s_len)..end].to_vec()))
            }
            Peek => {
                need(1)?;
                let a = self.eval(&args[0])?.to_num() as u64;
                Ok(Value::Num(self.do_peek(a) as f64))
            }
            Fre => Ok(Value::Num(38911.0)),
            Pos => Ok(Value::Num(self.screen.col as f64)),
            Tab | Spc => Ok(Value::Str(Vec::new())),
        }
    }

    // -------------------------------------------------------------- PRINT
    fn do_print(&mut self, items: &[PrintItem], trailing_nl: bool) -> Result<(), String> {
        for it in items {
            match it {
                PrintItem::Expr(e) => {
                    let v = self.eval(e)?;
                    for b in v.to_bytes() {
                        self.screen.put_byte(b);
                    }
                }
                PrintItem::Tab(e) => {
                    let n = self.eval(e)?.to_num() as usize;
                    self.screen.print_tab(n);
                }
                PrintItem::Spc(e) => {
                    let n = self.eval(e)?.to_num() as usize;
                    for _ in 0..n {
                        self.screen.put_byte(b' ');
                    }
                }
                PrintItem::Semi => {}
                PrintItem::Comma => {
                    let cur = self.screen.col;
                    let next = ((cur / 10) + 1) * 10;
                    self.screen.print_tab(next.min(COLS - 1));
                }
            }
        }
        if trailing_nl {
            self.screen.put_byte(0x0D);
        }
        Ok(())
    }

    // -------------------------------------------------------------- GET
    fn do_get(&mut self, target: &LValue) -> Result<(), String> {
        let is_str = match target {
            LValue::Var(n) => n.ends_with('$'),
            LValue::Index(n, _) => n.ends_with('$'),
        };
        let v = if self.pending_chars.is_empty() {
            if is_str {
                Value::Str(Vec::new())
            } else {
                Value::Num(0.0)
            }
        } else {
            let b = self.pending_chars.remove(0);
            if is_str {
                Value::Str(vec![b])
            } else {
                Value::Num(b as f64)
            }
        };
        self.assign(target, v)
    }

    fn deliver_input(&mut self, line: Vec<u8>, targets: Vec<LValue>) -> Result<(), String> {
        let text: String = line.iter().map(|&b| b as char).collect();
        let mut parts = text.splitn(targets.len(), ',');
        for t in &targets {
            let piece = parts.next().unwrap_or("").trim();
            let is_str = match t {
                LValue::Var(n) => n.ends_with('$'),
                LValue::Index(n, _) => n.ends_with('$'),
            };
            let v = if is_str {
                Value::Str(piece.bytes().collect())
            } else {
                Value::Num(piece.parse().unwrap_or(0.0))
            };
            self.assign(t, v)?;
        }
        Ok(())
    }

    // -------------------------------------------------------------- POKE/PEEK
    fn do_poke(&mut self, addr: u64, v: u8) {
        match addr {
            53280 => self.screen.set_border(v),
            53281 => self.screen.set_bg(v),
            646 => self.screen.color = v & 0x0F,
            650 => {}   // keyboard repeat
            53272 => {} // character-ROM bank – cosmetic
            a if (1024..=2023).contains(&a) => {
                let idx = (a - 1024) as usize;
                let row = idx / COLS;
                let col = idx % COLS;
                if row < ROWS && col < COLS {
                    self.screen.cells[row][col].byte = screencode_to_petscii(v);
                    self.screen.mark_dirty();
                }
            }
            a if (55296..=56295).contains(&a) => {
                let idx = (a - 55296) as usize;
                let row = idx / COLS;
                let col = idx % COLS;
                if row < ROWS && col < COLS {
                    self.screen.cells[row][col].color = v & 0x0F;
                    self.screen.mark_dirty();
                }
            }
            _ => {}
        }
    }

    fn do_peek(&self, addr: u64) -> u8 {
        match addr {
            53280 => self.screen.border,
            53281 => self.screen.bg,
            646 => self.screen.color,
            197 => 64, // "no key" sentinel
            a if (1024..=2023).contains(&a) => {
                let idx = (a - 1024) as usize;
                let row = idx / COLS;
                let col = idx % COLS;
                petscii_to_screencode(self.screen.cells[row][col].byte)
            }
            a if (55296..=56295).contains(&a) => {
                let idx = (a - 55296) as usize;
                let row = idx / COLS;
                let col = idx % COLS;
                self.screen.cells[row][col].color
            }
            _ => 0,
        }
    }
}

// ---------------------------------------------------------------------------
// PETSCII ↔ screen-code conversion (used by direct POKE/PEEK of the screen).
// Screen codes and PETSCII differ mainly in the top bit and letter offsets.
// ---------------------------------------------------------------------------

fn petscii_to_screencode(b: u8) -> u8 {
    match b {
        0x00..=0x1F => b + 0x80,
        0x20..=0x3F => b,
        0x40 => 0x00,
        0x41..=0x5A => b - 0x40,
        0x5B..=0x5F => b - 0x40,
        0x60..=0x7F => b - 0x20,
        0x80..=0x9F => b - 0x40,
        0xA0..=0xBF => b - 0x40,
        0xC0..=0xFE => b - 0x80,
        0xFF => 0x5E,
    }
}

fn screencode_to_petscii(c: u8) -> u8 {
    match c {
        0x00 => 0x40,
        0x01..=0x1A => c + 0x40,
        0x1B..=0x1F => c + 0x40,
        0x20..=0x3F => c,
        0x40..=0x5F => c + 0x20,
        0x60..=0x7F => c + 0x80,
        0x80..=0xFF => c,
    }
}
