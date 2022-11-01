use rand::{Rng, rngs::ThreadRng};
use std::io;

fn main() {
    // utilitaire pour générer des valeurs aléatoires
    let mut rng: ThreadRng = rand::thread_rng();

    // déclaration de la valeur du jeu
    let game_value: i32 = rng.gen_range(0..100);

    // déclaration de la valeur saisie par l'utilisateur
    let mut player_guess: String = String::new();

    println!("Le jeu du juste prix !");
    println!("=> Choisissez une valeur entre 0 et 100");

    // lecture de la valeur saisie par l'utilisateur dans 'player_guess'
    io::stdin()
        .read_line(&mut player_guess)
        .expect("Impossible de lire l'entrée utilisateur !");

    // transformation de la valeur saisie par l'utilisateur en entier
    let player_guess_as_int: i32 = player_guess
        .trim()
        .parse()
        .expect("Cette valeur n'est pas un entier !");

    println!("Vous avez saisi : {}", player_guess_as_int);

    if player_guess_as_int == game_value {
        println!("Vous avez trouvé !");
    } else if player_guess_as_int > game_value {
        println!("C'est moins !");
    } else {
        println!("C'est plus !");
    }
}