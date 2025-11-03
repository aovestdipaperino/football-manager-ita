fn main() {
    let input = "PRINTCHR$(142)";

    println!("Input: {}", input);
    println!("Starts with PRINT: {}", input.starts_with("PRINT"));
    println!("Char at position 5: {:?}", input.chars().nth(5));

    let is_keyword_boundary = |c: char| !c.is_ascii_alphabetic() && c != '_';
    if let Some(c) = input.chars().nth(5) {
        println!("Is keyword boundary: {}", is_keyword_boundary(c));
    }

    let should_match = input.starts_with("PRINT") && input.chars().nth(5).map_or(true, is_keyword_boundary);
    println!("Should match PRINT keyword: {}", should_match);

    // Try parsing
    let line = r#"170 PRINTCHR$(142):GOSUB2000:PRINT"""#;
    println!("\nParsing: {}", line);
    match basic_emulator::parser::Parser::parse_program(line) {
        Ok(program) => {
            println!("✓ Parsed successfully!");
        }
        Err(e) => {
            eprintln!("✗ Parse error: {}", e);
        }
    }
}
