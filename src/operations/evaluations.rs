use crate::data_structures::bitboard::*;

// now this function will be returning penlaty or reward when the piece will be moving
pub fn evaluate_move(bitboards:[[u64;7];2],phase:usize,piece_position:usize,move_position:usize,piece_color:usize,piece_type:usize)->i64
{
    let mut piece_square_score=PSQT[piece_type][phase][piece_color][piece_position];
    let mut move_square_score=PSQT[piece_type][phase][piece_color][move_position];

    let mut piece_enemy_score=0;
    for i in 0..2
    {
        for j in 0..7
        {
            if (bitboards[i][j]&(1<<piece_position))!=0
            {
             piece_enemy_score=PSQT[j][phase][i][move_position];
             break;
            }
        }
    }
    
    return move_square_score-piece_square_score-(piece_enemy_score);
}