#[derive(Debug)]
pub enum Logic {
    Pos, Neg, X, Z
}

pub struct Ast {
    
}


pub enum Module {
    Table(TableModule),
    Comb(CombModule),
    Syncr(SyncrModule)
}


pub struct TableModule {
    name : String,
    inputs : Vec<String>,
    outputs : Vec<String>,
    values : Vec<Vec<Logic>>
}

pub struct CombModule {
    inputs : Vec<String>,
    outputs : Vec<String>,
    //nand : NandGraph,
}

pub struct SyncrModule {
    inputs : Vec<String>,
    outputs : Vec<String>,
    procs : Vec<String>,
    //procs : Vec<NandGraph>,
}