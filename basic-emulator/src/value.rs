use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    /// String stored as original PETSCII bytes for proper charset handling
    String(Vec<u8>),
}

impl Value {
    pub fn as_number(&self) -> Result<f64, String> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::String(_) => Err("Type mismatch: expected number".to_string()),
        }
    }

    pub fn as_string(&self) -> Result<&[u8], String> {
        match self {
            Value::String(s) => Ok(s),
            Value::Number(_) => Err("Type mismatch: expected string".to_string()),
        }
    }

    pub fn as_string_bytes(&self) -> Result<Vec<u8>, String> {
        match self {
            Value::String(s) => Ok(s.clone()),
            Value::Number(_) => Err("Type mismatch: expected string".to_string()),
        }
    }

    pub fn as_int(&self) -> Result<i64, String> {
        Ok(self.as_number()?.floor() as i64)
    }

    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
        }
    }

    pub fn to_display_string(&self) -> String {
        match self {
            Value::Number(n) => {
                // C64-style number formatting
                let formatted = if n.fract() == 0.0 && n.abs() < 1e10 {
                    // Integer display
                    format!("{}", *n as i64)
                } else {
                    // Float display - limit to 9 significant digits like C64
                    // C64 BASIC shows max 9 digits after the decimal point
                    let s = format!("{:.9}", n);
                    // Trim trailing zeros after decimal point
                    let mut result = s;
                    if result.contains('.') {
                        result = result.trim_end_matches('0').trim_end_matches('.').to_string();
                    }
                    result
                };

                // Add leading space for positive numbers (C64 sign alignment)
                if *n >= 0.0 {
                    format!(" {}", formatted)
                } else {
                    formatted
                }
            }
            Value::String(bytes) => {
                // Convert PETSCII bytes to UTF-8 for display
                // This is only for debug display, actual printing uses original bytes
                String::from_utf8_lossy(bytes).to_string()
            }
        }
    }

    /// Get the PETSCII bytes for printing (preserves original bytes)
    pub fn as_petscii_bytes(&self) -> Vec<u8> {
        match self {
            Value::Number(n) => {
                // Convert number to string bytes
                let s = self.to_display_string();
                s.into_bytes()
            }
            Value::String(bytes) => bytes.clone(),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_display_string())
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

impl From<Vec<u8>> for Value {
    fn from(bytes: Vec<u8>) -> Self {
        Value::String(bytes)
    }
}

impl From<&[u8]> for Value {
    fn from(bytes: &[u8]) -> Self {
        Value::String(bytes.to_vec())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s.into_bytes())
    }
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.as_bytes().to_vec())
    }
}
