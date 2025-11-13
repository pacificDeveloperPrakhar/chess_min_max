pub mod data_structures;
pub mod operations;
use data_structures::bitboard::*;
use operations::*;
use operations::moves_generate_pieces::*;
use operations::piece_wise_generation::*;
fn main() {
   let ffn=String::from("rnbqkb1r/ppp2ppp/4p3/3p4/1Q6/3P1N1P/PPP1PnP1/RNB1KB1R w KQkq - 0 1");
   let bitboards=operations::init_board(&ffn);
   // let moves=pawn_moves(bitboards,1,2);
   // let moves=operations::display_bitboard(moves);
   // println!("{}",moves);
   // println!("{}",bitboards[PieceColor::W as usize][Piece::P as usize]);
   // println!("{}",bitboards[PieceColor::B as usize][Piece::P as usize]);
   let checking=generate_moves(bitboards,0,5);
   // let moves=operations::display_bitboard(bitboards[PieceColor::W as usize][Piece::A as usize]);
   let moves=operations::display_bitboard(checking);
   println!("{}",moves);
   

}
