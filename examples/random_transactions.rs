//! Shows distribution curve under random transactions.

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
    start_fortune: f64,
    players: usize,
    avg_trans: f64,
    trans: u32,
    periods: u32,
    smooth_fact: f64,
}

fn main() {
    let series = &[
        (98, 0.43),
        (99, 0.43),
        (100, 0.44),
        (101, 0.44),
        (102, 0.45),
        (103, 0.45),
        (104, 0.46),
        (105, 0.46),
        (106, 0.47),
        (107, 0.47),
        (108, 0.48),
        (109, 0.48),
        (110, 0.49),
        (111, 0.49),
        (112, 0.50),
        (113, 0.50),
        (114, 0.51),
        (115, 0.51),
        (116, 0.52),
        (117, 0.52),
        (118, 0.53),
        (119, 0.53),
        (120, 0.54),
        (121, 0.55),
        (122, 0.55),
        (123, 0.56),
        (124, 0.56),
        (125, 0.57),
        (126, 0.57),
        (127, 0.58),
        (128, 0.58),
        (129, 0.59),
        (130, 0.59),
        (131, 0.60),
        (132, 0.60),
        (133, 0.61),
        (134, 0.61),
        (135, 0.62),
        (136, 0.62),
        (137, 0.63),
        (138, 0.63),
        (139, 0.64),
        (140, 0.64),
        (141, 0.65),
        (142, 0.65),
        (143, 0.66),
        (144, 0.66),
        (145, 0.67),
        (146, 0.67),
        (147, 0.68),
        (148, 0.68),
        (149, 0.69),
        (150, 0.69),
        (151, 0.70),
        (152, 0.70),
        (153, 0.71),
        (154, 0.71),
        (155, 0.72),
        (156, 0.72),
        (157, 0.73),
        (158, 0.73),
        (159, 0.74),
        (160, 0.74),
        (161, 0.75),
        (162, 0.75),
        (163, 0.76),
        (164, 0.76),
        (165, 0.77),
        (166, 0.77),
        (167, 0.78),
        (168, 0.78),
        (169, 0.79),
        (170, 0.79),
        (171, 0.80),
        (172, 0.80),
    ];

    for &(id, tax) in series.iter() {
        simulate(tmp_settings(id, tax));
    }
}

fn tmp_settings(id: u32, tax: f64) -> Settings {
    Settings {
        id: id,
        tax_1: tax,
        tax_2: 0.0,
        start_fortune: 0.2,
        players: 100,
        avg_trans: 0.1,
        trans: 1000,
        periods: 10,
        smooth_fact: 0.9999,
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
    let mut smooth = 1.0;

    let mut rng = rand::rngs::OsRng;

    let window = false;
    if window {
        let mut window: PistonWindow =
            WindowSettings::new("Random transactions", [1024, 768])
            .exit_on_esc(true)
            .build()
            .unwrap();
        window.set_bench_mode(true);
        while let Some(e) = window.next() {
            window.draw_2d(&e, |c, g, _| {
                clear([1.0, 1.0, 1.0, 1.0], g);
                draw_economy(&economy, &c, g, players);
                let d = c.trans(0.0, 500.0);
                draw_economy(&economy2, &d, g, players);
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

                economy.update();
                economy.players.sort_by(|a, b| a.partial_cmp(b).unwrap());
                economy2.update();
                economy2.players.sort_by(|a, b| a.partial_cmp(b).unwrap());
            });
            gini_timer.event(&e, || {
                smooth_gini += (economy.gini() - smooth_gini) * smooth;
                smooth_gini2 += (economy2.gini() - smooth_gini2) * smooth;
                println!("gini \ttop: {} \tbottom: {} \tsmooth: {}", smooth_gini, smooth_gini2, smooth);
                smooth *= smooth_fact;
            })
        }
    } else {
        loop {
            for _ in 0..periods {
                // Make random transactions.
                for _ in 0..transactions {
                    let len = economy.players.len();
                    let from = rng.gen_range(0..len);
                    let to = rng.gen_range(0..len);
                    let _ = economy.transaction(from, to, avg_transaction);
                    let _ = economy2.transaction(from, to, avg_transaction);
                }

                economy.update();
                // economy.players.sort_by(|a, b| a.partial_cmp(b).unwrap());
                economy2.update();
                // economy2.players.sort_by(|a, b| a.partial_cmp(b).unwrap());
            }

            smooth_gini += (economy.gini() - smooth_gini) * smooth;
            smooth_gini2 += (economy2.gini() - smooth_gini2) * smooth;
            smooth *= smooth_fact;
            if smooth < 1.0 - smooth_fact { break; }
        }

        println!("id {}", settings.id);
        println!("tax 1: {}\ttax 2: {}", settings.tax_1, settings.tax_2);
        println!("gini \ttop: {} \tbottom: {} \tsmooth: {}", smooth_gini, smooth_gini2, smooth);
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
