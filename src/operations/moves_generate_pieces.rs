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
    //calculate all the enemies pieces
    let all_enemies = bitboards[enemy_side][Piece::A as usize];

    // now proceed with the square calculation
    l_shape_attackings=l_squares(bitboards,rank,file)& all_enemies;
    // now will be calculating for the horizontal and vertical attacking positions
    horizontal_vertical_attackings = horizontal_vertical_moves(bitboards,rank,file) &all_enemies;
    // now for all the diagonal attacking sides
    diagnol_attackings=diagnol_moves(bitboards,rank,file)&all_enemies;
    // pawn attackings
    pawn_attackings=pawn_moves(bitboards,rank,file)&all_enemies;
    let board=display_bitboard(pawn_attackings);

    //king attacking
    let king_attackings=one_square_move(bitboards,rank,file)&all_enemies& bitboards[enemy_side as usize][Piece::K as usize];
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
    return result|pawn_attackings|king_attackings;
}
// converting the move from bitboard to file and rank
pub fn resolve_move(num:u64)->(usize,usize)
{
    let sqaures:u8=(63-num.leading_zeros()) as u8;
    
    for i in 0..8
    {
      let x=i*8;
      if (sqaures-x)<8
      {
        return (i as usize,(sqaures-x) as usize);
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
    let white_pieces=bitboards[PieceColor::W as usize][Piece::A as usize];
    let black_pieces=bitboards[PieceColor::B as usize][Piece::A as usize];

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