#![allow(warnings)]
pub mod data_structures;
pub mod operations;
use data_structures::bitboard::*;
use operations::*;
use operations::moves_generate_pieces::*;
use operations::piece_wise_generation::*;
use operations::bit_operation::*;
fn main() {
   let ffn=String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
   let bitboards=operations::init_board(&ffn);
   let state=State::init();
   // let moves=pawn_moves(bitboards,1,2);
   // let moves=operations::display_bitboard(moves);
   // println!("{}",moves);
   // println!("{}",bitboards[PieceColor::W as usize][Piece::P as usize]);
   // println!("{}",bitboards[PieceColor::B as usize][Piece::P as usize]);
   // let moves=operations::display_bitboard(bitboards[PieceColor::W as usize][Piece::A as usize]);
   let moves=operations::display_bitboard(bitboards[0][0]);
   println!("{}",bitboards[0][0]/(2^16));
   

}

