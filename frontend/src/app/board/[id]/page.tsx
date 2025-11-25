"use client"
import "@/style/chess_board.css"
import clsx from "clsx";
import { useEffect, useState } from "react"
import Image from "next/image";
import { useParams ,useSearchParams } from "next/navigation";
import {motion,useDragControls} from "motion/react";
import { io } from "socket.io-client";
import { h1 } from "motion/react-client";
import shiftChar from "@/utils/shiftChar"
const socket=io("http://127.0.0.1:1234", {
  path: "/interface_chess/"
})
export default function Board(){
    // the game initial overall board piece do be rendered accordingly
    const [state,setState]=useState(Array.from({length:8},()=>Array(8).fill(0)));
    const [room,setRoom]=useState({id:null,current_connections:0});
    const params=useSearchParams();
    const isUpperCase=/^(?=[A-Z])[A-Z\s]+$/;
    const color=params.get("color");
    const { id } = useParams();
    console.log("this is the room",color);

    // this is the epd notation which do be used to render the state of the chess game
    const boardSetup = "rnbqkbnr/pppppppp/Q0000Q00/0000Q000/00000000/0000Q000/PPPPPPPP/RNBQKBNR";

const [epd, setEpd] = useState(() => {
  return !isUpperCase.test(color?color:'a')
    ? boardSetup.split("/").reverse().map(row => row.split("")) 
    : boardSetup.split("/").map(row => row.split(""));
});
    const [epd_display,setEpdDisplay]=useState("00000000/00000000/000...00/....0.../000...00/00.0.0.0/0-00-00-/00000000".split("/").reverse().map((row)=>row.split("")));
    useEffect(()=>{
      console.log("executing the effect");
      socket.emit("initialize",id);
      socket.on("joined",async (data)=>{
        const {id,current_connections}=data;
        console.log(current_connections);
        setRoom({
          id,current_connections
        });
      })
      socket.on("moves_state_change",(data)=>{
        console.log(data);
        if(color=='a')
        setEpdDisplay(data.split("/").reverse().map((row)=>row.split("")))
        else
        setEpdDisplay(data.split("/").map((row)=>row.split("")))
      })
    },[])
    return<> 
        <div className="board  absolute "> 
            {state.map((squares,row)=>{

                return squares.map((square,col)=>{
                const isBlack=isUpperCase.test(epd[row][col])
                const piece_to_be_placed=isBlack?epd[row][col]+"b":epd[row][col];
                return <>
                <div className={clsx(`${(col+row)%2==0?"square_white":"square_black"}`, {})
                }
                >
                    <motion.div className={clsx("square_piece",{
            'kill_move': epd_display[row][col] === '-',
            'movable_move': epd_display[row][col] === '.'
        })}
        {...( 
          (epd_display[row][col] === '.' || epd_display[row][col] === '-') && {
            onClick: () => {
              console.log("move selected");
              const move = shiftChar(row, 'a') + shiftChar(col, '1');
              
              socket.emit("move_selected", { move, type: "moveType" ,color});
            }
          }
        )}
        >
                        
                          
                          {(epd[row][col] !== "0" && epd[row][col] !== "-" && epd[row][col] !== ".") ? (
                            <Image
  src={`/chess_piece/${piece_to_be_placed.toLowerCase()}.svg`}
  width={59}
  height={59}
  alt={piece_to_be_placed}
  {...(isBlack === isUpperCase.test(color ? color : 'a') && {
    onClick: () => {
      // Convert row and col to chess notation (e.g., a6)
      const move = [row,col]
      
      socket.emit("piece_selected", {
        move,
        type: "pieceType",
        epd: epd.map(epd_row => epd_row.join("")).join("/"),
        color
      });
    }
  })}
/>
                          ) :<></>}
                          
                        
                    </motion.div>
                </div>
                </>})
            })}
        </div>
        <div className=" absolute right-0">
          {room&&<h1>room joined:{room.id} </h1>}
          {room.current_connections&&<h1>connected sockets:{room.current_connections}</h1>}
        </div>
    </>
}
