use crate::grid;
use grid::HexGrid;
use macroquad::window::Conf;
use macroquad::{color::BLUE, prelude::*};

pub const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "PyroHex".to_owned(),
        ..Default::default()
    }
}

pub struct Game {
    pub q: usize,
    pub r: usize,
    pub level: Level,
    pub total_points: u32,
    pub game_over: bool,
}

pub struct Level {
    board: HexGrid,
    density: f32,
    player_points: u32,
    best_points: u32,
    hex_size: f32,
    offset_x: f32,
    offset_y: f32,
}

impl Game {
    pub fn init(q: usize, r: usize) -> Game {
        Game {
            q,
            r,
            level: Level::init(q, r, 0.5),
            total_points: 0,
            game_over: false,
        }
    }

    pub async fn run_game(&mut self) {
        loop {
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
            self.level.run_level().await;
        }
    }
}

impl Level {
    pub fn init(q: usize, r: usize, density: f32) -> Level {
        let board = HexGrid::new(q, r).planting_trees(&density);
        Level {
            board,
            density,
            player_points: 0,
            best_points: 0,
            hex_size: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    pub async fn run_level(&mut self) {
        print!("{}", self.board.dead_trees_num);

        let mouse_pos = mouse_position();
        let rows = self.board.r as f32;
        let cols = self.board.q as f32;
        let margin = 0.0;

        let available_width = screen_width() * (1.0 - 2.0 * margin);
        let available_height = screen_height() * (1.0 - 2.0 * margin);
        let hex_size_width = available_width / ((cols + (rows - 1.0) / 2.0) * SQRT_3);
        let hex_size_height = available_height / (rows * 1.5);
        let hex_size = hex_size_width.min(hex_size_height);
        let total_width = (cols + (rows - 1.0) / 2.0) * hex_size * SQRT_3;
        let total_height = rows * hex_size * 1.5;
        let offset_x = (screen_width() - total_width) / 2.0;
        let offset_y = (screen_height() - total_height) / 2.0;

        if is_mouse_button_pressed(MouseButton::Right) {
            for row in 0..self.board.grid.len() {
                let row_f = row as f32;
                for col in
                    (self.board.grid.len() / 2 - row / 2)..(self.board.grid[row].len() - row / 2)
                {
                    let x = offset_x + (col as f32 + row_f / 2.0) * hex_size * SQRT_3;
                    let y = offset_y + row_f * hex_size * 1.5;
                    let distance = ((mouse_pos.0 - x).powi(2) + (mouse_pos.1 - y).powi(2)).sqrt();
                    if distance < hex_size * 0.8 {
                        self.board.grid[row][col] = 2;
                        self.board.smoldering_tree.insert((row, col));
                    }
                }
            }
        }

        self.render().await;
        self.board.update();
    }

    async fn render(&self) {
        clear_background(BLACK);
        draw_rectangle(
            0.0,
            0.0,
            screen_width(),
            screen_height(),
            Color::new(0.2, 0.15, 0.1, 1.0),
        );
        draw_rectangle(
            0.0,
            screen_height() / 2.0,
            screen_width(),
            screen_height() / 2.0,
            Color::new(0.3, 0.25, 0.15, 0.5),
        );

        let rows = self.board.r as f32;
        let cols = self.board.q as f32;
        let margin = 0.0;
        let available_width = screen_width() * (1.0 - 2.0 * margin);
        let available_height = screen_height() * (1.0 - 2.0 * margin);
        let hex_size_width = available_width / ((cols + (rows - 1.0) / 2.0) * SQRT_3);
        let hex_size_height = available_height / (rows * 1.5);
        let hex_size = hex_size_width.min(hex_size_height);
        let total_width = (cols + (rows - 1.0) / 2.0) * hex_size * SQRT_3;
        let total_height = rows * hex_size * 1.5;
        let offset_x = (screen_width() - total_width) / 2.0;
        let offset_y = (screen_height() - total_height) / 2.0;

        let mouse_pos = mouse_position();
        let colors = [
            (
                Color::new(0.5, 0.3, 0.1, 1.0),
                Color::new(0.3, 0.2, 0.05, 1.0),
                Color::new(1.0, 1.0, 1.0, 0.2),
            ),
            (
                Color::new(0.0, 0.8, 0.0, 1.0),
                Color::new(0.0, 0.4, 0.0, 1.0),
                Color::new(1.0, 1.0, 1.0, 0.3),
            ),
            (
                Color::new(1.0, 0.5, 0.0, 1.0),
                Color::new(0.8, 0.3, 0.0, 1.0),
                Color::new(1.0, 1.0, 1.0, 0.4),
            ),
            (
                Color::new(1.0, 0.1, 0.1, 1.0),
                Color::new(0.7, 0.05, 0.05, 1.0),
                Color::new(1.0, 1.0, 1.0, 0.5),
            ),
            (
                Color::new(0.4, 0.4, 0.4, 1.0),
                Color::new(0.2, 0.2, 0.2, 1.0),
                Color::new(1.0, 1.0, 1.0, 0.1),
            ),
        ];

        for row in 0..self.board.grid.len() {
            let row_f = row as f32;
            for col in (self.board.grid.len() / 2 - row / 2)..(self.board.grid[row].len() - row / 2)
            {
                let x = offset_x + (col as f32 + row_f / 2.0) * hex_size * SQRT_3;
                let y = offset_y + row_f * hex_size * 1.5;

                let is_hovered = mouse_pos.0 >= x - hex_size
                    && mouse_pos.0 <= x + hex_size
                    && mouse_pos.1 >= y - hex_size
                    && mouse_pos.1 <= y + hex_size;

                let state = self.board.grid[row][col] as usize;
                let (fill_color, border_color, top_highlight) = colors[state];

                let final_fill = if is_hovered {
                    Color::new(
                        fill_color.r + 0.2,
                        fill_color.g + 0.2,
                        fill_color.b + 0.2,
                        fill_color.a,
                    )
                } else {
                    fill_color
                };

                draw_hexagon(x, y, hex_size, 0.0, true, border_color, final_fill);

                if state >= 1 && state <= 3 {
                    draw_hexagon(
                        x,
                        y - hex_size * 0.1,
                        hex_size * 0.9,
                        0.4,
                        true,
                        Color::new(0.0, 0.0, 0.0, 0.0),
                        top_highlight,
                    );
                }

                if state == 1 {
                    let pulse = (get_time().sin() as f32 * 0.5 + 0.5) * 0.1 + 0.9;
                    draw_hexagon(
                        x,
                        y,
                        hex_size * pulse,
                        0.4,
                        false,
                        Color::new(0.0, 1.0, 0.0, 0.3),
                        Color::new(0.0, 0.0, 0.0, 0.0),
                    );
                }
            }
        }

        next_frame().await;
    }
}
