#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_change() {
        let options = Options {
            k: 20.0,
            rating: 1200.0,
        };
        let elo = Elo::new(options);
        let opponent_rating = 1600.0;
        let result = 1.0; // Win

        let rating_change = elo.change(opponent_rating, result);

        assert_eq!(rating_change.change, 10.0); // Change should be 10
        assert_eq!(rating_change.new_rating, 1210); // New rating should be 1210
    }

    #[test]
    fn test_probability() {
        let options = Options {
            k: 20.0,
            rating: 1200.0,
        };
        let elo = Elo::new(options);
        let opponent_rating = 1400.0;

        let probability = elo.probability(opponent_rating);

        assert!((probability - 0.359935).abs() < 0.000001); // Probability should be approximately 0.359935
    }

    #[test]
    fn test_performance_rating() {
        let options = Options {
            k: 20.0,
            rating: 1200.0,
        };
        let elo = Elo::new(options);
        let results = vec![
            Result {
                opponent_rating: 1300.0,
                result: 1.0,
            }, // Win
            Result {
                opponent_rating: 1400.0,
                result: 0.5,
            }, // Draw
            Result {
                opponent_rating: 1500.0,
                result: 0.0,
            }, // Loss
        ];

        let performance_rating = elo.performance_rating(&results);

        assert_eq!(performance_rating.games, 3); // 3 games played
        assert_eq!(performance_rating.change.len(), 3); // 3 rating changes
        assert_eq!(performance_rating.ratings.len(), 3); // 3 different opponent ratings
        assert!((performance_rating.tpr - 1299.666666).abs() < 0.000001); // TPR should be approximately 1299.666666
    }
}
