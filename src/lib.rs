use rand::Rng;


#[derive(Clone, Copy)]
enum Square {
    Empty,
    Ladder { to: usize },
    Chute { to: usize },
}

struct Player {
    name: String,
    square: usize,
}

pub struct Game {
    players: Vec<Player>,
    //Ignore board[0], board squares range from 1..100
    board: [Square; 101],
    move_num: usize,
    //Index into players
    turn: usize,
    game_over: bool,
}

impl Game {
    pub fn new(player_names: Vec<String>) -> Game {
        let mut board = [Square::Empty; 101];
        
        for i in 1..100 {
            board[i] = match i {
                1 => Square::Ladder { to: 38 },
                4 => Square::Ladder { to: 14 },
                9 => Square::Ladder { to: 31 },
                16 => Square::Chute { to: 6 },
                21 => Square::Ladder { to: 42 },
                28 => Square::Ladder { to: 84 },
                36 => Square::Ladder { to: 44 },
                47 => Square::Chute { to: 26 },
                49 => Square::Chute { to: 11 },
                51 => Square::Ladder { to: 67 },
                56 => Square::Chute { to: 53},
                62 => Square::Chute { to: 19},
                64 => Square::Chute { to: 60 },
                80 => Square::Ladder { to: 100 },
                87 => Square::Chute { to: 24 },
                93 => Square::Chute { to: 73 },
                95 => Square::Chute { to: 75 },
                98 => Square::Chute { to: 78 },
                _ => Square::Empty,
            }
        }

        let mut players = Vec::new();
        
        for name in player_names {
            players.push(Player { name, square: 1 });
        }

        Game { players, board, move_num: 1 , turn: 0, game_over: false}
    }

    //Play one turn and move player accordingly
    //If at the start of the turn someone has already won return Err
    //Else return Ok
    pub fn turn(&mut self) -> Result<(), &'static str> {
        if self.game_over {
            return Err("Game is over!");
        }
        
        let mut rng = rand::thread_rng();
        let player = self.players.get_mut(self.turn).unwrap();
        let mv = rng.gen_range(1..6) as usize;
        let mut next_square = if player.square + mv <= 100 { 
                                        player.square + mv 
                                    } else {
                                        player.square
                                    };

        print!("{}: {}: {} --> {}", self.move_num, player.name, player.square, next_square);
        
        match self.board[next_square] {
            Square::Empty => println!(),
            Square::Ladder { to } => {
                println!("--LADDER--> {}", to);
                next_square = to;
            },
            Square::Chute { to} => {
                println!("--CHUTE--> {}", to);
                next_square = to;
            }
        }

        if next_square == 100 {
            self.game_over = true;
            println!("The winner is {}!", player.name)
        }

        player.square = next_square;
        
        self.turn = if self.turn == self.players.len() - 1 { 0 } else { self.turn + 1};
        self.move_num+=1;

        Ok(())
    }
}




