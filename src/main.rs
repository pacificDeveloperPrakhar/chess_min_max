#![allow(warnings)]
pub mod data_structures;
pub mod operations;
use data_structures::bitboard::*;
use operations::*;
use operations::moves_generate_pieces::*;
use operations::piece_wise_generation::*;
use operations::bit_operation::*;
fn main() {
   let ffn=String::from("7Q/6q1/8/4b3/8/2B5/1P6/B7 w - - 0 1");
   let bitboards=operations::init_board(&ffn);
   let state=State::init();
   // let moves=pawn_moves(bitboards,1,2);
   // let moves=operations::display_bitboard(moves);
   // println!("{}",moves);
   // println!("{}",bitboards[PieceColor::W as usize][Piece::P as usize]);
   // println!("{}",bitboards[PieceColor::B as usize][Piece::P as usize]);
   let checking=diagnols_optimised(bitboards,1<<(3*8+4) as u64);
   // let moves=operations::display_bitboard(bitboards[PieceColor::W as usize][Piece::A as usize]);
   let moves=operations::display_bitboard(checking);
   println!("{}",moves);
   

}
