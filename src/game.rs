use std::cmp::Ordering;
use std::collections::HashMap;

use gtk::prelude::*;
use gtk::{ApplicationWindow, Fixed, Picture};
use gtk4 as gtk;

const LASER_WIDTH: i32 = 4;
const LASER_HEIGHT: i32 = 32;
const ENEMY_SIZE: i32 = 60;
const PLAYER_SIZE: i32 = 100;
const X_GAP_SIZE: i32 = 10;
const Y_GAP_SIZE: i32 = 10;
const MOVES_PER_ROW: i32 = 10;
const MOVE_SIZE: i32 = 10;
const LASER_ADVANCE_SIZE: i32 = 20;

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    Left,
    Right,
}

pub struct Game {
    enemies: Vec<Vec<Enemy>>,
    player: Player,
    lasers: Vec<Laser>,
    moves: i32,
    width: i32,
    view: Fixed,
}

// Enable use with Mutex
unsafe impl Send for Game {}

impl Game {
    pub fn new(width: i32, height: i32, window: &ApplicationWindow) -> Self {
        // Ahhhh algebra
        let enemies_per_row =
            (width - (MOVES_PER_ROW * MOVE_SIZE) + X_GAP_SIZE) / (X_GAP_SIZE + ENEMY_SIZE);

        let fixed = Fixed::new();
        window.set_child(Some(&fixed));

        let player = Player::new(
            // Midpoint on bottom row
            (width / 2) - (PLAYER_SIZE / 2),
            height - PLAYER_SIZE,
            &fixed,
        );

        let enemies = vec![
            Enemy::generate_row(enemies_per_row, EnemyKind::Hard, 0, &fixed),
            Enemy::generate_row(
                enemies_per_row,
                EnemyKind::Medium,
                ENEMY_SIZE + Y_GAP_SIZE,
                &fixed,
            ),
            Enemy::generate_row(
                enemies_per_row,
                EnemyKind::Easy,
                2 * (ENEMY_SIZE + Y_GAP_SIZE),
                &fixed,
            ),
        ];

        Self {
            view: fixed,
            width,
            moves: 0,
            lasers: vec![],
            player,
            enemies,
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

        self.render_player();
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

        self.render_enemies();
    }

    pub fn fire_laser(&mut self) {
        let laser = Laser::new(
            self.player.location.x + PLAYER_SIZE / 2,
            self.player.location.y - 50,
            &self.view,
        );

        self.lasers.push(laser);
    }

    pub fn next_tick(&mut self) {
        let mut to_remove_lasers = vec![];
        let mut to_remove_enemies = vec![];
        for (laser_idx, laser) in self.lasers.iter_mut().enumerate() {
            laser.location.y -= LASER_ADVANCE_SIZE;

            if laser.location.y < -LASER_HEIGHT {
                laser.unrender(&self.view);
                to_remove_lasers.push(laser_idx);
                continue;
            }

            // Detect collisions w/ enemies
            for (row_idx, row) in self.enemies.iter_mut().enumerate() {
                for (enemy_idx, enemy) in row.iter_mut().enumerate() {
                    if contains(enemy.location, ENEMY_SIZE, ENEMY_SIZE, laser.location) {
                        if to_remove_enemies.contains(&(row_idx, enemy_idx)) {
                            continue;
                        }

                        // Collision! Clear laser and deal damage to enemy
                        laser.unrender(&self.view);
                        to_remove_lasers.push(laser_idx);

                        enemy.health -= 1;

                        if enemy.health < 1 {
                            enemy.unrender(&self.view);
                            to_remove_enemies.push((row_idx, enemy_idx));
                        }
                    }
                }
            }

            laser.render(&self.view);
        }

        // Each removal shifts down indices!
        let mut lasers_removed = 0;
        for laser_idx in to_remove_lasers {
            self.lasers.remove(laser_idx - lasers_removed);
            lasers_removed += 1;
        }

        // Sort to remove enemies to use same index decrement trick
        to_remove_enemies.sort_by(|(a_row, a_idx), (b_row, b_idx)| {
            let row_order = a_row.cmp(b_row);
            if row_order != Ordering::Equal {
                return row_order;
            }
            a_idx.cmp(b_idx)
        });
        let mut enemies_removed = vec![0, 0, 0];
        for (row_idx, enemy_idx) in to_remove_enemies {
            self.enemies[row_idx].remove(enemy_idx - enemies_removed[row_idx]);
            enemies_removed[row_idx] += 1;
        }
    }

    fn render_enemies(&mut self) {
        // Renders enemies to the GTK fixed view.
        // Uses latest locations defined on the enemies.
        for row in self.enemies.iter_mut() {
            for enemy in row.iter_mut() {
                enemy.render(&self.view);
            }
        }
    }

    fn render_player(&mut self) {
        // Renders player to GTK fixed view.
        // Uses latest location on player.
        self.player.render(&self.view);
    }
}

#[derive(Copy, Clone)]
enum EnemyKind {
    Hard,
    Medium,
    Easy,
}

#[derive(Copy, Clone)]
struct Location {
    x: i32,
    y: i32,
}

struct Enemy {
    health: i32,
    image: Picture,
    location: Location,
}

impl Enemy {
    fn generate_row(count: i32, kind: EnemyKind, y: i32, view: &Fixed) -> Vec<Self> {
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

        let health = match kind {
            EnemyKind::Easy => 1,
            EnemyKind::Medium => 2,
            EnemyKind::Hard => 3,
        };

        for i in 0..count {
            let p = Picture::for_filename(img_path);
            p.set_size_request(ENEMY_SIZE, ENEMY_SIZE);
            let enemy = Self {
                health,
                image: p,
                location: Location {
                    x: start_x + i * (ENEMY_SIZE + X_GAP_SIZE),
                    y,
                },
            };
            view.put(
                &enemy.image,
                enemy.location.x as f64,
                enemy.location.y as f64,
            );
            enemies.push(enemy);
        }
        enemies
    }

    fn render(&mut self, view: &Fixed) {
        view.move_(&self.image, self.location.x as f64, self.location.y as f64);
    }

    fn unrender(&mut self, fixed: &Fixed) {
        fixed.remove(&self.image);
    }
}

struct Laser {
    location: Location,
    image: Picture,
}

impl Laser {
    fn new(x: i32, y: i32, view: &Fixed) -> Self {
        let p = Picture::for_filename(
            "/home/jack/projects/recurse-application/space-invaders-rust/assets/laser.png",
        );
        p.set_size_request(LASER_WIDTH, LASER_HEIGHT);

        let laser = Self {
            location: Location { x, y },
            image: p,
        };

        view.put(
            &laser.image,
            laser.location.x as f64,
            laser.location.y as f64,
        );

        laser
    }

    fn render(&mut self, fixed: &Fixed) {
        fixed.move_(&self.image, self.location.x as f64, self.location.y as f64);
    }

    fn unrender(&mut self, fixed: &Fixed) {
        fixed.remove(&self.image);
    }
}

struct Player {
    location: Location,
    image: Picture,
}

impl Player {
    fn new(x: i32, y: i32, view: &Fixed) -> Self {
        let p = Picture::for_filename(
            "/home/jack/projects/recurse-application/space-invaders-rust/assets/penelope.png",
        );
        p.set_size_request(PLAYER_SIZE, PLAYER_SIZE);

        let player = Self {
            location: Location { x, y },
            image: p,
        };

        view.put(
            &player.image,
            player.location.x as f64,
            player.location.y as f64,
        );

        player
    }

    fn render(&mut self, fixed: &Fixed) {
        fixed.move_(&self.image, self.location.x as f64, self.location.y as f64);
    }
}

fn contains(box_tl: Location, width: i32, height: i32, point: Location) -> bool {
    point.x >= box_tl.x
        && point.x <= box_tl.x + width
        && point.y >= box_tl.y
        && point.y <= box_tl.y + height
}
