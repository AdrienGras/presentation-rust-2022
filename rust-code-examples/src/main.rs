use rand::{Rng, rngs::ThreadRng};
use std::io;
use std::cmp::Ordering;

/// Game states
#[derive(PartialEq)]
enum GameState {
    /// The game is running
    Running,
    /// The player has won the game
    Won
}

/// Game representation
struct Game {
    /// the value to guess
    value_to_guess: i32,
    /// the value input by the player
    player_value: i32,
    /// current state of the game
    state: GameState
}

/// implementation for struct Game
impl Game {
    /// Create a game
    fn new() -> Self {
        let mut rng: ThreadRng = rand::thread_rng();

        println!("Le jeu du juste prix !");

        Game { 
            value_to_guess: rng.gen_range(0..100), 
            player_value: 0,
            state: GameState::Running
        }
    }

    /// Runs the game
    fn run(&mut self) {
        while self.state != GameState::Won {
            println!("=> Choisissez une valeur entre 0 et 100");

            let mut input_val: String = String::new();
    
            io::stdin()
                .read_line(&mut input_val)
                .expect("Impossible de lire l'entrée utilisateur !");
    
            self.player_value = input_val
                .trim()
                .parse()
                .unwrap();
    
            println!("Vous avez saisi : {}", self.player_value);
    
            match self.player_value.cmp(&self.value_to_guess) {
                Ordering::Less => println!("C'est plus !"),
                Ordering::Greater => println!("C'est moins !"),
                Ordering::Equal => {
                    println!("Gagné !");
                    self.state = GameState::Won;
                },
            }
        }
    }
}

fn main() {
    let mut game: Game = Game::new();
    game.run();
}