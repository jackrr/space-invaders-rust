use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use gtk4 as gtk;
use std::thread;
use std::time;

use std::sync::Arc;
use std::sync::Mutex;

use crate::game::{Direction, Game};
mod game;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 600;

fn main() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("space_invaders_game")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Space Invaders")
            .default_width(WIDTH)
            .default_height(HEIGHT)
            .build();

        let game = Game::new(WIDTH, HEIGHT, &window);
        let game_mutex_enemies = Arc::new(Mutex::new(game));
        let game_mutex_user_input = Arc::clone(&game_mutex_enemies);
        let game_animations = Arc::clone(&game_mutex_enemies);

        // Move enemies every 1 second
        thread::spawn(move || loop {
            let mut game = game_mutex_enemies.lock().unwrap();
            game.move_enemies();
            drop(game);
            thread::sleep(time::Duration::from_secs(1));
        });

        thread::spawn(move || loop {
            let mut game = game_animations.lock().unwrap();
            game.next_tick();
            drop(game);
            thread::sleep(time::Duration::from_millis(150));
        });

        // Listen for keyboard interactions
        let key_ev_controller = gtk::EventControllerKey::new();

        key_ev_controller.connect_key_pressed(move |_, key, _, _| {
            let mut game = game_mutex_user_input.lock().unwrap();
            match key {
                gtk::gdk::Key::Left => {
                    game.move_player(Direction::Left);
                }
                gtk::gdk::Key::Right => {
                    game.move_player(Direction::Right);
                }
                gtk::gdk::Key::Up => {
                    game.fire_laser();
                }
                _ => (),
            }
            glib::Propagation::Proceed
        });

        window.add_controller(key_ev_controller);
        window.present();
    });

    application.run()
}
