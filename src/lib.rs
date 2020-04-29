use rand::distributions::{Distribution, Uniform};
use rand::rngs::ThreadRng;
use std::error::Error;

pub struct Simulation {
    probabilities: Vec<usize>,
    iterations: usize,
    day_dist: Uniform<u64>,
    month_dist: Uniform<u64>,
    rng: ThreadRng,
}
impl Simulation {
    pub fn new(args: Vec<String>) -> Result<Simulation, Box<dyn Error>> {
        let iterations = if args.len() < 2 {
            return Err("Must provide an iteration count".into());
        } else {
            if let Ok(val) = args[1].parse::<usize>() {
                val
            } else {
                return Err("Invalid iteration value".into());
            }
        };
        Ok(Simulation {
            probabilities: vec![35, 5, 10, 5, 5, 5, 10, 0, 5, 10, 0, 10],
            iterations,
            day_dist: Uniform::from(0..30),
            month_dist: Uniform::from(0..100),
            rng: ThreadRng::default(),
        })
    }

    pub fn start(&mut self) {
        let mut results = Vec::new();
        for _ in 0..self.iterations {
            results.push(self.simulate());
        }
        results.sort();
        println!("{:?}", results[self.iterations / 2] + 1);
    }

    fn simulate(&mut self) -> usize {
        let mut months = vec![vec![false; 30]; 12];
        let mut people = 0;
        loop {
            people += 1;
            if self.birth_person(&mut months) {
                return people;
            }
        }
    }

    fn birth_person(&mut self, months: &mut Vec<Vec<bool>>) -> bool {
        let mut month = self.month_dist.sample(&mut self.rng) as isize;
        for (idx, &prob) in self.probabilities.iter().enumerate() {
            month -= prob as isize;
            if month <= 0 {
                let day = self.day_dist.sample(&mut self.rng) as usize;
                if months[idx][day] {
                    return true;
                } else {
                    months[idx][day] = true;
                    return false;
                }
            }
        }
        false
    }
}
