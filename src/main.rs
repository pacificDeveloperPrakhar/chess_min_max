pub mod data_structures;
pub mod operations;
use data_structures::bitboard::*;
use operations::moves_generate_pieces::*;
use operations::piece_wise_generation::*;
fn main() {
   let ffn=String::from("rn1qkb1r/pppp2pp/8/5b1B/8/8/PPPPP1PP/RNBQK1NR w KQkq - 0 1");
   let bitboards=operations::init_board(&ffn);
   // let moves=pawn_moves(bitboards,1,2);
   // let moves=operations::display_bitboard(moves);
   // println!("{}",moves);
   // println!("{}",bitboards[PieceColor::W as usize][Piece::P as usize]);
   // println!("{}",bitboards[PieceColor::B as usize][Piece::P as usize]);
   let checking=diagnol_move_generation_with_king_safety(bitboards,3,5);
   // let moves=operations::display_bitboard(bitboards[PieceColor::W as usize][Piece::A as usize]);
   let moves=operations::display_bitboard(checking);
   println!("{}",moves);
   

}
