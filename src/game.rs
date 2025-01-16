use gtk::prelude::*;
use gtk::{ApplicationWindow, Fixed, Picture};
use gtk4 as gtk;

const ENEMY_SIZE: i32 = 60;
const PLAYER_SIZE: i32 = 100;
const X_GAP_SIZE: i32 = 10;
const Y_GAP_SIZE: i32 = 10;
const MOVES_PER_ROW: i32 = 10;
const MOVE_SIZE: i32 = 10;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Left,
    Right,
}

pub struct Game {
    enemies: Vec<Vec<Enemy>>,
    player: Player,
    moves: i32,
    width: i32,
    view: Fixed,
}

unsafe impl Send for Game {}

impl Game {
    pub fn new(width: i32, height: i32, window: &ApplicationWindow) -> Self {
        // Ahhhh algebra
        let enemies_per_row =
            (width - (MOVES_PER_ROW * MOVE_SIZE) + X_GAP_SIZE) / (X_GAP_SIZE + ENEMY_SIZE);

        let fixed = Fixed::new();
        window.set_child(Some(&fixed));

        Self {
            view: fixed,
            width,
            moves: 0,
            player: Player::new(
                // Midpoint on bottom row
                (width / 2) - (PLAYER_SIZE / 2),
                height - PLAYER_SIZE,
            ),
            enemies: vec![
                Enemy::generate_row(enemies_per_row, EnemyKind::Hard, 0),
                Enemy::generate_row(enemies_per_row, EnemyKind::Medium, ENEMY_SIZE + Y_GAP_SIZE),
                Enemy::generate_row(
                    enemies_per_row,
                    EnemyKind::Easy,
                    2 * (ENEMY_SIZE + Y_GAP_SIZE),
                ),
            ],
        }
    }

    pub fn render(&mut self) {
        // Renders player and enemies to the GTK fixed view.
        // Uses latest locations defined on the entities.
        self.player.render(&self.view);
        for row in self.enemies.iter_mut() {
            for enemy in row.iter_mut() {
                enemy.render(&self.view);
            }
        }
    }

    pub fn move_player(&mut self, direction: Direction) {
        // println!("Handling move {:?}", direction);
        if direction == Direction::Left {
            self.player.location.x -= MOVE_SIZE;
            if self.player.location.x < 0 {
                self.player.location.x = 0;
            }
        }

        if direction == Direction::Right {
            self.player.location.x += MOVE_SIZE;
            if self.player.location.x > (self.width - PLAYER_SIZE) {
                self.player.location.x = self.width - PLAYER_SIZE;
            }
        }
    }

    pub fn move_enemies(&mut self) {
        // Shifts enemies by 1 step.
        // On even shift #s, moves to the right.
        // On odd shift #s, moves to the left.
        self.moves += 1;

        let num_row_shifts = self.moves / MOVES_PER_ROW;
        let is_row_shift = self.moves % MOVES_PER_ROW == 0;
        let direction = num_row_shifts % 2;

        let x_offset = if direction == 0 {
            MOVE_SIZE
        } else {
            -MOVE_SIZE
        };

        for row in self.enemies.iter_mut() {
            for enemy in row.iter_mut() {
                if is_row_shift {
                    enemy.location.y += MOVE_SIZE;
                } else {
                    enemy.location.x += x_offset;
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
enum EnemyKind {
    Hard,
    Medium,
    Easy,
}

struct Location {
    x: i32,
    y: i32,
}

struct Enemy {
    kind: EnemyKind,
    image: Picture,
    location: Location,
    rendered: bool,
}

impl Enemy {
    fn generate_row(count: i32, kind: EnemyKind, y: i32) -> Vec<Self> {
        let mut enemies: Vec<Self> = Vec::new();
        let start_x = 0;

        let img_path = match kind {
            // FIXME: No absolute paths on system!
            EnemyKind::Easy => {
                "/home/jack/projects/recurse-application/space-invaders-rust/assets/balloon.png"
            }
            EnemyKind::Medium => {
                "/home/jack/projects/recurse-application/space-invaders-rust/assets/basketball.png"
            }
            EnemyKind::Hard => {
                "/home/jack/projects/recurse-application/space-invaders-rust/assets/vacuum.png"
            }
        };

        for i in 0..count {
            let p = Picture::for_filename(img_path);
            p.set_size_request(ENEMY_SIZE, ENEMY_SIZE);
            enemies.push(Self {
                kind,
                image: p,
                location: Location {
                    x: start_x + i * (ENEMY_SIZE + X_GAP_SIZE),
                    y,
                },
                rendered: false,
            });
        }
        enemies
    }

    fn render(&mut self, fixed: &Fixed) {
        if self.rendered {
            fixed.move_(&self.image, self.location.x as f64, self.location.y as f64);
        } else {
            fixed.put(&self.image, self.location.x as f64, self.location.y as f64);
            self.rendered = true;
        }
    }
}

struct Player {
    location: Location,
    image: Picture,
    rendered: bool,
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        let p = Picture::for_filename(
            "/home/jack/projects/recurse-application/space-invaders-rust/assets/penelope.png",
        );
        p.set_size_request(PLAYER_SIZE, PLAYER_SIZE);

        Self {
            location: Location { x, y },
            image: p,
            rendered: false,
        }
    }

    fn render(&mut self, fixed: &Fixed) {
        if self.rendered {
            fixed.move_(&self.image, self.location.x as f64, self.location.y as f64);
        } else {
            fixed.put(&self.image, self.location.x as f64, self.location.y as f64);
            self.rendered = true;
        }
    }
}
