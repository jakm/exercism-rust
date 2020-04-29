pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            row_count: row_count,
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(self.row_count as usize);
        for i in 0..self.row_count {
            let mut row: Vec<u32> = Vec::with_capacity((i + 1) as usize);
            for j in 0..i + 1 {
                row.push(binomial_coefficient(i, j));
            }
            rows.push(row);
        }
        rows
    }
}

pub fn binomial_coefficient(n: u32, k: u32) -> u32 {
    fact(n) / (fact(k) * fact(n - k))
}

pub fn fact(n: u32) -> u32 {
    let mut n = n;
    let mut fact = 1;
    while n > 1 {
        fact *= n;
        n -= 1;
    }
    fact
}
