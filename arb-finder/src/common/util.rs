pub fn calculate_ev_percentage(odds: f64, true_probability: f64) -> f64 {
    return 100.0 * (odds * true_probability) - 100.0;
}
