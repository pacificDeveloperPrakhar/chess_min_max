pub const WQ:u8=1;
pub const WR:u8=2;
pub const WB:u8=3;
pub const WK:u8=4;
pub const WN:u8=5;
pub const WP:u8=6;

// Black pieces
pub const BQ:u8=7;
pub const BR:u8=8;
pub const BB:u8=9;
pub const BK:u8=10;
pub const BN:u8=11;
pub const BP:u8=12;

//last constatnt for the empty squares
pub const ES:u8=0;

// now encode the board prepresentation data structure
pub static mut BOARD: [[u8;8];8] = [
    [BR, BN, BB, BQ, BK, BB, BN, BR],
    [BP, BP, BP, BP, BP, BP, BP, BP],
    [ES, ES, ES, ES, ES, ES, ES, ES],
    [ES, ES, ES, ES, ES, ES, ES, ES],
    [ES, ES, ES, ES, ES, ES, ES, ES],
    [ES, ES, ES, ES, ES, ES, ES, ES],
    [WP, WP, WP, WP, WP, WP, WP, WP],
    [WR, WN, WB, WQ, WK, WB, WN, WR]
];