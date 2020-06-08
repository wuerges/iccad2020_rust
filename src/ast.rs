#[derive(Debug)]
pub enum Logic {
    Pos, Neg, X, Z, Unknown
}

pub enum Level {
    Single(Logic),
    Pair(Logic, Logic)
}

#[derive(Debug)]
pub struct Ast {
    
}


#[derive(Debug)]
pub enum Module {
    Primitive(String)//,
    // Comb(CombModule),
    // Syncr(SyncrModule)
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