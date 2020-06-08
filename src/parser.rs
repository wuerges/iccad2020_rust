// extern crate nom;

use crate::ast::*;

use nom::{
  IResult,
//   bytes::complete::{tag, take_while_m_n},
//   combinator::map_res, 
//   sequence::tuple,
    // combinator::complete,
    // bytes::streaming::take,
    // Err::Error,
    // error::ErrorKind
};


named!(initial<char>, one_of!("abcdefghijklmnopqrstuvwxyz_"));
named!(subsequent<char>, one_of!("0123456789"));
// named!(special_initial<char>, one_of!("!$%&*/:<=>?^_~"));
// named!(special_subsequent<char>, one_of!("+-.@"));

// named!(space<&[u8], ()>, 
//     do_parse!(many0!(one_of!(" \r\n\t")) >> ())
// );

named!(endl<&[u8], ()>, 
    do_parse!(tag!("\n") >> ())
);

named!(space<&[u8], ()>, 
    do_parse!(
        many0!(alt!(
            do_parse!(one_of!("\r\n\t ") >> ())
            | do_parse!(
                tag!("//") >> 
                many0!(none_of!("\n")) >> 
                tag!("\n") >> ())
        ))  
    >> ())
);

named!(identifier<&[u8], String>,
    map!(
        recognize!(
            do_parse!( initial >> many0!(complete!(alt!(initial | subsequent))) >> ())
        )
        ,
        |s| String::from_utf8_lossy(s).into_owned()
    )
);

named!(timescale<&[u8], ()>,
    do_parse!( tag!("`timescale") >> space >>  tag!("1ps/1ps") >> space >>  ())
);

named!(pub vlib_file<&[u8], Vec<Module>>,
    do_parse!( 
        timescale >> 
        mods: many1!(module) >> 
        (mods)
    )
);

named!(module<&[u8], Module>,
    map!(udp,
        |x| Module::Primitive(x)
    )
);

// <UDP>
//    ::= primitive <name_of_UDP> ( <output_terminal_name>,
//       <input_terminal_name> <,<input_terminal_name>>* ) ;
//    <UDP_declaration>+
//    <UDP_initial_statement>?
//    <table_definition>
//    endprimitive


named!(comma<&[u8], ()>, do_parse!( tag!(",") >> space >> ()));

named!(udp<&[u8], String>, 
    do_parse!( 
        tag!("primitive") >> space >> 
        name_of_udp : identifier >> 
        take_until!("endprimitive") >> 
        tag!("endprimitive")  >> space

        // tag!(")")  >> space >>
        >> (name_of_udp)
    )
);

// <UDP_declaration>
//    ::= <UDP_output_declaration>
//    ||= <reg_declaration>
//    ||= <UDP_input_declaration>

named!(udp_declaration<&[u8], ()>, 
    do_parse!( 
        alt!(output_declaration 
            | reg_declaration 
            | input_declaration)
            >> ()        
    )
);

// <UDP_output_declaration>
//    ::= output <output_terminal _name>;

named!(output_declaration<&[u8], ()>, 
    do_parse!( 
        tag!("output") >> space >> 
        identifier >> space >>
        tag!(";") >> space >> 
        ()
    )
);

// <reg_declaration>
//    ::=   reg <output_terminal_name> ;

named!(reg_declaration<&[u8], ()>, 
    do_parse!( 
        tag!("reg") >> space >> 
        identifier >> space >>
        tag!(";") >> space >> 
        ()
    )
);

// <UDP_input_declaration>
//    ::= input <input_terminal_name> <,<input_terminal_name>>* ;

named!(input_declaration<&[u8], ()>, 
    do_parse!( 
        tag!("input") >> space >> 
        identifier >> space >>
        many0!( do_parse!(comma >> identifier >> space >> ()))>> 
        tag!(";") >> space >> 
        ()
    )
);


// <UDP_initial_statement>
//    ::= initial <output_terminal_name> = <init_val> ;

// <init_val>
//    ::= 1'b0
//    ||= 1'b1
//    ||= 1'bx
//    ||= 1
//    ||= 0

// <table_definition>
//    ::= table
//          <table_entries>
//       endtable
// <table_entries>
//    ::= <combinational_entry>+
//    ||= <sequential_entry>+

named!(level_symbol<&[u8], Logic>,
    alt!(
        do_parse!(tag!("?") >> (Logic::Unknown))
        | do_parse!(tag!("0") >> (Logic::Neg))
        | do_parse!(tag!("1") >> (Logic::Pos))
        | do_parse!(tag!("x") >> (Logic::X))
        | do_parse!(tag!("X") >> (Logic::X))
    )
);

named!(level<&[u8], Level>,
    alt!( 
        do_parse!( l : level_symbol >> (Level::Single(l)) )
        | edge 
    )
);

named!(edge<&[u8], Level>,
    do_parse!(
        tag!("(") >> 
        a : level_symbol >> 
        b : level_symbol >> tag!(")")
        >> (Level::Pair(a, b))
    )
);

named!(table_def<&[u8], ()>,
    do_parse!( tag!("table") >> space >>
        // alt!(many1!(combinational_entry)
        //     | many1!(sequential_entry))
            // >> 
            ()
    )
);


// <combinational_entry>
//    ::= <level_input_list> : <OUTPUT_SYMBOL> ;



// <sequential_entry>
//    ::= <input_list> : <state> : <next_state> ;

// <input_list>
//    ::= <level_input_list>
//    ||= <edge_input_list>

// <level_input_list>
//    ::= <LEVEL_SYMBOL>+

// <edge_input_list>
//    ::= <LEVEL_SYMBOL>* <edge> <LEVEL_SYMBOL>*

// <edge>
//    ::= ( <LEVEL_SYMBOL> <LEVEL_SYMBOL> )
//    ||= <EDGE_SYMBOL>

// <state>
//    ::= <LEVEL_SYMBOL>

// <next_state>
//    ::= <OUTPUT_SYMBOL>
//    ||= -    


// fn ident(input : &str) -> IResult<&str, String> {
//     return identifier(input);
// }

// named!(pub parse_file<&[u8]>, identifier );


#[cfg(test)]
mod tests {
    // use nom::bytes::streaming::take;
    // use nom::combinator::complete;
    // use nom::Err;
    // use nom::error::ErrorKind;
    // use nom::IResult;
    // use nom::IResult::Done;
    // use nom::IResult::Error;
    use crate::parser::*;
    
    #[test] 
    fn test_identifier() {
        assert_eq!(identifier(b"aaaa"), Ok((&[][..], "aaaa".to_owned())));
    }

    // #[test]
    // fn test_parse() {
    //     // let parser = complete(shitty);
        
        
        
        
    //     println!("\n\n\n\n>>>>>>>>{:?}<<<<<<<<<", identifier(b"aaaa"));
    //     // println!("\n\n\n\n>>>>>>>>{:?}<<<<<<<<<", complete(parser2)("aaaa"));

    //     //assert_eq!(multi(&b[..]), Error(error_position!(ErrorKind::Many1,&b[..])));
    // }
}