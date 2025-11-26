#![allow(warnings)]
pub mod data_structures;
pub mod operations;
use data_structures::bitboard::*;
use operations::*;
use operations::moves_generate_pieces::*;
use operations::piece_wise_generation::*;
use operations::bit_operation::*;
use tokio_stream::Stream;
use std::pin::Pin;
// fn main() {
//    let ffn=String::from("8/8/8/8/5P2/8/1P6/2P5 w - - 0 1");
//    let bitboards=operations::init_board(&ffn);
//    let state=State::init();
//    // let moves=pawn_moves(bitboards,1,2);
//    // let moves=operations::display_bitboard(moves);
//    // println!("{}",moves);
//    // println!("{}",bitboards[PieceColor::W as usize][Piece::P as usize]);
//    // println!("{}",bitboards[PieceColor::B as usize][Piece::P as usize]);
//    // let moves=operations::display_bitboard(bitboards[PieceColor::W as usize][Piece::A as usize]);
//    let moves=operations::display_bitboard(bitboards[0][0]);
//    let epd:String=operations::get_modified_epd(bitboards[1][0],'-').iter().collect();
//    println!("{}",epd);
   

// }

mod idl
{
   include!("./chess_engine.rs");
}



// // Global shared game state — initialized lazily at runtime
// static GAME_STATE: LazyLock<Arc<Mutex<State>>> = LazyLock::new(|| {
   //     Arc::new(Mutex::new(State::init()))
   // });
   use std::sync::{Arc, Mutex, LazyLock};

   use idl::chess_service_server::ChessService;

   pub struct EngineServer
   {
      root:Arc<Mutex<State>>,
   }

   impl EngineServer
   {
      pub fn new()->Self
      {
         return Self
         {
            root:Arc::new(Mutex::new(State::init([[0_u64;7];2])))
         };
      }
   }


   use tokio_stream::iter;
   use tonic::{Request, Response, Status};
   use crate::idl::{LoadBoardReq};

   #[tonic::async_trait]
   impl idl::chess_service_server::ChessService for EngineServer {

      async fn load_board(
         &self,
         request: Request<LoadBoardReq>,
      ) -> Result<Response<idl::Ffnboard>, Status> {
         let req:idl::LoadBoardReq=request.into_inner();
         let bitboards=init_board(&(req.board));
         let root=State::init(bitboards);

         let mut root=self.root.lock().unwrap();
         root.bitboards=bitboards;
         println!("loading_state {:?}",bitboards);
         let ffnboard=bitboards_to_modified_epd(bitboards);

         return Ok(Response::new(idl::Ffnboard{
            board:ffnboard.iter().collect()
         }));
         
      }

      async fn generate_moves(&self,request:tonic::Request<idl::GenMoveReq>)->Result<tonic::Response<idl::Ffnboard>,tonic::Status>
      {
      let req:idl::GenMoveReq=request.into_inner();
      let mut bitboards=self.root.lock().unwrap().bitboards;
      let board=operations::generate_moves(bitboards,req.rank as usize,req.file as usize);
      let result=idl::Ffnboard
         {
         board:get_modified_epd(board,'-').iter().collect(),
         };
      
         return Result::Ok(tonic::Response::new(result));
      }
// ==================================================================================================================================================================================
// async fn make_move(
//     &self,
//     request: Request<idl::MakeMoveReq>,
// ) -> Result<Response<idl::Ffnboard>, Status> {
//     let req = request.into_inner();

//     // 1. Lock the root state MUTABLY
//     let mut guard = self.root.lock().unwrap();

//     // 2. Make a mutable pointer to the root node
//     let mut node: &mut State = &mut *guard;

//     // 3. Traverse to the last state
//     while let Some(next) = node.next_state.as_mut() {
//         node = next;
//     }

//     // 4. Clone the previous bitboards
//     let mut bitboards = node.bitboards;

//     // 5. Apply the move
//     let c = req.piece_color as usize;
//     let p = req.piece as usize;

//     bitboards[c][p] ^= req.piece_position;
//     bitboards[c][p] |= req.move_position;

//     // 6. Build new state
//     let new_state = State {
//         active_color: 1 - node.active_color,
//         castling: 0b1111,
//         halfmove_clock: 0,
//         en_passant: None,
//         fullmove_number: 0,
//         zobrist_key: 0,
//         material: 0,
//         phase: 618,
//         next_move: 0,
//         is_checked: false,
//         bitboards,
//         children: Vec::new(),
//         next_state: None,
//         evaluation: 0,
//     };

//     // 7. Append new state as a child
//     node.next_state = Some(Box::new(new_state));

//     // 8. Build response
//     let ffnboard: String = bitboards_to_modified_epd(bitboards).iter().collect();

//     Ok(Response::new(idl::Ffnboard { board: ffnboard }))
// }
// ==================================================================================================================================================================================
async fn make_move(
   &self,
   request: Request<idl::MakeMoveReq>,
) -> Result<Response<idl::Ffnboard>, Status> {
   let req = request.into_inner();

   //  Lock the game state
   let mut guard = self.root.lock().unwrap();
   

   // now what if we the person did a double move ,meaning if the same side which previous made a move ,again 
   // made a new move ,we have to invlaidate that
   if (*guard).active_color as u32 == req.piece_color 
   {

      let ffnboard = bitboards_to_modified_epd((*guard).bitboards)
      .iter()
      .collect();
      
      return  Ok(Response::new(idl::Ffnboard { board: ffnboard })) 
   }
   // we will first dref the MutexGuard object which is returned then dref it 
   // get the actual data that is the state then convert it to the reference this reference 
   // will be used stored as the raw mutable pointer
   let mut current: *mut State =  &mut * guard;
   let mut count=0;
   unsafe {
      //  first deref the current then get the state and then convert it back to the refrence
       while let Some(next) = &mut (*current).next_state {
         // there are two * because first time we are getting the Box<State> from the next then
         // getting the actual next state,Box implements the deref trait so we can deref the Box wrapped data
         // to get the data inside 
           current = &mut **next;
           println!(" here is the stuff{:?}",current);
           count+=1;
       }
      // clone the bitboards
       let mut bitboards = (*current).bitboards;

      // now we will update the bitboard on the basis of the input from the user
       let c = req.piece_color as usize;
       let p = req.piece as usize;

       bitboards[c][p] ^= req.piece_position;
       bitboards[c][p] |= req.move_position;
       bitboards[c][6] ^= req.piece_position;
       bitboards[c][6] |= req.move_position;

       for i in 0..7
       {
         if (bitboards[1-c][p]&req.move_position)!=0
         {
            bitboards[1-c][p]^=req.move_position;
         }

       }
       println!("move making {:?}",bitboards);
      //  create a entirely new state ,successor the last state corresponding the last move
       let new_state = State {
           active_color: 1 - (*current).active_color,
           castling: 0b1111,
           halfmove_clock: 0,
           en_passant: None,
           fullmove_number: 0,
           zobrist_key: 0,
           material: 0,
           phase: 618,
           next_move: 0,
           is_checked: false,
           bitboards,
           children: Vec::new(),
           next_state: None,
           evaluation: 0,
       };

       // Attach new node
       (*current).next_state = Some(Box::new(new_state));
   }

   // Response text
   let ffnboard = bitboards_to_modified_epd(unsafe { (*current).bitboards })
       .iter()
       .collect();

   Ok(Response::new(idl::Ffnboard { board: ffnboard }))
}

      async fn get_boards(&self,request:tonic::Request<()>)->Result<tonic::Response<idl::Ffnboard>,tonic::Status>
      {
            return Result::Ok(tonic::Response::new(idl::Ffnboard
         {
               board:String::from("epd")
         }));
      }
   }



#[tokio::main]
async fn main()
{
    let addr: std::net::SocketAddr = "[::1]:50051".parse().unwrap();
    let chess_engine=EngineServer::new();
    use tonic::transport::Server;

    Server::builder()
        .add_service(idl::chess_service_server::ChessServiceServer::new(chess_engine))
        .serve(addr)
        .await
        .unwrap();
}