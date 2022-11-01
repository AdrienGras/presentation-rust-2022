use rand::{Rng, rngs::ThreadRng};
use std::io;
use std::cmp::Ordering;

struct Game {
    value_to_guess: i32,
    player_value: i32,
    won: bool
}

impl Game {
    fn new() -> Self {
        let mut rng: ThreadRng = rand::thread_rng();

        println!("Le jeu du juste prix !");

        Game { 
            value_to_guess: rng.gen_range(0..100), 
            player_value: 0,
            won: false
        }
    }

    fn run(&mut self) {
        while self.won != true {
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
                    self.won = true;
                },
            }
        }
    }
}

fn main() {
    let mut game: Game = Game::new();
    game.run();
}