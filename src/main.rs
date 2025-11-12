pub mod data_structures;
pub mod operations;
use data_structures::bitboard::*;
use operations::moves_generate_pieces::*;
fn main() {
   let ffn=String::from("8/8/8/8/3q4/8/3r4/8 w - - 0 1");
   let bitboards=operations::init_board(&ffn);
   let moves=one_square_move(bitboards,7,0);
   let moves=operations::display_bitboard(moves);
   println!("{}",moves);


}
