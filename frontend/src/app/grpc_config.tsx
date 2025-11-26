import * as grpc from "@grpc/grpc-js";
import * as protoLoader from "@grpc/proto-loader";
import path from "path";

// 1️⃣ Path to proto
const PROTO_PATH = ("/home/prakhar/Desktop/prakhar/chess_min_max/chess.proto");

// 2️⃣ Load proto
const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
  keepCase: true,
  longs: String,   // uint64 → string
  enums: String,
  defaults: true,
  oneofs: true,
});

const proto: any = grpc.loadPackageDefinition(packageDefinition).chess_engine;

// 3️⃣ Create client
const client = new proto.ChessService(
  "localhost:50051",
  grpc.credentials.createInsecure()
);

// -----------------------------
// Types
interface ffnboard {
  board: string;
}

interface LoadBoardReq {
  board: string;
}

interface GenMoveReq {
  file: number;
  rank: number;
}

interface MakeMoveReq {
  piece: number;
  piece_color: number;
  move_position: string; // uint64 as string
  piece_position: string; // uint64 as string
}

// -----------------------------
// Unary RPC Calls

// 1. LoadBoard
function loadBoard() {
  const req: LoadBoardReq = {
    board: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
  };

  client.LoadBoard(req, (err: grpc.ServiceError | null, res: ffnboard) => {
    if (err) console.error("LoadBoard error:", err);
    else console.log("LoadBoard result:", res.board);
  });
}

// 2. GenerateMoves
function generateMoves() {
  const req: GenMoveReq = { file: 4, rank: 3 };

  client.GenerateMoves(req, (err: grpc.ServiceError | null, res: ffnboard) => {
    if (err) console.error("GenerateMoves error:", err);
    else console.log("GenerateMoves result:", res.board);
  });
}

// 3. GetBoards
function getBoards() {
  client.GetBoards({}, (err: grpc.ServiceError | null, res: ffnboard) => {
    if (err) console.error("GetBoards error:", err);
    else console.log("GetBoards result:", res.board);
  });
}

// 4. MakeMove
function makeMove() {
  const req: MakeMoveReq = {
    piece: 1,
    piece_color: 0,
    move_position: "1024",
    piece_position: "512",
  };

  client.MakeMove(req, (err: grpc.ServiceError | null, res: ffnboard) => {
    if (err) console.error("MakeMove error:", err);
    else console.log("MakeMove result:", res.board);
  });
}

// -----------------------------
// Run Example
loadBoard();
// generateMoves();
// getBoards();
// makeMove();
