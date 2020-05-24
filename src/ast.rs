#[derive(Debug)]
pub enum Logic {
    Pos, Neg, X, Z
}

pub struct Ast {
    
}

pub struct TableModule {
    header : Vec<String>,
    values : Vec<Vec<Logic>>
}