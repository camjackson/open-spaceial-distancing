use crate::navigation::Navigator;
use crate::office::Office;
use rand::Rng;

pub struct SimulationParams {
    pub sample_count: u32,
    pub office_width: usize,
    pub office_height: usize,
}

pub fn safe_pct_for_p<N: Navigator>(p: f32, params: &SimulationParams, navigator: &mut N) -> f32 {
    let mut rng = rand::thread_rng();
    let mut safe_offices = 0;
    for _ in 0..params.sample_count {
        let office = Office::new(params.office_width, params.office_width, p);
        let start_column = rng.gen_range(0, params.office_width);

        if navigator.try_to_escape(&office, start_column).is_ok() {
            safe_offices += 1
        }
    }
    safe_offices as f32 / params.sample_count as f32 * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::navigation::Path;

    struct FakeNavigator {
        pub result_sequence: Vec<Result<(), ()>>,
        call_count: usize,
    }

    impl FakeNavigator {
        fn new(result_sequence: Vec<Result<(), ()>>) -> FakeNavigator {
            FakeNavigator {
                result_sequence,
                call_count: 0,
            }
        }
    }

    impl Navigator for FakeNavigator {
        fn try_to_escape(&mut self, _: &Office, _: usize) -> Result<Path, Path> {
            let result_index = self.call_count % self.result_sequence.len();
            let result = self.result_sequence.get(result_index).unwrap();
            self.call_count += 1;
            if result.is_ok() {
                Ok(Vec::new())
            } else {
                Err(Vec::new())
            }
        }
    }

    fn floats_close(f1: f32, f2: f32) -> bool {
        (f1 - f2).abs() < 0.01
    }

    #[test]
    fn it_gives_zero_if_the_navigator_always_fails() {
        let params = SimulationParams {
            sample_count: 100,
            office_width: 10,
            office_height: 10,
        };
        let mut always_succeed_nav = FakeNavigator::new(vec![Err(())]);
        // p is irrelevant here, it's the navigator stub that matters
        let safe_pct = safe_pct_for_p(1.0, &params, &mut always_succeed_nav);
        assert!(floats_close(safe_pct, 0.0));
    }

    #[test]
    fn it_gives_the_sample_count_if_the_navigator_always_fails() {
        let params = SimulationParams {
            sample_count: 100,
            office_width: 10,
            office_height: 10,
        };
        let mut always_fail_nav = FakeNavigator::new(vec![Ok(())]);
        // p is irrelevant here, it's the navigator stub that matters
        let safe_pct = safe_pct_for_p(0.0, &params, &mut always_fail_nav);
        assert!(floats_close(safe_pct, 100.0));
    }

    #[test]
    fn it_gives_a_portion_of_the_sample_count_if_the_navigator_gives_mixed_results() {
        let params = SimulationParams {
            sample_count: 99,
            office_width: 10,
            office_height: 10,
        };
        // p is irrelevant here, it's the navigator stub that matters
        let mut mostly_succeed_nav = FakeNavigator::new(vec![Ok(()), Err(()), Ok(())]);
        let safe_pct = safe_pct_for_p(0.0, &params, &mut mostly_succeed_nav);
        assert!(floats_close(safe_pct, 66.66));
    }
}
