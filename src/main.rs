mod direction;
mod navigation;
mod office;
mod show_path;
mod stats;

use navigation::WallHuggingNavigator;
use stats::{stats_for_p, SimulationParams, Stats};

fn main() {
    let params = SimulationParams {
        sample_count: 10_000,
        office_width: 10,
        office_height: 10,
        print_path: false,
    };
    let mut nav = WallHuggingNavigator {};

    println!("Number of samples for each p: {:?}", params.sample_count);
    println!("  p | % | avg. length");
    print_stats_for_p(0.0, &params, &mut nav);
    print_stats_for_p(0.1, &params, &mut nav);
    print_stats_for_p(0.2, &params, &mut nav);
    print_stats_for_p(0.3, &params, &mut nav);
    print_stats_for_p(0.4, &params, &mut nav);
    print_stats_for_p(0.5, &params, &mut nav);
    print_stats_for_p(0.6, &params, &mut nav);
    print_stats_for_p(0.7, &params, &mut nav);
    print_stats_for_p(0.8, &params, &mut nav);
    print_stats_for_p(0.9, &params, &mut nav);
    print_stats_for_p(1.0, &params, &mut nav);
}

fn print_stats_for_p(p: f32, params: &SimulationParams, navigator: &mut WallHuggingNavigator) {
    let stats: Stats = stats_for_p(p, &params, navigator);
    println!(
        "{:?} | {:?}% | {:?}",
        p, stats.safe_percentage, stats.average_path_length
    );
}
