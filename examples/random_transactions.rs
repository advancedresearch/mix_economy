//! Shows distribution curve under random transactions.

extern crate piston_mix_economy;
extern crate piston_window;
extern crate rand;
extern crate timer_controller;

use rand::Rng;
use piston_window::*;
use piston_mix_economy::Economy;
use timer_controller::Timer;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Random transactions", [1024, 768])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let tax = 0.01;
    let start_fortune = 0.2;
    let players = 1000;
    let diff = 5.0;
    let mut economy = Economy::new(tax, start_fortune, players);
    let mut economy2 = Economy::new(tax * diff, start_fortune, players);
    let update_interval = 0.1;
    let mut timer = Timer::new(update_interval);

    let avg_transaction = tax;
    let transactions = 1000;

    let mut rng = rand::thread_rng();

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            draw_economy(&economy, &c, g);
            let d = c.trans(0.0, 200.0);
            draw_economy(&economy2, &d, g);
        });
        timer.event(&e, || {
            // Make random transactions.
            for _ in 0..transactions {
                let len = economy.players.len();
                let from = rng.gen_range(0, len);
                let to = rng.gen_range(0, len);
                let _ = economy.transaction(from, to, avg_transaction);
                let _ = economy2.transaction(from, to, avg_transaction);
            }

            economy.update();
            economy.players.sort_by(|a, b| a.partial_cmp(b).unwrap());
            economy2.update();
            economy2.players.sort_by(|a, b| a.partial_cmp(b).unwrap());
        });
    }
}

fn draw_economy<G: Graphics>(economy: &Economy, c: &Context, g: &mut G) {
    let color = [1.0, 0.0, 0.0, 1.0];
    let w: f64 = 1.0;
    for (i, p) in economy.players.iter().enumerate() {
        let rect = [i as f64 * w, 0.0, w, *p * 100.0];
        rectangle(color, rect, c.transform, g);
    }
    rectangle([0.0, 1.0, 0.0, 1.0], [0.0, economy.start_fortune * 100.0 - 2.0, 1000.0, 2.0],
        c.transform, g);
    rectangle([0.0, 0.0, 1.0, 1.0], [0.0, 98.0, 1000.0, 2.0],
        c.transform, g);
}
