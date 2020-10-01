// extern crate nom;

use crate::ast::*;

use nom::{
    character::complete::{
        one_of, 
        alpha1,
        alphanumeric0,
        multispace0,
        line_ending,
        not_line_ending,
        anychar
    },
    bytes::complete::{tag, take_until},
    combinator::{recognize, map, opt},
    sequence::{pair, tuple},
    branch::{alt, permutation},

    multi::{many0, many1, many_till, separated_list},
    IResult
//   bytes::complete::{tag, take_while_m_n},
//   combinator::map_res, 
//   sequence::tuple,
    // combinator::complete,
    // bytes::streaming::take,
    // Err::Error,
    // error::ErrorKind
};

fn initial(input: &str) -> IResult<&str, &str> {
    // tag("_")(input)
    alt((alpha1, tag("_")))(input)
}

fn identifier(input: &str) -> IResult<&str, &str> {
    recognize(pair(initial, alphanumeric0))(input)
}

// // named!(special_initial<char>, one_of!("!$%&*/:<=>?^_~"));
// // named!(special_subsequent<char>, one_of!("+-.@"));


fn comment(i: &str) -> IResult<&str, ()> {
    map(
        pair(tag("//"), many_till(anychar, line_ending)),
        |_| ()
    )(i)
}

fn space(input: &str) -> IResult<&str, ()> {
    // map(separated_list(comment, multispace0), |_| ())(input)
    map(pair(many0(pair(multispace0, comment)), multispace0), |_| ())(input)
    // let ms0 = map(multispace0, |_| ());
    // map(many0(alt((ms0, comment))), |_| ())(input)
}

fn endl(input: &str) -> IResult<&str, ()> {
    map(line_ending, |_| ())(input)
}

fn timescale(i: &str) -> IResult<&str, ()> {
    map(
        tuple(
            (tag("`timescale"), 
            space, 
            tag("1ps/1ps"), 
            space)
        ), |_| ())(i)
}

pub fn vlib_file(i: &str) -> IResult<&str, Vec<Module>> {
    map(pair(timescale, many1(module)), |(_, b)| b)(i)
}

fn module(i: &str) -> IResult<&str, Module> {
    map(udp, |x| Module::Primitive(x.to_owned()))(i)
}

fn comma(i: &str) -> IResult<&str, ()> {
    map(pair(tag(","), space), |_| ())(i)
}

// <UDP>
//    ::= primitive <name_of_UDP> ( <output_terminal_name>,
//       <input_terminal_name> <,<input_terminal_name>>* ) ;
//    <UDP_declaration>+
//    <UDP_initial_statement>?
//    <table_definition>
//    endprimitive
// <UDP_declaration>
//    ::= <UDP_output_declaration>
//    ||= <reg_declaration>
//    ||= <UDP_input_declaration>
// <UDP_output_declaration>
//    ::= output <output_terminal _name>;

fn udp(i :&str) -> IResult<&str, &str> {
    map(tuple((
        tag("primitive"), space, 
        identifier, take_until("endprimitive"), space,
        tag("endprimitive"), space
    )), |(_,_, x,_, _, _, _)| x)(i)
}

// named!(udp_declaration<&[u8], ()>, 
//     do_parse!( 
//         alt!(output_declaration 
//             | reg_declaration 
//             | input_declaration)
//             >> ()        
//     )
// );


// named!(output_declaration<&[u8], ()>, 
//     do_parse!( 
//         tag!("output") >> space >> 
//         identifier >> space >>
//         tag!(";") >> space >> 
//         ()
//     )
// );

// <reg_declaration>
//    ::=   reg <output_terminal_name> ;

// named!(reg_declaration<&[u8], ()>, 
//     do_parse!( 
//         tag!("reg") >> space >> 
//         identifier >> space >>
//         tag!(";") >> space >> 
//         ()
//     )
// );

// <UDP_input_declaration>
//    ::= input <input_terminal_name> <,<input_terminal_name>>* ;

// named!(input_declaration<&[u8], ()>, 
//     do_parse!( 
//         tag!("input") >> space >> 
//         identifier >> space >>
//         many0!( do_parse!(comma >> identifier >> space >> ()))>> 
//         tag!(";") >> space >> 
//         ()
//     )
// );


// // <UDP_initial_statement>
// //    ::= initial <output_terminal_name> = <init_val> ;

// // <init_val>
// //    ::= 1'b0
// //    ||= 1'b1
// //    ||= 1'bx
// //    ||= 1
// //    ||= 0

// // <table_definition>
// //    ::= table
// //          <table_entries>
// //       endtable
// // <table_entries>
// //    ::= <combinational_entry>+
// //    ||= <sequential_entry>+

// named!(level_symbol<&[u8], Logic>,
//     alt!(
//         do_parse!(tag!("?") >> (Logic::Unknown))
//         | do_parse!(tag!("0") >> (Logic::Neg))
//         | do_parse!(tag!("1") >> (Logic::Pos))
//         | do_parse!(tag!("x") >> (Logic::X))
//         | do_parse!(tag!("X") >> (Logic::X))
//     )
// );

// named!(level<&[u8], Level>,
//     alt!( 
//         do_parse!( l : level_symbol >> (Level::Single(l)) )
//         | edge 
//     )
// );

// named!(edge<&[u8], Level>,
//     do_parse!(
//         tag!("(") >> 
//         a : level_symbol >> 
//         b : level_symbol >> tag!(")")
//         >> (Level::Pair(a, b))
//     )
// );

// named!(table_def<&[u8], ()>,
//     do_parse!( tag!("table") >> space >>
//         // alt!(many1!(combinational_entry)
//         //     | many1!(sequential_entry))
//             // >> 
//             ()
//     )
// );


// // <combinational_entry>
// //    ::= <level_input_list> : <OUTPUT_SYMBOL> ;



// // <sequential_entry>
// //    ::= <input_list> : <state> : <next_state> ;

// // <input_list>
// //    ::= <level_input_list>
// //    ||= <edge_input_list>

// // <level_input_list>
// //    ::= <LEVEL_SYMBOL>+

// // <edge_input_list>
// //    ::= <LEVEL_SYMBOL>* <edge> <LEVEL_SYMBOL>*

// // <edge>
// //    ::= ( <LEVEL_SYMBOL> <LEVEL_SYMBOL> )
// //    ||= <EDGE_SYMBOL>

// // <state>
// //    ::= <LEVEL_SYMBOL>

// // <next_state>
// //    ::= <OUTPUT_SYMBOL>
// //    ||= -    


// // fn ident(input : &str) -> IResult<&str, String> {
// //     return identifier(input);
// // }

// // named!(pub parse_file<&[u8]>, identifier );

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
        assert_eq!(identifier("aaaa"), Ok(("", "aaaa")));
        assert_eq!(identifier("aaa a"), Ok((" a", "aaa")));
        assert_eq!(identifier("_aaa a"), Ok((" a", "_aaa")));
    }

    #[test]
    fn test_comment() {
        assert_eq!(comment("// hello\nworld"), Ok(("world", ())));
        assert_eq!(comment("// hello\n \n world"), Ok((" \n world", ())));
    }

    #[test]
    fn test_space() {
        assert_eq!(space(" // hello\n    world"), Ok(("world", ())));
        assert_eq!(space("// hello\n \n world"), Ok(("world", ())));
    }

    // #[test]
    // fn test_parse() {
    //     // let parser = complete(shitty);
        
        
        
        
    //     println!("\n\n\n\n>>>>>>>>{:?}<<<<<<<<<", identifier(b"aaaa"));
    //     // println!("\n\n\n\n>>>>>>>>{:?}<<<<<<<<<", complete(parser2)("aaaa"));

    //     //assert_eq!(multi(&b[..]), Error(error_position!(ErrorKind::Many1,&b[..])));
    // }
}