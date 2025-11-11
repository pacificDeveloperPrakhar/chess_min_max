use crate::data_structures::bitboard::*;

pub fn is_king_checked(bitboards: [[u64; 7]; 2], side: char) -> u64 {
    let rank;
    let file;
    let king_position =
        bitboards[PieceColor::from(side) as usize][Piece::from('K') as usize];
    (rank, file) = resolve_move(king_position);

    let mut diagnol_attackings: u64 = 0;
    let mut l_shape_attackings: u64 = 0;
    let mut horizontal_vertical_attackings: u64 = 0;
    let mut pawn_attackings: u64 = 0;

    // calculate the enemy side bitboard
    let enemy_side: usize = 1 - (PieceColor::from(side) as usize);
    println!("enemy side {}",enemy_side);
    println!("my side {}",side);
    let all_enemies = bitboards[enemy_side][Piece::A as usize];
    // now proceed with the square calculation
    let l_squares: [[u64; 2]; 2] = [[2, 1], [2, 1]];

    // calculating for the knight attacking position
    for i in 0..2 {
        for j in 0..2 {
            let rank_to_add = l_squares[0][i];
            let file_to_add = l_squares[1][j];

            // now reverse the squares among the rank and file
            let another_rank_to_add = l_squares[1][i];
            let another_file_to_add = l_squares[0][j];

            // now add the calculated squares to the attacking position info containing number
            let pos1 = (1 << (rank * 8 + file)) >> (rank_to_add * 8) << file_to_add;
            let pos2 = (1 << (rank * 8 + file)) << (another_rank_to_add * 8) >> another_file_to_add;

            if pos1 & all_enemies != 0 {
                l_shape_attackings |= pos1;
            }
            if pos2 & all_enemies != 0 {
                l_shape_attackings |= pos2;
            }
        }
    }

    // now will be calculating for the horizontal and vertical attacking positions
    {
        let enemy_side: usize = 1 - (PieceColor::from(side) as usize);
        let all_enemies = bitboards[enemy_side][Piece::A as usize];
        let mut step = 0;

        // in right direction
        while step + file < 8 {
            horizontal_vertical_attackings |= (king_position >> step) & all_enemies;
            if horizontal_vertical_attackings != 0 {
                break;
            }
            step += 1;
        }

        step = 0;
        // now in the left direction
        while file >= step {
            let pos1 = (king_position << step) & all_enemies;
            if pos1 != 0 {
                horizontal_vertical_attackings |= pos1;
                break;
            }
            step += 1;
            if step > file {
                break;
            }
        }

        // now in the vertical direction
        step = 0;
        while step + rank < 8 {
            let pos = (king_position << (step * 8)) & all_enemies;
            if pos != 0 {
                horizontal_vertical_attackings |= pos;
                break;
            }
            step += 1;
        }

        // now for the vertical downward direction
        step = 0;
        while rank >= step {
            let pos = (king_position >> (step * 8)) & all_enemies;
            if pos != 0 {
                horizontal_vertical_attackings |= pos;
                break;
            }
            step += 1;
            if step > rank {
                break;
            }
        }
    }

    // now for all the diagonal attacking sides
    {
        let mut step_row = 0;
        let mut step_col = 0;

        // upper right
        while (step_row + rank < 8) && (file + step_col < 8) {
            let pos = ((king_position << (step_row * 8)) >> step_col) & all_enemies;
            if pos != 0 {
                diagnol_attackings |= pos;
                break;
            }
            step_col += 1;
            step_row += 1;
        }

        step_col = 0;
        step_row = 0;

        // upper left
        while (file >= step_col) && (step_row + rank < 8) {
            let pos = ((king_position << (step_row * 8)) << step_col) & all_enemies;
            if pos != 0 {
                diagnol_attackings |= pos;
                break;
            }
            step_col += 1;
            step_row += 1;
            if step_col > file {
                break;
            }
        }

        step_col = 0;
        step_row = 0;

        // bottom right
        while (file >= step_col) && (rank >= step_row) {
            let pos = ((king_position >> (step_row * 8)) >> step_col) & all_enemies;
            if pos != 0 {
                diagnol_attackings |= pos;
                break;
            }
            step_col += 1;
            step_row += 1;
            if step_col > file || step_row > rank {
                break;
            }
        }

        step_col = 0;
        step_row = 0;

        // bottom left
        while (file + step_col < 8) && (rank >= step_row) {
            let pos = ((king_position >> (step_row * 8)) << step_col) & all_enemies;
            if pos != 0 {
                diagnol_attackings |= pos;
                break;
            }
            step_col += 1;
            step_row += 1;
        }
    }

    // pawn attackings
    {
        let pos1 = king_position << 8 << 1;
        let pos2 = king_position << 8 << 1;
        let res = (pos1 | pos2) & all_enemies;
        if res != 0 {
            pawn_attackings |= res;
        }
    }
    let mut result=0;
    if pawn_attackings &bitboards[enemy_side][Piece::P as usize]!=0
    {
        result|=(pawn_attackings &bitboards[enemy_side][Piece::P as usize]);
    }
    if horizontal_vertical_attackings &(bitboards[enemy_side][Piece::Q as usize]|bitboards[enemy_side][Piece::R as usize]) !=0
    {
        result|=(horizontal_vertical_attackings &(bitboards[enemy_side][Piece::Q as usize]|bitboards[enemy_side][Piece::R as usize]))
    }

    if l_shape_attackings&(bitboards[enemy_side][Piece::N as usize])!=0
    {
        result|=(l_shape_attackings&(bitboards[enemy_side][Piece::N as usize]));
    }
    if diagnol_attackings&(bitboards[enemy_side][Piece::Q as usize]|bitboards[enemy_side][Piece::B as usize]) !=0
    {
        result|=(diagnol_attackings&(bitboards[enemy_side][Piece::Q as usize]|bitboards[enemy_side][Piece::B as usize]));
    }
    return result;
}
// converting the move from bitboard to file and rank
pub fn resolve_move(num:u64)->(u8,u8)
{
    let sqaures:u8=(63-num.leading_zeros()) as u8;
    
    for i in 0..8
    {
      let x=i*8;
      if (sqaures-x)<8
      {
        return (i,sqaures-x);
      }
    }
    return (0,0);
}