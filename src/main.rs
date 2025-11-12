pub mod data_structures;
pub mod operations;
use data_structures::bitboard::*;
use operations::moves_generate_pieces::*;
fn main() {
   let ffn=String::from("b1n5/4Q3/8/2N5/4k3/3PK3/6B1/8 w - - 0 1");
   let bitboards=operations::init_board(&ffn);
   // let moves=pawn_moves(bitboards,1,2);
   // let moves=operations::display_bitboard(moves);
   // println!("{}",moves);
   // println!("{}",bitboards[PieceColor::W as usize][Piece::P as usize]);
   // println!("{}",bitboards[PieceColor::B as usize][Piece::P as usize]);
   let checking= is_king_checked(bitboards,'w');
   let moves=operations::display_bitboard(checking);
   println!("{}",moves);
   

}
