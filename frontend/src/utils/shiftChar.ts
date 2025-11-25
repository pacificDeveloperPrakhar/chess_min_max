export default function shiftChar(n: number, char: string): string {
    if (char.length !== 1) throw new Error("Input must be a single character.");
    return String.fromCharCode(char.charCodeAt(0) + n);
}