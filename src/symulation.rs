use crate::grid;
use grid::HexGrid;
use rand::Rng;

pub struct Symulation {
    q: usize,
    r: usize,
    density_step: f32,
    pub steps: usize,
}

impl Symulation {
    pub fn new(q: usize, r: usize, steps: usize) -> Self {
        Symulation {
            q,
            r,
            steps: steps,
            density_step: 0.1,
        }
    }

    pub fn run(&mut self) -> Vec<(f32, f32)> {
        let mut results = Vec::new();
        let mut density = 1.0;

        while density >= 0.0 {
            let mut total_survivors = 0.0;

            for x in 0..self.steps {
                println!("{}", x);
                let mut grid = HexGrid::new(self.q, self.r).planting_trees(&density);

                if grid.alive_trees.is_empty() {
                    continue;
                }

                // Choose a random tree to ignite
                let mut rng = rand::thread_rng();
                let random_index = rng.gen_range(0..grid.alive_trees.len());
                let random_tree = *grid.alive_trees.iter().nth(random_index).unwrap();

                grid.smoldering_tree.insert(random_tree);
                grid.grid[random_tree.0][random_tree.1] = 2;

                // Simulate fire spread until it stops
                while !grid.smoldering_tree.is_empty() || !grid.burning_tree.is_empty() {
                    grid.update();
                }

                let survivors = grid.alive_trees.len() as f32;
                total_survivors += survivors;
            }
            let average_survivors = if self.steps > 0 {
                total_survivors / self.steps as f32
            } else {
                0.0
            };

            results.push((density, average_survivors));
            density -= self.density_step
        }
        results
    }
}
