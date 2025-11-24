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
//    let ffn=String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
//    let bitboards=operations::init_board(&ffn);
//    let state=State::init();
//    // let moves=pawn_moves(bitboards,1,2);
//    // let moves=operations::display_bitboard(moves);
//    // println!("{}",moves);
//    // println!("{}",bitboards[PieceColor::W as usize][Piece::P as usize]);
//    // println!("{}",bitboards[PieceColor::B as usize][Piece::P as usize]);
//    // let moves=operations::display_bitboard(bitboards[PieceColor::W as usize][Piece::A as usize]);
//    let moves=operations::display_bitboard(bitboards[0][0]);
//    println!("{}",bitboards[0][0]/(2^16));
   

// }

mod idl
{
   include!("./chess_engine.rs");
}


use std::sync::{Arc, Mutex, LazyLock};

// Global shared game state — initialized lazily at runtime
static GAME_STATE: LazyLock<Arc<Mutex<State>>> = LazyLock::new(|| {
    Arc::new(Mutex::new(State::init()))
});

use idl::chess_service_server::ChessService;

pub struct EngineServer;

impl EngineServer
{
   pub fn new()->Self
   {
      return EngineServer;
   }
}


use tokio_stream::iter;
use tonic::{Request, Response, Status};
use crate::idl::{LoadBoardReq};

#[tonic::async_trait]
impl idl::chess_service_server::ChessService for EngineServer {
    type LoadBoardStream = Pin<Box<dyn Stream<Item = Result<idl::Bitboard, Status>> + Send + 'static>>;

    async fn load_board(
        &self,
        request: Request<LoadBoardReq>,
    ) -> Result<Response<Self::LoadBoardStream>, Status> {

        let data: LoadBoardReq = request.into_inner();

        // Your board initializer
        let bitboards: [[u64; 7]; 2] = init_board(&data.board);

        // Build the stream data
        let mut stream_data: Vec<Result<idl::Bitboard, Status>> = Vec::new();

        for j in 0..bitboards.len() {
            for i in 0..bitboards[j].len() {
                let bitboard=idl::Bitboard{
                  board:bitboards[j][i]
                };
                stream_data.push(Ok(bitboard));
            }
        }

        // Convert vector into a stream
        let stream = iter(stream_data);

        Ok(Response::new(Box::pin(stream)))
    }

    async fn generate_moves(&self,request:tonic::Request<idl::GenMoveReq>)->Result<tonic::Response<idl::Bitboard>,tonic::Status>
    {
     let req:idl::GenMoveReq=request.into_inner();
     let white_boards=req.white_bitboard;
     let black_boards=req.black_bitboard;
     let mut bitboards=[[0u64;7];2];
     {
        let mut j=0;

        for i in 0..7
        {
            bitboards[0][i]=white_boards[i];
            bitboards[1][i]=black_boards[i];
        }
     }

      let result=idl::Bitboard
      {
        board:operations::generate_moves(bitboards,req.rank as usize,req.file as usize),
      };
        return Result::Ok(tonic::Response::new(result));
    }

    async fn make_move(&self, request:tonic::Request<idl::MakeMoveReq>)->Result<tonic::Response<idl::Bitboards>,tonic::Status>
    {
        let req:idl::MakeMoveReq=request.into_inner();
        let mut bitboards:[[u64;7];2]=[[0u64;7];2];

        bitboards[0] = req.white_bitboard.clone().try_into().expect("white must be len 7");
        bitboards[1] = req.black_bitboard.clone().try_into().expect("black must be len 7");

        
        
        
        for i in 0..2
        {
            for j in 0..7
            {
                if (bitboards[i as usize][j as usize]&req.move_position)!=0
                {
                    bitboards[i as usize][j as usize]^=req.move_position;
                    break;
                }
            }
        }

        bitboards[req.piece_color as usize][req.piece as usize]^=req.piece_position;
        bitboards[req.piece_color as usize][req.piece as usize]|=req.move_position ;
        let result =idl::Bitboards{
            white_bitboard:bitboards[0].to_vec(),
            black_bitboard:bitboards[1].to_vec()
        };
        return Result::Ok(tonic::Response::new(result));
    }

    async fn get_boards(&self,request:tonic::Request<()>)->Result<tonic::Response<idl::Bitboard>,tonic::Status>
    {
         return Result::Ok(tonic::Response::new(idl::Bitboard
        {
            board:200u64
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