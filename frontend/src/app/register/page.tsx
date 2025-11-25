"use client";
import { useRouter } from "next/navigation";
import { useState } from "react";

export default function Registration() {
    const router = useRouter();
    const [id, setId] = useState("");
    const [color, setColor] = useState("a");

    const handleNavigate = () => {
        if (!id) return;
        router.push(`/board/${id}?color=${color}`);
    };

    return (
        <>
            <input 
                type="text" 
                id="socket_id" 
                onChange={(e) => setId(e.target.value)} 
                value={id} 
                placeholder="Enter Board ID"
            />
            
            <select 
                id="piece_color"
                onChange={(e) => setColor(e.target.value)} 
                value={color}
            >
                <option value="a">White</option>
                <option value="A">Black</option>
            </select>

            <button onClick={handleNavigate}>Play</button>
        </>
    );
}
