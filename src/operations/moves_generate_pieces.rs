use crate::data_structures::bitboard::*;
use crate::operations::*;

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
        while (8 > step_col+file) && ( step_row>=0) {
            let pos=((king_position >> (step_row * 8)) >> step_col);
            let pos_with = ((king_position >> (step_row * 8)) >> step_col) & all_enemies;
            if pos_with != 0 {
                diagnol_attackings |= pos_with;
                break;
    
            }
            step_col += 1;
            step_row += 1;
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

pub fn diagnol_moves(bitboards:[[u64;7];2],rank:usize,file:usize)->u64
{
 let mut all_piece=bitboards[PieceColor::W as usize][Piece::A as usize]|bitboards[PieceColor::B as usize][Piece::A as usize];
 let position:u64=1<<((rank*8) as u64 +file as u64);
 let mut diagnol_attackings:u64=0;
 all_piece=all_piece ^ position;

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
            diagnol_attackings |= pos_with;
            break;
        }
        diagnol_attackings|=pos;
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
            diagnol_attackings |= pos_with;
            break;
        }
        diagnol_attackings|=pos;
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
            diagnol_attackings |= pos_with;
            break;

        }
        diagnol_attackings |= pos;
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
            diagnol_attackings |= pos_with;
            break;
        }
        diagnol_attackings|=pos;
        step_col += 1;
        step_row += 1;
    }
}
return diagnol_attackings;
}

pub fn horizontal_vertical_moves(bitboards:[[u64;7];2],rank:usize,file:usize)->u64
{
    let mut horizontal_vertical_attackings: u64 = 0;
    let position:u64=1<<((rank*8) as u64 +file as u64);
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
                horizontal_vertical_attackings |= pos;
                break;
            }
            horizontal_vertical_attackings|=pos_with;
            step += 1;
        }

        step = 0;
        // now in the left direction
        while file+step<8 {

            let pos_with = (position << step) ;
            let pos = pos_with & all_side;
            if pos != 0 {
                horizontal_vertical_attackings |= pos;
                break;
            }
            horizontal_vertical_attackings |= pos_with;
            step += 1;
        }

        // now in the vertical upward direction
        step = 0;
        while step + rank < 8 {
            let pos_with=(position << (step * 8));
            let pos = pos_with & all_side;
            if pos != 0 {
                horizontal_vertical_attackings |= pos;
                break;
            }
            horizontal_vertical_attackings|=pos_with;
            step += 1;
        }

        // now for the vertical downward direction
        step = 0;
        while rank>=step{
            let pos_with = (position >> (step * 8)) ;
            let pos = (position >> (step * 8)) & all_side;
            if pos != 0 {
                horizontal_vertical_attackings |= pos;
                break;
            }
            horizontal_vertical_attackings |= pos_with;
            step += 1;
 
        }
    }
    return horizontal_vertical_attackings;
}

//now implementing the l shaped moves

pub fn l_squares(bitboards:[[u64;7];2],rank:usize,file:usize)->u64
{
    let mut l_shape_attackings: u64 = 0;
    let position=1<<(rank*8 +file);
    let l_squares: [[u64; 2]; 2] = [[2, 1], [2, 1]];
    

    let board=display_bitboard(position);
    println!("{}",board);
    // calculating for the knight attacking position
    if (rank+2)<8 && file >=1 
    {
        l_shape_attackings|=(position << 2*8 >> 1); 
    }
    if rank>=2 && (rank+1)<8
    {

        l_shape_attackings|=(position >> 2*8 << 1); 
    }
    if rank+2<8 && file+1<8
    {

        l_shape_attackings|=(position << 2*8 << 1); 
    }
    if rank>=1&& file>=2
    {

        l_shape_attackings|=(position >> 1*8 >> 2); 
    }
    if rank>=1 && (file+2) <8
    {

        l_shape_attackings|=(position >> 1*8 << 2); 
    }
    if (rank +1) <8 && (file+2) <8
    {

        l_shape_attackings|=(position << 1*8 << 2);
    }
    if (rank+1)<8 && file >=2
    {

        l_shape_attackings|=(position << 1*8 >> 2);
    }
    if rank>=2 && file>=1
    {

        l_shape_attackings|=(position >> 2*8 >> 1) ;
    }


    return l_shape_attackings|position;

}

pub fn one_square_move(bitboards:[[u64;7];2],rank:usize,file:usize)->u64
{
    let position =1 <<((rank*8) as u64 + file as u64);
    let mut  result:u64=0;
    if (rank+1)<8
    {
        result|=(position <<1*8 );
        if file >=1
        {
            result|=(position<<(1*8)>>1);
        }
        if file+1 <8
        {
            result|=(position<<(1*8)<<1);
        }
    }
    if rank>=1
    {
        result|=(position >> 1*8 );
        if file >=1
        {
            result|=(position>>(1*8)>>1);
        }
        if file+1 <8
        {
            result|=(position >>(1*8)<<1);
            result|=(position<<1);
        }
    }
    if file>=1
    {
        result|=(position>>1);
    }
    if file+1 <8
    {
        result|=(position<<1);
    }
    
    return result;
}

pub fn pawn_moves(bitboards:[[u64;7];2],rank:usize,file:usize)->u64
{
    let position=1<<((rank*8) as u64 +file as u64);
    let white_pieces=bitboards[PieceColor::W as usize][Piece::P as usize];
    let black_pieces=bitboards[PieceColor::B as usize][Piece::P as usize];

    let white_start:u64=65280;
    let black_start:u64=71776119061217280;
    
    let mut pawn_moves:u64=0;
    
    let all_pieces=bitboards[PieceColor::W as usize][Piece::A as usize]|bitboards[PieceColor::B as usize][Piece::A as usize];
    if (white_pieces & position & white_start)!=0
    {   let mut step=1;
        while (step<=2)
        {
        pawn_moves|=(position<<(step*8));
        if (position<<step*8)& all_pieces !=0
        {
         break;
        }
        step+=1;
        }
    }
    // for the pieces that are starting from the position second last row for the black pieces of pawn
    else if (black_pieces & position & black_start)!=0
    {   let mut step=1;
        while (step<=2)
        {
        pawn_moves|=(position>>(step*8));
        if (position>>step*8)& all_pieces !=0
        {
         break;
        }
        step+=1;
        }
    }




    {
        

        let mut pawn_attack_moves=0;
        if position & white_pieces !=0
        {
            if rank<8 && file>=1
            {
                pawn_attack_moves|=((position <<8)>>1)&black_pieces; 
            }
            if rank<8 && file<8
            {

                pawn_attack_moves|=((position <<8)<<1)&black_pieces;
            }
            if rank+1<8 && (((position<<8)& (white_pieces|black_pieces))==0)
            {
                pawn_moves|=(position<<8);
            }    
        }

        // now we will tlka of the black pieces
        if position & black_pieces!=0
        {
            if rank<8 && file>=1
            {
                pawn_attack_moves|=((position >>8)>>1)&white_pieces; 
            }
            if rank<8 && file<8
            {

                pawn_attack_moves|=((position >>8)<<1)& white_pieces;
            }
            if rank+1<8 && (((position>>8)& (white_pieces|black_pieces))==0)
            {
                pawn_moves|=(position  >>8);
            }    
        }

        pawn_moves|=pawn_attack_moves;
    }

    return pawn_moves;
}