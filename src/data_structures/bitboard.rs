use std::convert::From;
#[repr(usize)]
pub enum Piece
{
     P=0,
     N=1,
     B=2,
     R=3,
     K=4,
     Q=5,
     A=6,

}
// enum corresponding to the color of the pieces
#[repr(usize)]
pub enum PieceColor
{
    W=0,
    B=1
}
// implementing the From trait for the piece enum which has a function called from takes an input of one data type and then return
// the data type for what we have specified for 
impl From<char> for Piece{
    fn from(c:char)->Self
    {
     match c{
        'Q'|'q'=>Piece::Q,
        'K'|'k'=>Piece::K,
        'B'|'b'=>Piece::B,
        'R'|'r'=>Piece::R,
        'P'|'p'=>Piece::P,
        'N'|'n'=>Piece::N,
         _=> Piece::A
     }
    }
} 

impl From<char> for PieceColor
{
 fn from(c:char)->Self
 {
    match c
    {
        'W'|'w'=>PieceColor::W,
        'B'|'b'=>PieceColor::B,
        _=>PieceColor::W
    }
 }
}