//this implementation is not gonna improve the chess engine move generation in terms of performance 
//NOT BEING USED IN CHESS ENGINE
//INCOMPELETE
use crate::data_structures::bitboard::*;
pub fn diagnols_optimised(bitboards:[[u64;7];2],pos:u64)->u64
{
 let mut answer:u64=0;
 let result:u64=pos^(bitboards[0][6]|bitboards[1][6]);
 {
     let result=(result-pos)^result;
     answer|=(1<<(63-result.leading_zeros()));
 }

 {
    let result=(result-1)^result;
    answer|=(1<<(63-result. leading_zeros()));
 }

 return answer;

}