mod direction;
mod navigation;
mod office;
mod show_path;
mod stats;

use navigation::WallHuggingNavigator;
use stats::{safe_pct_for_p, SimulationParams};

fn main() {
    let params = SimulationParams {
        sample_count: 10_000,
        office_width: 10,
        office_height: 10,
        print_path: false,
    };
    let mut nav = WallHuggingNavigator {};

    println!("Number of samples for each p: {:?}", params.sample_count);
    println!("  p | safety percentage");
    println!("{:?} | {:?}%", 0.0, safe_pct_for_p(0.0, &params, &mut nav));
    println!("{:?} | {:?}%", 0.1, safe_pct_for_p(0.1, &params, &mut nav));
    println!("{:?} | {:?}%", 0.2, safe_pct_for_p(0.2, &params, &mut nav));
    println!("{:?} | {:?}%", 0.3, safe_pct_for_p(0.3, &params, &mut nav));
    println!("{:?} | {:?}%", 0.4, safe_pct_for_p(0.4, &params, &mut nav));
    println!("{:?} | {:?}%", 0.5, safe_pct_for_p(0.5, &params, &mut nav));
    println!("{:?} | {:?}%", 0.6, safe_pct_for_p(0.6, &params, &mut nav));
    println!("{:?} | {:?}%", 0.7, safe_pct_for_p(0.7, &params, &mut nav));
    println!("{:?} | {:?}%", 0.8, safe_pct_for_p(0.8, &params, &mut nav));
    println!("{:?} | {:?}%", 0.9, safe_pct_for_p(0.9, &params, &mut nav));
    println!("{:?} | {:?}%", 1.0, safe_pct_for_p(1.0, &params, &mut nav));
}
