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
   
   println!("{} ={}",c,(7-((step%8) as u8))+rank*8);
   let piece_position:u64 = 1 << ((7-((step%8) as u8))+rank*8);
   let side = if (c as u8) < 91 { 'b' } else { 'w' };

   
   bitboards[PieceColor::from(side) as usize][Piece::from(c) as usize]|=piece_position;
   step+=1;
  }
  return bitboards;
}


//function to display the bitboard
pub fn display_bitboard(board:u64)->String
{
    let mut temp=board;
    let mut result=String::new();
    for rank in 0..8
    {
        for file in 0..8
        {
            if (temp&1)==1
            {
                result=String::from("1 ")+&result;
            }
            else
            {
                result=String::from("0 ")+&result; // calls method of the string newly created and then add it to it
            }
            temp=temp>>1;
        }
        result=String::from("\n")+&result;
    }
    println!("{}",result);
    return result;
}