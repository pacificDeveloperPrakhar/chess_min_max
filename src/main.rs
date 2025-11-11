pub mod data_structures;
pub mod operations;
use data_structures::bitboard::*;
use operations::moves_generate_pieces::*;
fn main() {
   let ffn=String::from("8/8/4N3/8/2qK1n2/2Q5/8/8 w - - 0 1");
   let bitboards:[[u64;7];2]=operations::init_board(&ffn);
   let king_attacks=operations::moves_generate_pieces::is_king_checked(bitboards,'b');
   let  (rank,file)=operations::moves_generate_pieces::resolve_move(bitboards[PieceColor::B as usize][Piece::K as usize]);
   println!("{}",rank);
   println!("{}",file);
   let res=operations::display_bitboard(king_attacks);
   println!("is king checked {}",res);

}
