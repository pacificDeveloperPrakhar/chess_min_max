import * as grpc from "@grpc/grpc-js";
import * as protoLoader from "@grpc/proto-loader";
import path from "path";

// 1️⃣ Path to your proto file
const PROTO_PATH = "/home/prakhar/Desktop/prakhar/chess_min_max/chess.proto";

// 2️⃣ Load proto definition
const packageDefinition = protoLoader.loadSync(PROTO_PATH, {
  keepCase: true,
  longs: String,      // uint64 → string
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
// Types (Optional)
interface Bitboard {
  board: string;
}

interface Bitboards {
  white_bitboard: string[];
  black_bitboard: string[];
}

interface GenMoveReq {
  file: number;
  rank: number;
}

interface LoadBoardReq {
  board: string;
}

interface MakeMoveReq {
  piece: number;
  piece_color: number;
  move_position: string; // uint64 as string
  piece_position: string; // uint64 as string
}

// -----------------------------
// Example Unary Call: GenerateMoves
function generateMoves() {
  const req: GenMoveReq = { file: 4, rank: 3 };
  client.GenerateMoves(req, (err: grpc.ServiceError | null, res: Bitboard) => {
    if (err) console.error("GenerateMoves error:", err);
    else console.log("GenerateMoves result:", res.board);
  });
}

// Example Unary Call: MakeMove
function makeMove() {
  const req: MakeMoveReq = {
    piece: 1,
    piece_color: 0,
    move_position: "1024",
    piece_position: "512",
  };

  client.MakeMove(req, (err: grpc.ServiceError | null, res: Bitboards) => {
    if (err) console.error("MakeMove error:", err);
    else {
      console.log("White Bitboards:", res.white_bitboard);
      console.log("Black Bitboards:", res.black_bitboard);
    }
  });
}

// Example Unary Call: GetBoards
function getBoards() {
  client.GetBoards({}, (err: grpc.ServiceError | null, res: Bitboard) => {
    if (err) console.error("GetBoards error:", err);
    else console.log("GetBoards result:", res.board);
  });
}

// Example Server Streaming Call: LoadBoard
function loadBoard() {
  const req: LoadBoardReq = { board: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR" };
  const stream = client.LoadBoard(req);

  stream.on("data", (chunk: Bitboard) => console.log("Bitboard:", chunk.board));
  stream.on("end", () => console.log("LoadBoard stream finished"));
  stream.on("error", (err: Error) => console.error("LoadBoard error:", err));
}

// Run examples
loadBoard();
// generateMoves();
// getBoards();
// makeMove();
