use rand::Rng;

const ALPHAS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"1234567890";

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Robot::generate_name(),
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::generate_name();
    }

    fn generate_name() -> String {
        let mut rng = rand::thread_rng();
        let mut buf = Vec::new();
        for _ in 0..2 {
            let idx = rng.gen_range(0, ALPHAS.len());
            let c = ALPHAS[idx] as char;
            buf.push(c);
        }
        for _ in 0..3 {
            let idx = rng.gen_range(0, NUMBERS.len());
            let c = NUMBERS[idx] as char;
            buf.push(c);
        }
        buf.iter().collect()
    }
}
