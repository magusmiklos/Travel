#[derive(Debug)]
pub enum Expr {
    Call(String),
    Number(u32),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Stmt {
    Expr(Expr, u32),
    Travel {
        frames: u32,
        body: Vec<Stmt>,
    },
    If {
        condition: Box<Expr>,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
}
