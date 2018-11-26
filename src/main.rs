use std::fmt;

// Yes, I'm aware arrays exist
struct Player {
    pit1: i8,
    pit2: i8,
    pit3: i8,
    pit4: i8,
    pit5: i8,
    pit6: i8,
    score: i8,
    is_turn: bool,
}

struct GameState {
    white: Player,
    black: Player,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            " {} |{: >2}|{: >2}|{: >2}|{: >2}|{: >2}|{: >2}|   
{: >2} ------------------- {}
 {} |{: >2}|{: >2}|{: >2}|{: >2}|{: >2}|{: >2}|   
",
            if self.black.is_turn { ">" } else { " " },
            self.black.pit1,
            self.black.pit2,
            self.black.pit3,
            self.black.pit4,
            self.black.pit5,
            self.black.pit6,
            self.black.score,
            self.white.score,
            if self.white.is_turn { ">" } else { " " },
            self.white.pit6,
            self.white.pit5,
            self.white.pit4,
            self.white.pit3,
            self.white.pit2,
            self.white.pit1,
        )
    }
}

impl GameState {
    fn new() -> GameState {
        GameState {
            white: Player {
                is_turn: true,
                pit1: 4,
                pit2: 4,
                pit3: 4,
                pit4: 4,
                pit5: 4,
                pit6: 4,
                score: 0,
            },
            black: Player {
                is_turn: false,
                pit1: 4,
                pit2: 4,
                pit3: 4,
                pit4: 4,
                pit5: 4,
                pit6: 4,
                score: 0,
            },
        }
    }
}

fn main() {
    let game_state = GameState::new();
    println!("{}", game_state);
}
