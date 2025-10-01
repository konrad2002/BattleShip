export interface Board {
  winner: number;
  boards: {
    size: number;
    player1: number[];
    player2: number[];
  }
}
