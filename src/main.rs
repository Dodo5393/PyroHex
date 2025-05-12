mod game;
mod grid;
mod plot;
mod symulation;
use clap::{Arg, ArgAction, ArgGroup, Command, value_parser};
use game::Game;
use game::window_conf;
use macroquad::prelude::*;
use plot::plot_results;
use symulation::Symulation;

#[macroquad::main(window_conf)]
async fn main() {
    // Zdefiniuj aplikację z poprawnym parsowaniem flag boolean
    let matches = Command::new("PyroHex")
        .version("1.0")
        .about("Symulacja i gra w PyroHex")
        .arg(
            Arg::new("game")
                .short('g')
                .long("game")
                .help("Uruchamia tryb gry z userem")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("simulation")
                .short('s')
                .long("simulation")
                .help("Uruchamia tryb symulacji z wykresem")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("steps")
                .long("steps")
                .value_name("STEPS")
                .value_parser(value_parser!(usize))
                .help("Liczba kroków symulacji")
                .requires("simulation"),
        )
        .arg(
            Arg::new("grid")
                .long("grid")
                .value_names(&["WIDTH", "HEIGHT"])
                .num_args(2)
                .default_values(&["25", "50"])
                .help("Ustawia rozmiar siatki: szerokość i wysokość"),
        )
        .group(
            ArgGroup::new("mode")
                .args(&["game", "simulation"])
                .required(true)
                .multiple(false),
        )
        .get_matches();

    // Odczytanie wartości grid
    let grid_values: Vec<_> = matches
        .get_many::<String>("grid")
        .expect("Domyślne wartości zawsze są dostępne")
        .collect();
    let width: usize = grid_values[0].parse().unwrap_or(25);
    let height: usize = grid_values[1].parse().unwrap_or(50);
    println!("Rozmiar siatki: {} na {}", width, height);

    // Sprawdzenie wybranej opcji za pomocą get_flag
    let game_mode = matches.get_flag("game");
    let sim_mode = matches.get_flag("simulation");
    println!("game flag present: {}", game_mode);
    println!("simulation flag present: {}", sim_mode);

    if game_mode {
        println!("Uruchamiam tryb gry...");
        let mut game = Game::init(height, width);
        game.run_game().await;
    } else if sim_mode {
        let steps = *matches.get_one::<usize>("steps").unwrap();
        println!("Uruchamiam symulację na {} kroków...", steps);
        let mut sym = Symulation::new(width, height, steps);
        let results = sym.run();
        plot_results(results);
    }
}
