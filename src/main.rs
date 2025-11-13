pub mod data_structures;
pub mod operations;
use data_structures::bitboard::*;
use operations::moves_generate_pieces::*;
use operations::piece_wise_generation::*;
fn main() {
   let ffn=String::from("rnbqkb1r/pppp2pp/5n2/7B/8/8/PPPPPPPP/RNBQK1NR w KQkq - 0 1");
   let bitboards=operations::init_board(&ffn);
   // let moves=pawn_moves(bitboards,1,2);
   // let moves=operations::display_bitboard(moves);
   // println!("{}",moves);
   // println!("{}",bitboards[PieceColor::W as usize][Piece::P as usize]);
   // println!("{}",bitboards[PieceColor::B as usize][Piece::P as usize]);
   let checking= piece_wise_l_squares(bitboards,2,5);
   let moves=operations::display_bitboard(bitboards[PieceColor::W as usize][Piece::A as usize]);
   println!("{}",moves);
   

}
