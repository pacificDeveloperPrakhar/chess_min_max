pub mod data_structures;
pub mod operations;
use data_structures::bitboard::*;
fn main() {
   let ffn=String::from("rnbqkbnr/pppp0ppp/4p3/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
   let bitboards:[[u64;7];2]=operations::init_board(&ffn);
   for i in 0..2
   {
    for j in 0..7
    {
        println!("piece={} {:?}",j,operations::display_bitboard(bitboards[i][j]));
        println!("==============================================================================");
    }
   }
}
