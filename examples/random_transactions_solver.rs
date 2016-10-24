//! Solves Gini for random transactions.

extern crate piston_mix_economy;
extern crate piston_window;
extern crate rand;
extern crate timer_controller;

use rand::Rng;
use piston_window::*;
use piston_mix_economy::Economy;
use timer_controller::Timer;

pub struct Settings {
    id: u32,
    tax_1: f64,
    tax_2: f64,
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
    let n = 100;
    for i in 0..100 {
        let target_gini = 0.5 * (n-i) as f64 / n as f64;
        simulate(tmp_settings(i, target_gini));
    }
}

fn tmp_settings(id: u32, target_gini: f64) -> Settings {
    let start_fortune = 0.05;
    println!("Start fortune: {}", start_fortune);
    Settings {
        id: id,
        tax_1: 0.0,
        tax_2: 0.0,
        target_gini: target_gini,
        smooth_target: 0.9,
        start_fortune: start_fortune,
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

    let mut rng = rand::os::OsRng::new().unwrap();

    let window = false;
    if window {
        let mut window: PistonWindow =
            WindowSettings::new("Random transactions", [1024, 768])
            .exit_on_esc(true)
            .build()
            .unwrap();
        window.set_bench_mode(true);
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g| {
                clear([1.0, 1.0, 1.0, 1.0], g);
                draw_economy(&economy, &c, g, players);
                let d = c.trans(0.0, 500.0);
                draw_economy(&economy2, &d, g, players);
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

                economy.solve(settings.target_gini, settings.smooth_target);
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
    } else {
        loop {
            for _ in 0..periods {
                // Make random transactions.
                for _ in 0..transactions {
                    let len = economy.players.len();
                    let from = rng.gen_range(0, len);
                    let to = rng.gen_range(0, len);
                    let _ = economy.transaction(from, to, avg_transaction);
                    let _ = economy2.transaction(from, to, avg_transaction);
                }

                economy.solve(settings.target_gini, settings.smooth_target);
                // economy.players.sort_by(|a, b| a.partial_cmp(b).unwrap());
                economy2.update();
                // economy2.players.sort_by(|a, b| a.partial_cmp(b).unwrap());
            }

            smooth_gini += (economy.gini() - smooth_gini) * smooth;
            smooth_gini2 += (economy2.gini() - smooth_gini2) * smooth;
            smooth_tax += (economy.tax - smooth_tax) * smooth;
            smooth *= smooth_fact;
            if smooth < 1.0 - smooth_fact { break; }
        }

        println!("id: {} \ttarget_gini: {}", settings.id, settings.target_gini);
        println!("gini \ttop: {} \tbottom: {}", smooth_gini, smooth_gini2);
        println!("tax \ttop: {} \tsmooth: {}", smooth_tax, smooth);
    }
}

fn draw_economy<G: Graphics>(economy: &Economy, c: &Context, g: &mut G, players: usize) {
    let color = [1.0, 0.0, 0.0, 1.0];
    let w: f64 = 1000.0 / players as f64;
    for (i, p) in economy.players.iter().enumerate() {
        let rect = [i as f64 * w, 0.0, w, *p * 100.0];
        rectangle(color, rect, c.transform, g);
    }
    rectangle([0.0, 1.0, 0.0, 1.0], [0.0, economy.start_fortune * 100.0 - 2.0, 1000.0, 2.0],
        c.transform, g);
    rectangle([0.0, 0.0, 1.0, 1.0], [0.0, 98.0, 1000.0, 2.0],
        c.transform, g);
}
