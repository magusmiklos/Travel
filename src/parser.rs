use tree_sitter_gifdsl::language;
use tree_sitter::{Parser, Node};
use crate::ast::{Stmt, Expr};

/// Parses the source DSL and returns a Vec<Stmt>
pub fn parse(source: &str) -> Vec<Stmt> {
    let mut parser = Parser::new();
    parser.set_language(&language()).expect("Language version mismatch");
    let tree = parser.parse(source, None).unwrap();
    let root = tree.root_node();

    let mut stmts = Vec::new();
    
    for node in root.named_children(&mut root.walk()) {
        match node.kind() {
            "travel_block" => {
                let frames_node = node.child(2).unwrap();
                let frames: u32 = frames_node.utf8_text(source.as_bytes()).unwrap().parse().unwrap();
                let body = parse_block(&node, source);
                stmts.push(Stmt::Travel { frames, body });
            }
            _ => {}
        }
    }
    stmts
}

/// Parse a block of statements (inside travel or if/else)
fn parse_block(node: &Node, source: &str) -> Vec<Stmt> {
    let mut body = Vec::new();
    
    // Start from the colon (index 3) for travel blocks
    let start_index = if node.kind() == "travel_block" { 4 } else { 0 };
    
    for i in start_index..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.is_named() {
                body.push(parse_stmt(&child, source));
            }
        }
    }
    body
}

/// Parse an individual statement
fn parse_stmt(node: &Node, source: &str) -> Stmt {
    println!("Parsing statement: {}", node.kind());
    match node.kind() {
        "expr_stmt" => {
            let expr_node = node.child(0).unwrap();
            let expr = parse_expr(&expr_node, source);
            
            let mut times = 1;
            for i in 1..node.child_count() {
                if let Some(num_node) = node.child(i) {
                    if num_node.kind() == "number" {
                        times = num_node.utf8_text(source.as_bytes()).unwrap().parse().unwrap();
                    }
                }
            }
            Stmt::Expr(expr, times)
        }
        "if_stmt" => {
            println!("Found if statement");
            // Condition is the first named child after 'if' and before ':'
            let cond_node = node.named_child(0).unwrap();
            let condition = parse_expr(&cond_node, source);
            println!("Parsed condition: {:?}", condition);
            
            // Find the then and else blocks
            let mut then_body = Vec::new();
            let mut else_body = Vec::new();
            let mut in_else = false;
            
            for i in 0..node.child_count() {
                if let Some(child) = node.child(i) {
                    if child.kind() == "else" {
                        println!("Found else keyword");
                        in_else = true;
                    } else if child.is_named() && child.kind() != "expr" {
                        if in_else {
                            println!("Adding to else body");
                            then_body.push(parse_stmt(&child, source));
                        } else {
                            println!("Adding to then body");
                            else_body.push(parse_stmt(&child, source));
                        }
                    }
                }
            }
            
            Stmt::If {
                condition: Box::new(condition),
                then_body,
                else_body,
            }
        }
        _ => panic!("Unknown statement type: {}", node.kind()),
    }
}

/// Parse an expression
fn parse_expr(node: &Node, source: &str) -> Expr {
    match node.kind() {
        "expr" => {
            // Handle expression wrapper node
            if let Some(child) = node.named_child(0) {
                parse_expr(&child, source)
            } else {
                panic!("Expression node has no children")
            }
        }
        "call_expr" => {
            let name = node.child(0).unwrap().utf8_text(source.as_bytes()).unwrap().to_string();
            Expr::Call(name)
        }
        "binary_expr" => {
            let left = node.child(0).unwrap();
            let operator = node.child(1).unwrap();
            let right = node.child(2).unwrap();
            
            Expr::Binary {
                left: Box::new(parse_expr(&left, source)),
                op: operator.utf8_text(source.as_bytes()).unwrap().to_string(),
                right: Box::new(parse_expr(&right, source)),
            }
        }
        "identifier" => {
            let name = node.utf8_text(source.as_bytes()).unwrap().to_string();
            Expr::Variable(name)
        }
        "number" => {
            let value = node.utf8_text(source.as_bytes()).unwrap().parse().unwrap();
            Expr::Number(value)
        }
        _ => panic!("Unknown expression type: {}", node.kind()),
    }
}
