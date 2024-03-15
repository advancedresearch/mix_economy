#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

/// Represents the whole economy.
///
/// Each player has a normalized fortune against an upper soft limit.
/// The difference from the upper limit is charging the economy.
///
/// The tax tells how fast to burn money of fortunes above the soft limit,
/// and how much to give each player below the soft limit per time interval.
///
/// The start fortune is given to new players.
///
/// Call `Economy::update` at regular time intervals to distribute wealth,
/// using a fixed tax rate. The Gini index can vary depending on economic activity.
///
/// Call `Economy::solve` at regular time intervals to distribute wealth,
/// using a target Gini coefficient.
/// The tax is automatically adjusted to meet the target.
#[derive(Clone)]
pub struct Economy {
    /// The fortunes of the players.
    pub players: Vec<f64>,
    /// The progressive tax factor, as the square root of fortune above 1.
    pub tax: f64,
    /// The initial fortune. Should be in the range [0, 1].
    pub start_fortune: f64,
}

impl Economy {
    /// Creates a new economy.
    pub fn new(tax: f64, start_fortune: f64, players: usize) -> Economy {
        Economy {
            players: vec![start_fortune; players],
            tax: tax,
            start_fortune: start_fortune,
        }
    }

    /// Adds a player to the economy.
    pub fn add_player(&mut self) -> usize {
        self.players.push(self.start_fortune);
        self.players.len() - 1
    }

    /// Finds the minimum and maximum fortune.
    pub fn min_max(&self) -> (f64, f64) {
        let mut min: Option<f64> = None;
        let mut max: Option<f64> = None;
        for &p in &self.players {
            min = Some(min.map(|v| if v < p { v } else { p }).unwrap_or(p));
            max = Some(max.map(|v| if v > p { v } else { p }).unwrap_or(p));
        }
        (min.unwrap_or(0.0), max.unwrap_or(0.0))
    }

    /// Find the Gini coefficient (see [Wikipedia article](https://en.wikipedia.org/wiki/Gini_coefficient)).
    pub fn gini(&self) -> f64 {
        let mut sum = 0.0;
        let n = self.players.len();
        for i in 0..n {
            for j in 0..n {
                sum += (self.players[i] - self.players[j]).abs();
            }
        }
        let mut div = 0.0;
        for j in 0..n {
            div += self.players[j] * n as f64;
        }
        sum / (2.0 * div)
    }

    /// Does a transaction between two people.
    pub fn transaction(&mut self, from: usize, to: usize, amount: f64)
    -> Result<(), ()> {
        if from == to { return Err(()); }
        let new_fortune = self.players[from] - amount;
        if new_fortune > 0.0 {
            self.players[to] += amount;
            self.players[from] = new_fortune;
            Ok(())
        } else {
            Err(())
        }
    }

    /// Updates the economy using the fixed tax rate.
    /// The Gini index can vary depending on economic activity.
    pub fn update(&mut self) {
        // Remove wealth from rich players.
        for p in &mut self.players {
            if *p >= 1.0 {
                let amount = (*p - 1.0).sqrt() * self.tax;
                *p -= amount;
            }
        }

        // Compute weights and how much to distribute.
        let mut sum_weights = 0.0;
        let mut distribute = 0.0;
        for p in &self.players {
            if *p < 1.0 {
                distribute += 1.0 - *p;
                if *p < self.start_fortune {
                    sum_weights += self.start_fortune.sqrt();
                } else {
                    sum_weights += p.sqrt();
                }
            }
        }

        // Distribute the wealth among poor players.
        for p in &mut self.players {
            if *p < 1.0 {
                if *p < self.start_fortune {
                    *p += self.start_fortune.sqrt() / sum_weights *
                        distribute * self.tax;
                } else {
                    *p += p.sqrt() / sum_weights * distribute * self.tax;
                }
            }
        }
    }

    /// Updates the economy using a target Gini coefficient.
    /// The tax is automatically adjusted to meet the target.
    /// Uses convergent binary search to find the tax.
    ///
    /// The solver is less accurate for high Gini (`~0.5` or higher) in some cases.
    /// A very low Gini (`<0.1`) might not work at all, because the algorithm
    /// is incentivizing (players that have more gets more below the upper soft limit).
    ///
    /// The `smooth_target` parameter is a value in range `[0.5, 1)`.
    /// `0.5` gives binary search behavior, which assumes strict
    /// monotonic Gini (tax should be lowered if target Gini is above).
    /// Higher values weakens the assumption, interpreted as
    /// the mix-algorithm "tends to have" monotonic Gini.
    ///
    /// The `min_tax` parameter is a value usually above 0,
    /// to prevent the solver from getting stuck in 0% scenarios.
    pub fn solve(
        &mut self,
        target_gini: f64,
        smooth_target: f64,
        min_tax: f64,
    ) {
        let mut tax = 0.0;
        let mut step = 0.5;
        loop {
            if tax > 1.0 { break; }
            let mut copy = self.clone();
            copy.tax = tax;
            copy.update();
            let gini = copy.gini();
            let diff = target_gini - gini;
            if diff > 0.0 {
                tax -= step;
            } else {
                tax += step;
            }
            step *= smooth_target;
            if step < 0.0001 { break; }
        }

        if tax < min_tax { tax = min_tax; }
        if tax > 1.0 { tax = 1.0; }
        self.tax = tax;
        self.update();
    }
}
