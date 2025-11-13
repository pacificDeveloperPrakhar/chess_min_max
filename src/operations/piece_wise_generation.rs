use crate::data_structures::bitboard::*;
use crate::operations::moves_generate_pieces::*;
use crate::operations::*;

// now first i will be generating for the queen
pub fn diagnol_move_generation_with_king_safety(mut bitboards: [[u64;7];2],rank:usize,file:usize)->u64
{
    let mut all_piece=bitboards[PieceColor::B as usize][Piece::A as usize]|bitboards[PieceColor::B as usize][Piece::A as usize];
    let position:u64=1<<((rank*8) as u64 +file as u64);
    let mut diagnol_attackings:u64=0;
    all_piece=all_piece ^ position;
   let side= if (bitboards[PieceColor::W as usize][Piece::A as usize]&position)!=0{0}else{1};
    // now for all the diagonal attacking sides
    {
       let mut step_row = 0;
       let mut step_col = 0;
   
       // upper right
       while ((step_row + rank) < 8) && (step_col <= file) {
           print!("step_col {}",step_col+rank);
           let pos =((position << (step_row * 8)) >> step_col);
           let pos_with = ((position << (step_row * 8)) >> step_col) & all_piece;
           if pos_with != 0 {
            if is_king_safe(bitboards,position,pos_with,side)
            {
                diagnol_attackings|=pos;
            }
               break;
           }
           if is_king_safe(bitboards,position,pos,side)
           {
               diagnol_attackings|=pos;
           }
           step_col += 1;
           step_row += 1;
       }
   
       step_col = 0;
       step_row = 0;
   
       // upper left
       while (step_col+file<8) && (step_row + rank < 8) {
           let pos = ((position << (step_row * 8)) << step_col);
           let pos_with = ((position << (step_row * 8)) << step_col) & all_piece;
           if pos_with != 0 {
            if is_king_safe(bitboards,position,pos_with,side)
            {
                diagnol_attackings|=pos;
            }
               break;
           }
           if is_king_safe(bitboards,position,pos,side)
           {
               diagnol_attackings|=pos;
           }
           step_col += 1;
           step_row += 1;
       }
   
       step_col = 0;
       step_row = 0;
   
       // bottom right
       while (step_row<=rank) && ( step_col<=file) {
           let pos=((position >> (step_row * 8)) >> step_col);
           let pos_with = ((position >> (step_row * 8)) >> step_col) & all_piece;
           if pos_with != 0 {
            if is_king_safe(bitboards,position,pos_with,side)
            {
                diagnol_attackings|=pos;
            }
               break;
           }
           if is_king_safe(bitboards,position,pos,side)
           {
               diagnol_attackings|=pos;
           }
           step_col += 1;
           step_row += 1;
       }
   
       step_col = 0;
       step_row = 0;
   
       // bottom left
       while (file + step_col < 8) && (rank >= step_row) {
           let pos=((position >> (step_row * 8)) << step_col);
           let pos_with = pos & all_piece;
           if pos_with != 0 {
            if is_king_safe(bitboards,position,pos_with,side)
            {
                diagnol_attackings|=pos;
            }
               break;
           }
           if is_king_safe(bitboards,position,pos,side)
           {
               diagnol_attackings|=pos;
           }
           step_col += 1;
           step_row += 1;
       }
   }
   let my_side=bitboards[side][Piece::A as usize];
   return diagnol_attackings&(diagnol_attackings^my_side) ;
}
//now the horizontal veritcal position

pub fn piece_wise_horizontal_vertical_moves(mut bitboards:[[u64;7];2],rank:usize,file:usize)->u64
{
    let mut horizontal_vertical_attackings: u64 = 0;
    let position:u64=1<<((rank*8) as u64 +file as u64);
    let side= if (bitboards[PieceColor::W as usize][Piece::A as usize]&position)!=0{0}else{1};
    // now will be calculating for the horizontal and vertical attacking positions
    {
        
        let mut all_side = bitboards[PieceColor::W as usize][Piece::A as usize]|bitboards[PieceColor::B as usize][Piece::A as usize];
        all_side=all_side^position;
        let mut step = 0;

        // in right direction
        while step<=file {
            let pos_with= (position >> step) ;
            let pos=pos_with& all_side;
            if pos != 0 {
                if is_king_safe(bitboards,position,pos,side)
                {
                    horizontal_vertical_attackings |= pos;
                }
                break;
            }
            if is_king_safe(bitboards,position,pos_with,side)
            {
                horizontal_vertical_attackings |= pos_with;
            }
            step += 1;
        }

        step = 0;
        // now in the left direction
        while file+step<8 {

            let pos_with = (position << step) ;
            let pos = pos_with & all_side;
            if pos != 0 {
                if is_king_safe(bitboards,position,pos,side)
                {
                    horizontal_vertical_attackings |= pos;
                }
                break;
            }
            if is_king_safe(bitboards,position,pos_with,side)
            {
                horizontal_vertical_attackings |= pos_with;
            }
            step += 1;
        }

        // now in the vertical upward direction
        step = 0;
        while step + rank < 8 {
            let pos_with=(position << (step * 8));
            let pos = pos_with & all_side;
            if pos != 0 {
                if is_king_safe(bitboards,position,pos,side)
                {
                    horizontal_vertical_attackings |= pos;
                }
                break;
            }
            if is_king_safe(bitboards,position,pos_with,side)
            {
                horizontal_vertical_attackings |= pos_with;
            }
            step += 1;
        }

        // now for the vertical downward direction
        step = 0;
        while rank>=step{
            let pos_with = (position >> (step * 8)) ;
            let pos = (position >> (step * 8)) & all_side;
            if pos != 0 {
                if is_king_safe(bitboards,position,pos,side)
                {
                    horizontal_vertical_attackings |= pos;
                }
                break;
            }
            if is_king_safe(bitboards,position,pos_with,side)
            {
                horizontal_vertical_attackings |= pos_with;
            }
            step += 1;
 
        }
    }
    let my_side=bitboards[side][Piece::A as usize];
    return horizontal_vertical_attackings&(horizontal_vertical_attackings^my_side) ;
}
// ========================================Now L shaped positions==============================================================================
pub fn piece_wise_l_squares(mut bitboards: [[u64; 7]; 2], rank: usize, file: usize) -> u64 {
    let mut l_shape_attackings: u64 = 0;
    let position = 1 << (rank * 8 + file);
    let l_squares: [[u64; 2]; 2] = [[2, 1], [2, 1]];
    let side= if (bitboards[PieceColor::W as usize][Piece::A as usize]&position)!=0{0}else{1};

    let board = display_bitboard(position);
    println!("{}", board);

    // calculating for the knight attacking positions

    // (rank + 2, file - 1)
    if (rank + 2) < 8 && file >= 1 {
        if is_king_safe(bitboards, position, (position << 2 * 8 >> 1),side) {
            l_shape_attackings |= (position << 2 * 8 >> 1);
        }
    }

    // (rank - 2, file + 1)
    if rank >= 2 && (file + 1) < 8 {
        if is_king_safe(bitboards, position, (position >> 2 * 8 << 1),side) {
            l_shape_attackings |= (position >> 2 * 8 << 1);
        }
    }

    // (rank + 2, file + 1)
    if (rank + 2) < 8 && (file + 1) < 8 {
        if is_king_safe(bitboards, position, (position << 2 * 8 << 1),side) {
            l_shape_attackings |= (position << 2 * 8 << 1);
        }
    }

    // (rank - 1, file - 2)
    if rank >= 1 && file >= 2 {
        if is_king_safe(bitboards, position, (position >> 1 * 8 >> 2),side) {
            l_shape_attackings |= (position >> 1 * 8 >> 2);
        }
    }

    // (rank - 1, file + 2)
    if rank >= 1 && (file + 2) < 8 {
        if is_king_safe(bitboards, position, (position >> 1 * 8 << 2),side) {
            l_shape_attackings |= (position >> 1 * 8 << 2);
        }
    }

    // (rank + 1, file + 2)
    if (rank + 1) < 8 && (file + 2) < 8 {
        if is_king_safe(bitboards, position, (position << 1 * 8 << 2),side) {
            l_shape_attackings |= (position << 1 * 8 << 2);
        }
    }

    // (rank + 1, file - 2)
    if (rank + 1) < 8 && file >= 2 {
        if is_king_safe(bitboards, position, (position << 1 * 8 >> 2),side) {
            l_shape_attackings |= (position << 1 * 8 >> 2);
        }
    }

    // (rank - 2, file - 1)
    if rank >= 2 && file >= 1 {
        if is_king_safe(bitboards, position, (position >> 2 * 8 >> 1),side) {
            l_shape_attackings |= (position >> 2 * 8 >> 1);
        }
    }
    let my_side=bitboards[side][Piece::A as usize];
    return l_shape_attackings&(l_shape_attackings^my_side) ;
}
//============================================Now for the pawn movement moves===========================================================================
pub fn pawn_moves(mut bitboards: [[u64; 7]; 2], rank: usize, file: usize) -> u64 {
    let position = 1 << ((rank * 8) as u64 + file as u64);
    let white_pieces = bitboards[PieceColor::W as usize][Piece::A as usize];
    let black_pieces = bitboards[PieceColor::B as usize][Piece::A as usize];

    let white_start: u64 = 65280;
    let black_start: u64 = 71776119061217280;

    let mut pawn_moves: u64 = 0;
    let all_pieces = white_pieces | black_pieces;
    let side= if (bitboards[PieceColor::W as usize][Piece::A as usize]&position)!=0{0}else{1};

    // --- White pawn double-step start ---
    if (white_pieces & position & white_start) != 0 {
        let mut step = 1;
        while step <= 2 {
            let new_pos = position << (step * 8);
            if is_king_safe(bitboards,  position, new_pos,side) {
                pawn_moves |= new_pos;
            }
            if (new_pos & all_pieces) != 0 {
                break;
            }
            step += 1;
        }
    }

    // --- Black pawn double-step start ---
    else if (black_pieces & position & black_start) != 0 {
        let mut step = 1;
        while step <= 2 {
            let new_pos = position >> (step * 8);
            if is_king_safe(bitboards, position, new_pos,side) {
                pawn_moves |= new_pos;
            }
            if (new_pos & all_pieces) != 0 {
                break;
            }
            step += 1;
        }
    }

    // --- Pawn attack & single step moves ---
    {
        let mut pawn_attack_moves = 0;

        // ---- White pawn logic ----
        if (position & white_pieces) != 0 {
            // Diagonal left attack
            if rank < 8 && file >= 1 {
                let new_pos = (position << 8) >> 1;
                if (new_pos & black_pieces) != 0 && is_king_safe(bitboards,position, new_pos,side) {
                    pawn_attack_moves |= new_pos;
                }
            }

            // Diagonal right attack
            if rank < 8 && file < 8 {
                let new_pos = (position << 8) << 1;
                if (new_pos & black_pieces) != 0 && is_king_safe(bitboards,  position, new_pos,side) {
                    pawn_attack_moves |= new_pos;
                }
            }

            // Forward move (single)
            if rank + 1 < 8 {
                let new_pos = position << 8;
                if ((new_pos & all_pieces) == 0) && is_king_safe(bitboards, position, new_pos,side) {
                    pawn_moves |= new_pos;
                }
            }
        }

        // ---- Black pawn logic ----
        if (position & black_pieces) != 0 {
            // Diagonal left attack
            if rank < 8 && file >= 1 {
                let new_pos = (position >> 8) >> 1;
                if (new_pos & white_pieces) != 0 && is_king_safe(bitboards,  position, new_pos,side) {
                    pawn_attack_moves |= new_pos;
                }
            }

            // Diagonal right attack
            if rank < 8 && file < 8 {
                let new_pos = (position >> 8) << 1;
                if (new_pos & white_pieces) != 0 && is_king_safe(bitboards, position, new_pos,side) {
                    pawn_attack_moves |= new_pos;
                }
            }

            // Forward move (single)
            if rank + 1 < 8 {
                let new_pos = position >> 8;
                if ((new_pos & all_pieces) == 0) && is_king_safe(bitboards, position, new_pos,side) {
                    pawn_moves |= new_pos;
                }
            }
        }

        pawn_moves |= pawn_attack_moves;
    }

    pawn_moves
}

//============================================Implementation for the king moves========================================================================

pub fn one_square_move(mut bitboards: [[u64; 7]; 2], rank: usize, file: usize) -> u64 {
    let position = 1 << ((rank * 8) as u64 + file as u64);
    let mut result: u64 = 0;
    let side= if (bitboards[PieceColor::W as usize][Piece::A as usize]&position)!=0{0}else{1};
    // upward moves
    if (rank + 1) < 8 {
        let target = position << 1 * 8;
        if is_king_safe(bitboards, position, target,side) {
            result |= target;
        }

        if file >= 1 {
            let target = (position << (1 * 8)) >> 1;
            if is_king_safe(bitboards, position, target,side) {
                result |= target;
            }
        }

        if file + 1 < 8 {
            let target = (position << (1 * 8)) << 1;
            if is_king_safe(bitboards, position, target,side) {
                result |= target;
            }
        }
    }

    // downward moves
    if rank >= 1 {
        let target = position >> 1 * 8;
        if is_king_safe(bitboards, position, target,side) {
            result |= target;
        }

        if file >= 1 {
            let target = (position >> (1 * 8)) >> 1;
            if is_king_safe(bitboards, position, target,side) {
                result |= target;
            }
        }

        if file + 1 < 8 {
            let target = (position >> (1 * 8)) << 1;
            if is_king_safe(bitboards, position, target,side) {
                result |= target;
            }

            // horizontal move (probably leftover from your earlier logic)
            let target = position << 1;
            if is_king_safe(bitboards, position, target,side) {
                result |= target;
            }
        }
    }

    // left
    if file >= 1 {
        let target = position >> 1;
        if is_king_safe(bitboards, position, target,side) {
            result |= target;
        }
    }

    // right
    if file + 1 < 8 {
        let target = position << 1;
        if is_king_safe(bitboards, position, target,side) {
            result |= target;
        }
    }

    let my_side=bitboards[side][Piece::A as usize];
    return result&(result^my_side) ;
}

// ===========================================Implementation to check the safety of king========================================================================
pub fn is_king_safe(mut bitboards:[[u64;7];2],piece_position:u64,move_position:u64,side:usize)->bool
{
 bitboards[side][Piece::A as usize]^=piece_position;
 bitboards[side][Piece::A as usize]|=move_position;
 let side = if side==0 {'w'} else {'b'};
 if is_king_checked(bitboards,side) ==0
 {
    return true;
 }
 return false;
}