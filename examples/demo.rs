//! Demonstrates Gini solver for random transactions.

extern crate piston_mix_economy;
extern crate piston_window;
extern crate rand;
extern crate timer_controller;

use rand::Rng;
use piston_window::*;
use piston_mix_economy::Economy;
use timer_controller::Timer;

pub struct Settings {
    tax_1: f64,
    tax_2: f64,
    min_tax_1: f64,
    target_gini: f64,
    smooth_target: f64,
    start_fortune: f64,
    players: usize,
    avg_trans: f64,
    trans: u32,
    periods: u32,
    smooth_fact: f64,
}

fn main() {
    simulate(tmp_settings(0.2));
}

fn tmp_settings(target_gini: f64) -> Settings {
    let start_fortune = 0.25;
    println!("Start fortune: {}", start_fortune);
    Settings {
        tax_1: 0.0,
        tax_2: 0.0,
        min_tax_1: 0.001,
        target_gini,
        smooth_target: 0.9,
        start_fortune,
        players: 100,
        avg_trans: 0.03,
        trans: 1000,
        periods: 10,
        smooth_fact: 0.99,
    }
}

fn simulate(settings: Settings) {
    let tax_1 = settings.tax_1;
    let tax_2 = settings.tax_2;
    let start_fortune = settings.start_fortune;
    let players = settings.players;
    let avg_transaction = settings.avg_trans;
    let transactions = settings.trans;
    let periods = settings.periods;
    let smooth_fact = settings.smooth_fact;

    let mut economy = Economy::new(tax_1, start_fortune, players);
    let mut economy2 = Economy::new(tax_2, start_fortune, players);
    let update_interval = 0.1;
    let mut timer = Timer::new(update_interval);
    let mut gini_timer = Timer::new(update_interval * periods as f64);
    let mut smooth_gini = 0.0;
    let mut smooth_gini2 = 0.0;
    let mut smooth_tax = 0.0;
    let mut smooth = 1.0;

    let mut rng = rand::rngs::OsRng;

    let mut window: PistonWindow =
        WindowSettings::new("Random transactions", [1024, 768])
        .exit_on_esc(true)
        .build()
        .unwrap();
    window.set_bench_mode(false);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            clear([1.0, 1.0, 1.0, 1.0], g);
            draw_economy(&economy, [1.0, 0.0, 0.0, 1.0], &c.trans(10.0, 110.0), g);
            draw_economy(&economy2, [0.0, 0.0, 0.0, 1.0], &c.trans(10.0, 110.0), g);
        });
        timer.event(&e, || {
            // Make random transactions.
            for _ in 0..transactions {
                let len = economy.players.len();
                let from = rng.gen_range(0..len);
                let to = rng.gen_range(0..len);
                let _ = economy.transaction(from, to, avg_transaction);
                let _ = economy2.transaction(from, to, avg_transaction);
            }

            economy.solve(
                settings.target_gini,
                settings.smooth_target,
                settings.min_tax_1
            );
            economy.players.sort_by(|a, b| a.partial_cmp(b).unwrap());
            economy2.update();
            economy2.players.sort_by(|a, b| a.partial_cmp(b).unwrap());
        });
        gini_timer.event(&e, || {
            smooth_gini += (economy.gini() - smooth_gini) * smooth;
            smooth_gini2 += (economy2.gini() - smooth_gini2) * smooth;
            smooth_tax += (economy.tax - smooth_tax) * smooth;
            println!("gini \ttop: {} \tbottom: {}", smooth_gini, smooth_gini2);
            println!("tax \ttop: {} \tsmooth: {}", smooth_tax, smooth);
            smooth *= smooth_fact;
        })
    }
}

fn draw_economy<G: Graphics>(economy: &Economy, color: [f32; 4], c: &Context, g: &mut G) {
    let c = &c.trans(0.0, 160.0).scale(1.0, -1.0);
    let w: f64 = 1000.0 / economy.players.len() as f64;
    for (i, p) in economy.players.iter().enumerate() {
        let rect = [i as f64 * w, *p * 100.0 - 2.0, w, 2.0];
        rectangle(color, rect, c.transform, g);
    }
    rectangle([0.0, 1.0, 0.0, 1.0], [0.0, economy.start_fortune * 100.0 - 2.0, 1000.0, 2.0],
        c.transform, g);
    rectangle([0.0, 0.0, 1.0, 1.0], [0.0, 98.0, 1000.0, 2.0],
        c.transform, g);
}
