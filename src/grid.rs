use std::collections::HashSet;

use macroquad::rand::rand;
use rand::{Rng, rng};

pub struct HexGrid {
    pub grid: Vec<Vec<u32>>,
    pub alive_trees: HashSet<(usize, usize)>,
    pub smoldering_tree: HashSet<(usize, usize)>,
    pub burning_tree: HashSet<(usize, usize)>,
    pub dead_trees_num: u32,
    pub q: usize,
    pub r: usize,
}

// Znalazłem wzór na hex siatke na https://www.redblobgames.com/grids/hexagons/#map-storage
impl HexGrid {
    pub fn new(q: usize, r: usize) -> Self {
        println!("{} X {}", q + r / 2, r);
        HexGrid {
            grid: vec![vec![0_u32; q + r / 2]; r],
            alive_trees: HashSet::new(),
            smoldering_tree: HashSet::new(),
            burning_tree: HashSet::new(),
            dead_trees_num: 0,
            q: q + r / 2,
            r,
        }
    }

    pub fn get_alive_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours: Vec<(usize, usize)> = Vec::new();

        neighbours.push((x, y.wrapping_sub(1)));
        neighbours.push((x + 1, y.wrapping_sub(1)));
        neighbours.push((x + 1, y));
        neighbours.push((x, y + 1));
        neighbours.push((x.wrapping_sub(1), y + 1));
        neighbours.push((x.wrapping_sub(1), y));

        let mut alive_neighbors: Vec<(usize, usize)> = Vec::new();
        for neighbour in neighbours {
            if self.r > neighbour.0
                && self.q > neighbour.1
                && self.grid[neighbour.0][neighbour.1] == 1
            {
                alive_neighbors.push(neighbour);
            }
        }

        alive_neighbors
    }

    pub fn planting_trees(mut self, &density: &f32) -> Self {
        let mut population = (((self.q - self.r / 2) * self.r) as f32 * density) as usize;
        let mut rng = rand::thread_rng();

        while population > 0 {
            let x = rng.gen_range(0..self.r);
            let y = rng.gen_range((((self.r / 2) - (x / 2))..(self.q - (x / 2))));

            // Debuging
            // let start;
            // let end;

            // troche zajeło rozkminięcie dlaczego dla pażystych byłą nie jednoznaczność i nie działało jak było (((self.r  - x / 2))..((self.q - x ) / 2))
            // start = (self.r - x) / 2;
            // end = self.q - (x + 1) / 2;
            //

            // let length = end.saturating_sub(start);
            // println!(
            //     "Długość przedziału dla x = {}: {} : {}-{} | {}",
            //     x, length, start, end, population
            // );

            if self.grid[x][y] == 0 {
                self.grid[x][y] = 1;
                self.alive_trees.insert((x, y));
                population = population - 1;
            }
        }
        // for r in &self.grid {
        //     for c in r {
        //         print!(" {}", c);
        //     }
        //     println!();
        // }
        self
    }

    pub fn update(&mut self) {
        // Tworzymy nowe zbiory dla następnego stanu
        let mut new_smoldering = HashSet::new();
        let mut new_burning = HashSet::new();

        // Przetwarzamy drzewa w stanie "smoldering"
        for &(x, y) in &self.smoldering_tree {
            let neighbors = self.get_alive_neighbors(x, y);
            for (nx, ny) in neighbors {
                if self.grid[nx][ny] == 1 {
                    // Sprawdzamy, czy sąsiad jest żywy
                    self.grid[nx][ny] = 2; // Ustawiamy na "smoldering"
                    new_smoldering.insert((nx, ny));
                    self.alive_trees.remove(&(nx, ny));
                }
            }
            self.grid[x][y] = 3; // Przechodzimy ze "smoldering" na "burning"
            new_burning.insert((x, y));
        }

        // Przetwarzamy drzewa w stanie "burning"
        for &(x, y) in &self.burning_tree {
            self.grid[x][y] = 4; // Ustawiamy na "burned"
            self.dead_trees_num += 1
        }

        // Podmieniamy stare zbiory na nowe
        self.smoldering_tree = new_smoldering;
        self.burning_tree = new_burning;
    }
}
