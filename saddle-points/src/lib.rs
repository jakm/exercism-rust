pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let num_columns = if let Some(row) = input.get(0) {
        row.len()
    } else {
        return vec![];
    };
    let mut min_columns = vec![u64::MAX; num_columns];
    let mut max_rows = vec![0; input.len()];
    for (i, row) in input.iter().enumerate() {
        let mut max = 0;
        for (j, elem) in row.iter().enumerate() {
            let elem = *elem;
            if elem >= max {
                max = elem;
            }
            if elem < min_columns[j] {
                min_columns[j] = elem;
            }
        }
        max_rows[i] = max;
    }

    let mut res: Vec<(usize, usize)> = Vec::new();
    for (i, row) in input.iter().enumerate() {
        for (j, elem) in row.iter().enumerate() {
            let elem = *elem;
            if elem == max_rows[i] && elem == min_columns[j] {
                res.push((i, j));
            }
        }
    }

    res
}
