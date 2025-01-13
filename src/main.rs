use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Fixed};
use gtk4 as gtk;
use std::thread;
use std::time;

use crate::event::{Direction, Event};
use crate::game::Game;
mod event;
mod game;

const WIDTH: i32 = 800;
const HEIGHT: i32 = 200;

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

        window.set_resizable(false);

        let fixed = Fixed::new();

        window.set_child(Some(&fixed));

        // Channel for all events (clock moves, key presses, etc)
        let (sender, receiver) = async_channel::bounded(1);

        // Closure for processing all game events
        glib::spawn_future_local(async move {
            let mut game = Game::new(WIDTH, HEIGHT);

            while let Ok(result) = receiver.recv().await {
                game.process_event(result, &fixed);
            }
        });

        // Move enemies every 1 second
        let enemy_move_sender = sender.clone();
        thread::spawn(move || loop {
            enemy_move_sender.send_blocking(Event::enemy_move());
            thread::sleep(time::Duration::from_secs(1));
        });

        // Listen for keyboard interactions
        let key_ev_controller = gtk::EventControllerKey::new();
        let key_ev_sender = sender.clone();
        key_ev_controller.connect_key_pressed(move |_, key, _, _| {
            match key {
                gtk::gdk::Key::Left => {
                    key_ev_sender.send_blocking(Event::player_move(Direction::Left));
                }
                gtk::gdk::Key::Right => {
                    key_ev_sender.send_blocking(Event::player_move(Direction::Right));
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
