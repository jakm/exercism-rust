use std::collections::HashMap;

use crossbeam::channel::{bounded, Receiver, Sender};
use crossbeam::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let (total_snd, total_rcv) = bounded(1);

    thread::scope(|s| {
        let (text_snd, text_rcv) = bounded(worker_count);
        let (freq_snd, freq_rcv) = bounded(worker_count);

        // workers
        for _ in 0..worker_count {
            let text_rcv = text_rcv.clone();
            let freq_snd = freq_snd.clone();
            s.spawn(|_| freq_counter(text_rcv, freq_snd));
        }

        // collector
        {
            let freq_rcv = freq_rcv.clone();
            let total_snd = total_snd.clone();
            s.spawn(move |_| {
                let mut freq: HashMap<char, usize> = HashMap::new();
                for res in freq_rcv.iter() {
                    for (k, v) in res.iter() {
                        let cnt = freq.entry(*k).or_insert(0);
                        *cnt += v;
                    }
                }
                total_snd.send(freq).unwrap();
            });
        }

        // producer
        {
            let text_snd = text_snd.clone();
            s.spawn(move |_| {
                for text in input {
                    text_snd.send(text.to_string()).unwrap();
                }
            });
        }
    })
    .unwrap();

    total_rcv.recv().unwrap()
}

pub fn freq_counter(text_rcv: Receiver<String>, freq_snd: Sender<HashMap<char, usize>>) {
    let mut freq = HashMap::new();

    for text in text_rcv.iter() {
        for c in text
            .to_lowercase()
            .chars()
            .filter(|c| !c.is_ascii_punctuation() && !c.is_digit(10))
        {
            let cnt = freq.entry(c).or_insert(0);
            *cnt += 1;
        }
    }

    freq_snd.send(freq).unwrap();
}
