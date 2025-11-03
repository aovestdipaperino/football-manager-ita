/// Execution tracing for C64 fidelity debugging
///
/// This module provides tracing capabilities to compare our interpreter's
/// execution with a real C64 emulator.

use crate::value::Value;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceEntry {
    pub line_number: u32,
    pub statement_index: usize,
    pub statement_type: String,
    pub variables_snapshot: HashMap<String, String>, // Serialized values
    pub output: String,
    pub screen_output: Option<String>,
}

#[derive(Debug, Default)]
pub struct ExecutionTracer {
    pub entries: Vec<TraceEntry>,
    pub enabled: bool,
    current_output: String,
}

impl ExecutionTracer {
    pub fn new() -> Self {
        ExecutionTracer {
            entries: Vec::new(),
            enabled: false,
            current_output: String::new(),
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn record_statement(
        &mut self,
        line_number: u32,
        statement_index: usize,
        statement_type: &str,
        variables: &HashMap<String, Value>,
    ) {
        if !self.enabled {
            return;
        }

        let mut var_snapshot = HashMap::new();
        for (name, value) in variables {
            var_snapshot.insert(
                name.clone(),
                match value {
                    Value::Number(n) => format!("{}", n),
                    Value::String(s) => format!("\"{}\"", s),
                }
            );
        }

        let entry = TraceEntry {
            line_number,
            statement_index,
            statement_type: statement_type.to_string(),
            variables_snapshot: var_snapshot,
            output: self.current_output.clone(),
            screen_output: None,
        };

        self.entries.push(entry);
        self.current_output.clear();
    }

    pub fn record_output(&mut self, text: &str) {
        if self.enabled {
            self.current_output.push_str(text);
        }
    }

    pub fn save_to_json(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.entries)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn print_summary(&self) {
        println!("=== Execution Trace Summary ===");
        println!("Total statements executed: {}", self.entries.len());

        if let Some(first) = self.entries.first() {
            println!("First line: {}", first.line_number);
        }

        if let Some(last) = self.entries.last() {
            println!("Last line: {}", last.line_number);
        }

        // Count statement types
        let mut type_counts: HashMap<String, usize> = HashMap::new();
        for entry in &self.entries {
            *type_counts.entry(entry.statement_type.clone()).or_insert(0) += 1;
        }

        println!("\nStatement type counts:");
        let mut types: Vec<_> = type_counts.iter().collect();
        types.sort_by_key(|(_, count)| std::cmp::Reverse(*count));
        for (stmt_type, count) in types.iter().take(10) {
            println!("  {}: {}", stmt_type, count);
        }
    }

    pub fn print_detailed(&self, limit: Option<usize>) {
        println!("=== Detailed Execution Trace ===");
        let entries = if let Some(n) = limit {
            &self.entries[..n.min(self.entries.len())]
        } else {
            &self.entries
        };

        for (i, entry) in entries.iter().enumerate() {
            println!("\n[{}] Line {} Statement {}: {}",
                i, entry.line_number, entry.statement_index, entry.statement_type);

            if !entry.output.is_empty() {
                println!("  Output: {:?}", entry.output);
            }

            if !entry.variables_snapshot.is_empty() {
                println!("  Variables:");
                let mut vars: Vec<_> = entry.variables_snapshot.iter().collect();
                vars.sort_by_key(|(name, _)| name.as_str());
                for (name, value) in vars.iter().take(10) {
                    println!("    {} = {}", name, value);
                }
                if vars.len() > 10 {
                    println!("    ... and {} more", vars.len() - 10);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tracer_basic() {
        let mut tracer = ExecutionTracer::new();
        tracer.enable();

        let mut vars = HashMap::new();
        vars.insert("I".to_string(), Value::Number(1.0));

        tracer.record_statement(10, 0, "FOR", &vars);
        tracer.record_output("Hello");
        tracer.record_statement(20, 0, "PRINT", &vars);

        assert_eq!(tracer.entries.len(), 2);
        assert_eq!(tracer.entries[0].statement_type, "FOR");
        assert_eq!(tracer.entries[1].output, "Hello");
    }
}
