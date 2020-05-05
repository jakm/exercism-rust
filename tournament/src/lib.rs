extern crate itertools;

use itertools::Itertools;
use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut table = vec![String::from(
        "Team                           | MP |  W |  D |  L |  P",
    )];

    table.extend(
        match_results
            .split('\n')
            .filter(|line| !line.is_empty())
            .fold(HashMap::new(), |mut scores, line| {
                let mut fields = line.split(';');
                let team_a = fields.next().unwrap();
                let team_b = fields.next().unwrap();
                let status = fields.next().unwrap();

                scores
                    .entry(team_a)
                    .or_insert_with(Vec::new)
                    .push(match status {
                        "win" => 3,
                        "draw" => 1,
                        "loss" => 0,
                        _ => unreachable!(),
                    });
                scores
                    .entry(team_b)
                    .or_insert_with(Vec::new)
                    .push(match status {
                        "win" => 0,
                        "draw" => 1,
                        "loss" => 3,
                        _ => unreachable!(),
                    });
                scores
            })
            .drain()
            .map(|(key, mut scores)| {
                let res = scores.drain(..).fold((0, 0, 0, 0, 0), |mut acc, score| {
                    acc.0 += score; // points
                    acc.1 += 1; // matches
                    match score {
                        3 => acc.2 += 1, // won
                        1 => acc.3 += 1, // drawn
                        0 => acc.4 += 1, // lost
                        _ => unreachable!(),
                    };
                    acc
                });
                (res.0, key, res.1, res.2, res.3, res.4)
            })
            .sorted_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)))
            .map(|res| {
                format!(
                    "{:<30} | {:2} | {:2} | {:2} | {:2} | {:2}",
                    res.1, res.2, res.3, res.4, res.5, res.0
                )
            })
            .collect::<Vec<String>>(),
    );

    table.join("\n")
}
