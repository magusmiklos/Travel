use gif::{Encoder, Frame, Repeat};
use crate::ast::{Stmt, Expr};

fn draw_circle(buf: &mut [u8], w: usize, h: usize, cx: i32, cy: i32, radius: i32, color: [u8; 3]) {
    for y in (cy - radius)..=(cy + radius) {
        for x in (cx - radius)..=(cx + radius) {
            let dx = x - cx;
            let dy = y - cy;
            if dx * dx + dy * dy <= radius * radius {
                if x >= 0 && x < w as i32 && y >= 0 && y < h as i32 {
                    let idx = (y as usize * w + x as usize) * 3;
                    buf[idx] = color[0];
                    buf[idx + 1] = color[1];
                    buf[idx + 2] = color[2];
                }
            }
        }
    }
}

pub fn run_program(stmts: Vec<Stmt>) {
    let mut frames: Vec<Vec<u8>> = Vec::new();
    let w = 200;
    let h = 200;

    for stmt in stmts {
        if let Stmt::Travel { frames: n, body } = stmt {
            for t in 0..n {
                let mut buf = vec![0u8; w * h * 3];
                execute_block(&body, &mut buf, w, h, t, n);
                frames.push(buf);
            }
        }
    }

    let mut file = std::fs::File::create("output.gif").unwrap();
    let mut enc = Encoder::new(&mut file, w as u16, h as u16, &[]).unwrap();
    enc.set_repeat(Repeat::Infinite).unwrap();
    for buf in frames {
        let frame = Frame::from_rgb(w as u16, h as u16, &buf);
        enc.write_frame(&frame).unwrap();
    }
}

fn execute_block(block: &[Stmt], buf: &mut [u8], w: usize, h: usize, t: u32, n: u32) {
    for stmt in block {
        match stmt {
            Stmt::Expr(expr, _times) => {
                if let Expr::Call(name) = expr {
                    if name == "circle" {
                        let cx = w as i32 / 2;
                        let cy = h as i32 / 2;
                        let radius = 50;
                        let color = [255, 0, 0];
                        draw_circle(buf, w, h, cx, cy, radius, color);
                        println!("Drew circle at ({}, {})", cx, cy);
                    }
                }
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                let cond_value = eval_condition(condition, t);
                println!("Condition evaluated to: {}", cond_value);
                if cond_value {
                    execute_block(then_body, buf, w, h, t, n);
                } else {
                    execute_block(else_body, buf, w, h, t, n);
                }
            }
            _ => {}
        }
    }
}

fn eval_condition(expr: &Expr, current_frame: u32) -> bool {
    // Evaluate to u32 then convert to bool
    let value = eval_expr(expr, current_frame);
    value != 0
}

fn eval_expr(expr: &Expr, current_frame: u32) -> u32 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Variable(name) if name == "travel" => current_frame,
        Expr::Binary { left, op, right } => {
            let left_val = eval_expr(left, current_frame);
            let right_val = eval_expr(right, current_frame);
            
            match op.as_str() {
                "+" => left_val + right_val,
                "-" => left_val - right_val,
                "*" => left_val * right_val,
                "/" => left_val / right_val,
                "%" => left_val % right_val,
                "==" => (left_val == right_val) as u32,
                "!=" => (left_val != right_val) as u32,
                ">" => (left_val > right_val) as u32,
                "<" => (left_val < right_val) as u32,
                _ => 0,
            }
        }
        _ => 0,
    }
}
