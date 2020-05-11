pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let generator = SpiralGenerator::new(size);
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    for (x, y, number) in generator {
        matrix[x][y] = number;
    }
    matrix
}

pub struct SpiralGenerator {
    size: u32,
    number: u32,
    steps: u32,
    position: (i16, i16),
    increments: Vec<(i16, i16)>,
    pointer: usize,
}

impl SpiralGenerator {
    pub fn new(size: u32) -> Self {
        if size > 32768 {
            panic!("Too large size")
        }
        Self {
            size,
            number: 0,
            position: (0, 0),
            increments: vec![(0, 1), (1, 0), (0, -1), (-1, 0)],
            pointer: 0,
            steps: 0,
        }
    }
}

impl Iterator for SpiralGenerator {
    type Item = (usize, usize, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.size == 0 {
            return None;
        }

        self.number += 1;
        self.steps += 1;

        if self.steps >= self.size {
            self.steps = 0;
            self.pointer = (self.pointer + 1) % 4;
            if self.pointer % 2 == 1 {
                // decrease size when preparing to generate numbers in columns
                self.size -= 1;
            }
        }

        assert!(self.position.0 >= 0 && self.position.1 >= 0);

        let current = (self.position.0 as usize, self.position.1 as usize);

        self.position.0 += self.increments[self.pointer].0;
        self.position.1 += self.increments[self.pointer].1;

        Some((current.0, current.1, self.number))
    }
}
