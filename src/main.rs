pub mod data_structures;
pub mod operations;
use data_structures::bitboard::*;
use operations::moves_generate_pieces::*;
fn main() {
   let ffn=String::from("rnbqkbnr/ppp1pppp/1P6/8/p7/8/PP1PPPPP/RNBQKBNR w KQkq - 0 1");
   let bitboards=operations::init_board(&ffn);
   let moves=pawn_moves(bitboards,1,2);
   let moves=operations::display_bitboard(moves);
   println!("{}",moves);
   println!("{}",bitboards[PieceColor::W as usize][Piece::P as usize]);
   println!("{}",bitboards[PieceColor::B as usize][Piece::P as usize]);

}
