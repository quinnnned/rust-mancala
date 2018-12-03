use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
struct Player {
    pits: [i8; 6],
    score: i8,
}

impl Player {
    fn get_moves(&self) -> Vec<usize> {
        (0..6)
            .into_iter()
            .filter(|&i| self.pits[i] != 0)
            .collect::<Vec<_>>()
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum GameMode {
    WhiteTurn,
    BlackTurn,
    GameOver,
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct GameState {
    mode: GameMode,
    white: Player,
    black: Player,
}

struct GameSkeuomorph {
    b: [i8; 8],
    w: [i8; 8],
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "   |{: >2}|{: >2}|{: >2}|{: >2}|{: >2}|{: >2}|   
{: >2} |--{}--| {}
   |{: >2}|{: >2}|{: >2}|{: >2}|{: >2}|{: >2}|   
",
            // Top Row: Black Pits (reverse order)
            self.black.pits[5],
            self.black.pits[4],
            self.black.pits[3],
            self.black.pits[2],
            self.black.pits[1],
            self.black.pits[0],
            // Middle Row: Score and Status
            self.black.score,
            match self.mode {
                GameMode::WhiteTurn => "WHITE TO MOVE",
                GameMode::BlackTurn => "BLACK TO MOVE",
                GameMode::GameOver => "--GAME OVER--",
            },
            self.white.score,
            // Bottom Row: White Pits
            self.white.pits[0],
            self.white.pits[1],
            self.white.pits[2],
            self.white.pits[3],
            self.white.pits[4],
            self.white.pits[5],
        )
    }
}

impl GameState {
    fn get_valid_moves(&self) -> Vec<usize> {
        match self.mode {
            GameMode::WhiteTurn => self.white.get_moves(),
            GameMode::BlackTurn => self.black.get_moves(),
            GameMode::GameOver => vec![],
        }
    }

    fn from(s: GameSkeuomorph) -> GameState {
        GameState {
            mode: if s.b[7] == 0 {
                if s.w[0] == 0 {
                    GameMode::GameOver
                } else {
                    GameMode::WhiteTurn
                }
            } else {
                GameMode::BlackTurn
            },
            white: Player {
                pits: [s.w[1], s.w[2], s.w[3], s.w[4], s.w[5], s.w[6]],
                score: s.w[7],
            },
            black: Player {
                pits: [s.b[6], s.b[5], s.b[4], s.b[3], s.b[2], s.b[1]],
                score: s.b[0],
            },
        }
    }

    fn new() -> GameState {
        GameState::from(GameSkeuomorph {
            b: [0, 4, 4, 4, 4, 4, 4, 0],
            w: [1, 4, 4, 4, 4, 4, 4, 0],
        })
    }

    fn get_next_state(&self, pit_index: usize) -> GameState {
        // Ignore moves after Game Over
        if self.mode == GameMode::GameOver {
            return GameState { ..*self };
        }

        // Set up active/inactive semantics
        let (mut active_player, mut inactive_player) = match self.mode {
            GameMode::WhiteTurn => (self.white, self.black),
            _ => (self.black, self.white),
        };

        let mut last_stone_was_score = false;
        let mut stones_to_move = active_player.pits[pit_index];
        let mut pit_pointer = pit_index;
        const MAX_PIT_INDEX: usize = 5;
        active_player.pits[pit_pointer] = 0;

        while stones_to_move > 0 {
            while stones_to_move > 0 && pit_pointer < MAX_PIT_INDEX {
                pit_pointer += 1;
                stones_to_move -= 1;
                active_player.pits[pit_pointer] += 1;
            }
        }

        // Undo active/inactive semantics
        let (white, black) = match self.mode {
            GameMode::WhiteTurn => (active_player, inactive_player),
            _ => (inactive_player, active_player),
        };

        let mode = if last_stone_was_score {
            self.mode
        } else {
            if self.mode == GameMode::WhiteTurn {
                GameMode::BlackTurn
            } else {
                GameMode::WhiteTurn
            }
        };

        return GameState { mode, white, black };
    }
}

#[test]
fn game_over_is_permanent() {
    let game_over = GameState::from(GameSkeuomorph {
        b: [0, 4, 4, 4, 4, 4, 4, 0],
        w: [0, 4, 4, 4, 4, 4, 4, 0],
    });

    assert_eq!(game_over.get_next_state(0), game_over);
}

#[test]
fn basic_move() {
    assert_eq!(
        GameState::new().get_next_state(0),
        GameState::from(GameSkeuomorph {
            b: [0, 4, 4, 4, 4, 4, 4, 1],
            w: [0, 0, 5, 5, 5, 5, 4, 0],
        })
    );
}

fn main() {}
