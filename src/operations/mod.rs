use std::vec::Vec;
// this vec macro imported by default in the root crate
use alloc::vec;
pub fn init_board(epd:String)->[[u8;7];2]
{
  let epd_split:Vec<String>=epd.split(" ").collect();
  let board:String=epd_split[0];
  for (i,c) in board.chars().enumerate()
  {
   
  }
}