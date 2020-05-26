// extern crate nom;

use crate::ast;

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

named!(letter<char>, one_of!("abcdefghijklmnopqrstuvwxyz"));
named!(single_digit<char>, one_of!("0123456789"));
named!(special_initial<char>, one_of!("!$%&*/:<=>?^_~"));
named!(special_subsequent<char>, one_of!("+-.@"));

// named!(pub shitty<&[u8]>, recognize!(many1!(complete!(letter))) );

named!(pub identifier<&[u8], String>,
    map!(
        recognize!(
            do_parse!( letter >> many0!(complete!(alt!(letter | single_digit | special_subsequent))) >> ())
        )
        ,
        |s| String::from_utf8_lossy(s).into_owned()
    )
);

// fn ident(input : &str) -> IResult<&str, String> {
//     return identifier(input);
// }

named!(pub parser2<&str, Vec<&str> >, many1!( tag!( "a" ) ) );


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