use crate::navigation::try_to_escape;
use crate::office::Office;
use rand::Rng;

pub struct SimulationParams {
    pub sample_count: u32,
    pub office_width: usize,
    pub office_height: usize,
}

pub fn safe_percentage_for_p(p: f32, params: &SimulationParams) -> f32 {
    let mut rng = rand::thread_rng();
    let mut safe_offices = 0;
    for _ in 0..params.sample_count {
        let office = Office::new(params.office_width, params.office_width, p);
        let start_column = rng.gen_range(0, params.office_width);

        if try_to_escape(&office, start_column).is_ok() {
            safe_offices += 1
        }
    }
    safe_offices as f32 / params.sample_count as f32 * 100.0
}
