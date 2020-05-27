#[derive(Debug)]
pub enum Logic {
    Pos, Neg, X, Z
}

#[derive(Debug)]
pub struct Ast {
    
}


#[derive(Debug)]
pub enum Module {
    Table(TableModule)//,
    // Comb(CombModule),
    // Syncr(SyncrModule)
}


#[derive(Debug)]
pub struct TableModule {
    pub name : String,
    pub output : String,
    pub inputs : Vec<String>,
    // outputs : Vec<String>,
    // values : Vec<Vec<Logic>>
}

// impl TableModule {
//     create::
// }


// pub struct CombModule {
//     inputs : Vec<String>,
//     outputs : Vec<String>,
//     //nand : NandGraph,
// }

// pub struct SyncrModule {
//     inputs : Vec<String>,
//     outputs : Vec<String>,
//     procs : Vec<String>,
//     //procs : Vec<NandGraph>,
// }