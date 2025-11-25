import * as grpc from "@grpc/grpc-js";
import * as protoLoader from "@grpc/proto-loader";

// Load proto
const packageDef = protoLoader.loadSync("/home/prakhar/Desktop/prakhar/chess_min_max/chess.proto", {
  keepCase: true,
  longs: String, // uint64 → string
  enums: String,
  defaults: true,
  oneofs: true,
});

const proto = grpc.loadPackageDefinition(packageDef) as any;
const ChessService = proto.chess_engine.ChessService;

// Create the client
const client = new ChessService(
  "localhost:50051",
  grpc.credentials.createInsecure()
);

// -----------------------------
// Types (Optional but recommended)
// -----------------------------
interface Bitboard {
  board: string;
}

interface Bitboards {
  white_bitboard: string[];
  black_bitboard: string[];
}

interface GenMoveReq {
  white_bitboard: string[];
  black_bitboard: string[];
  file: number;
  rank: number;
}

interface LoadBoardReq {
  board: string;
}

interface MakeMoveReq {
  piece: number;
  piece_color: number;
  white_bitboard: string[];
  black_bitboard: string[];
  move_position: string;
  piece_position: string;
}

// -------------------------------------------------------------
// 1. LoadBoard (Server Streaming)
// -------------------------------------------------------------
export function loadBoardTS() {
  const req: LoadBoardReq = {
    board: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR",
  };

  const stream = client.LoadBoard(req);

  stream.on("data", (chunk: Bitboard) => {
    console.log("Bitboard from stream:", chunk.board);
  });

  stream.on("end", () => console.log("LoadBoard stream finished"));

  stream.on("error", (err: Error) =>
    console.error("LoadBoard error:", err)
  );
}

// -------------------------------------------------------------
// 2. GenerateMoves (Unary)
// -------------------------------------------------------------
export function generateMovesTS() {
  const req: GenMoveReq = {
    white_bitboard: ["12345", "67890"],
    black_bitboard: ["99999"],
    file: 4,
    rank: 3,
  };

  client.GenerateMoves(req, (err: grpc.ServiceError | null, res: Bitboard) => {
    if (err) return console.error("GenerateMoves error:", err);
    console.log("GenerateMoves result:", res.board);
  });
}

// -------------------------------------------------------------
// 3. GetBoards (Unary – Empty request)
// -------------------------------------------------------------
export function getBoardsTS() {
  client.GetBoards({}, (err: grpc.ServiceError | null, res: Bitboard) => {
    if (err) return console.error("GetBoards error:", err);
    console.log("GetBoards returned:", res.board);
  });
}

// -------------------------------------------------------------
// 4. MakeMove (Unary)
// -------------------------------------------------------------
export function makeMoveTS() {
  const req: MakeMoveReq = {
    piece: 1,
    piece_color: 0,
    white_bitboard: ["111", "222"],
    black_bitboard: ["333"],
    move_position: "1024",
    piece_position: "512",
  };

  client.MakeMove(req, (err: grpc.ServiceError | null, res: Bitboards) => {
    if (err) return console.error("MakeMove error:", err);
    console.log("White BB:", res.white_bitboard);
    console.log("Black BB:", res.black_bitboard);
  });
}

// -------------------------------------------------------------
// RUN TESTS
// -------------------------------------------------------------
loadBoardTS();
// generateMovesTS();
// getBoardsTS();
// makeMoveTS();
