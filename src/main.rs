mod direction;
mod navigation;
mod office;
mod stats;

use stats::{safe_percentage_for_p, SimulationParams};

fn main() {
    let params = SimulationParams {
        sample_count: 10_000,
        office_width: 10,
        office_height: 10,
    };

    println!("Number of samples for each p: {:?}", params.sample_count);
    println!("  p | safety percentage");
    println!("{:?} | {:?}%", 0.0, safe_percentage_for_p(0.0, &params));
    println!("{:?} | {:?}%", 0.1, safe_percentage_for_p(0.1, &params));
    println!("{:?} | {:?}%", 0.2, safe_percentage_for_p(0.2, &params));
    println!("{:?} | {:?}%", 0.3, safe_percentage_for_p(0.3, &params));
    println!("{:?} | {:?}%", 0.4, safe_percentage_for_p(0.4, &params));
    println!("{:?} | {:?}%", 0.5, safe_percentage_for_p(0.5, &params));
    println!("{:?} | {:?}%", 0.6, safe_percentage_for_p(0.6, &params));
    println!("{:?} | {:?}%", 0.7, safe_percentage_for_p(0.7, &params));
    println!("{:?} | {:?}%", 0.8, safe_percentage_for_p(0.8, &params));
    println!("{:?} | {:?}%", 0.9, safe_percentage_for_p(0.9, &params));
    println!("{:?} | {:?}%", 1.0, safe_percentage_for_p(1.0, &params));
}
