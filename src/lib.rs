pub struct Elo {
    options: Options,
}

#[derive(Debug, Clone, Copy)]
pub struct RatingChange {
    pub change: f64,
    pub new_rating: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Options {
    k: f64,
    rating: f64, // Use f64 for ratings
}

#[derive(Debug, Clone, Copy)]
pub struct Result {
    pub opponent_rating: f64, // Use f64 for ratings
    pub result: f64,
}

#[derive(Debug, Clone)]
pub struct PerformanceRating {
    games: i32,
    change: Vec<RatingChange>,
    ratings: Vec<f64>, // Use f64 for ratings
    tpr: f64,
}

impl Elo {
    pub fn new(options: Options) -> Self {
        Elo { options }
    }

    fn _change(
        &self,
        player_rating: f64,
        opponent_rating: f64,
        k_factor: f64,
        result: f64,
    ) -> RatingChange {
        let transform_pr = f64::powf(10.0, player_rating / 400.0);
        let transform_or = f64::powf(10.0, opponent_rating / 400.0);
        let expectation = transform_pr / (transform_pr + transform_or);
        let outcome = player_rating + k_factor * (result - expectation);

        RatingChange {
            change: outcome - player_rating,
            new_rating: outcome.round() as i32,
        }
    }

    fn _max_rating(&self, player_rating: f64, opponent_rating: f64) -> f64 {
        let diff = opponent_rating - player_rating;
        if diff > 400.0 {
            player_rating + 400.0
        } else if opponent_rating < 400.0 {
            player_rating - 400.0
        } else {
            opponent_rating
        }
    }

    fn _probability(&self, player_rating: f64, opponent_rating: f64) -> f64 {
        let diff = opponent_rating - player_rating;
        1.0 / (1.0 + f64::powi(10.0, (diff / 400.0) as i32))
    }

    fn _performance(&self, results: &[Result]) -> PerformanceRating {
        let perf = results.iter().fold(
            PerformanceRating {
                games: 0,
                change: Vec::new(),
                ratings: Vec::new(),
                tpr: 0.0,
            },
            |mut pre, cur| {
                let r = self._max_rating(self.options.rating, cur.opponent_rating);
                let c = self._change(self.options.rating, r, self.options.k, cur.result);

                let tpr = pre.ratings.iter().chain(std::iter::once(&r)).sum::<f64>() as f64
                    / (pre.games + 1) as f64;

                pre.games += 1;
                pre.change.push(c);
                pre.ratings.push(r);
                pre.tpr = tpr;
                pre
            },
        );

        perf
    }

    pub fn performance_rating(&self, results: &[Result]) -> PerformanceRating {
        self._performance(results)
    }

    pub fn change(&self, opponent_rating: f64, result: f64) -> RatingChange {
        self._change(self.options.rating, opponent_rating, self.options.k, result)
    }

    pub fn probability(&self, opponent_rating: f64) -> f64 {
        self._probability(self.options.rating, opponent_rating)
    }
}
