/*primitive udp_xbuf (o, i, check_signal);
  output o;
  input i, check_signal;
  table      
  // i check_signal : o
      0   1   : 0;
      1   1   : 1;
      x   1   : 1;
   endtable
endprimitive
*/


use crate::ast::*;

pub struct Local {
    
}
pub struct Global {

}

pub struct udp_xbuf {
    Vec<Logic> outputs,
    Vec<Logic> inputs
}


impl udp_xbuf {
    pub fn run(input : usize, value : Logic) {

    }
}