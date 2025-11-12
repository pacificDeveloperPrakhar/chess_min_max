pub mod moves_generate_pieces;
use std::vec::Vec;
use crate::data_structures::bitboard::PieceColor;
use crate::data_structures::bitboard::Piece;
pub fn init_board(epd:&String)->[[u64;7];2]
{
  let mut rank:u8=0;
  let mut step=0;
  let epd_split:Vec<&str>=epd.split(" ").collect();
  let board:&str=epd_split[0];

  // this bitboard will hold the data which will be our result
  let mut bitboards:[[u64;7];2]=[[0 as u64;7];2];
  for (i,c) in board.chars().enumerate()
  {
   if c=='/'
   {
    rank+=1;
    continue;
   }
   if (c as u8 >= 48) && (c as u8 <= 56)
   {
    step+=((c as u8)-48);
    if (c as u8)==48
    {
      step+=1;
    }
    continue;
   }
   
   let piece_position:u64 = 1 << step;
   let side = if (c as u8) < 91 { 'b' } else { 'w' };

   
   bitboards[PieceColor::from(side) as usize][Piece::from(c) as usize]|=piece_position;
   step+=1;
  }

//   now calculate the white side bitboard 
  for i in 0..6
  {
    bitboards[PieceColor::from('w') as usize][6]|=bitboards[PieceColor::from('w') as usize][i];
    bitboards[PieceColor::from('b') as usize][6]|=bitboards[PieceColor::from('b') as usize][i];
  }
  return bitboards;
}


//function to display the bitboard
// function to display the bitboard (reversed order)
pub fn display_bitboard(board: u64) -> String {
  let mut temp = board;
  let mut result = String::new();

  for _rank in 0..8 {
      let mut line = String::new();

      for _file in 0..8 {
          if (temp & 1) == 1 {
              line = String::from("1 ") + &line;
          } else {
              line = String::from("0 ") + &line;
          }
          temp >>= 1;
      }

      // append the constructed rank at the end of result
      result=line + &result;
      result=String::from("\n")+&result;

  }

  result
}


pub fn generate_moves(bitboards:[[u64;7];2],rank:usize,file:usize)->(u64,u64)
{
 return (0,0);
}