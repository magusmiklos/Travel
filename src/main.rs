mod parser;
mod ast;
mod interp;

fn main() {
    let path = std::env::args().nth(1).expect("usage: gifdsl <file>");
    let src = std::fs::read_to_string(&path).unwrap();
    println!("Source code:\n{}", src);
    
    let stmts = parser::parse(&src);
    println!("Parsed statements: {:#?}", stmts);
    
    interp::run_program(stmts);
    println!("✅ output.gif generated");
}
