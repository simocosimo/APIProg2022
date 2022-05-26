extern crate core;

use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut hm = HashMap::new();
    let mut tv = Vec::new();
    let (tx, rx) = channel();

    if worker_count == 0 { panic!("worker_count cannot be zero!"); }
    if input.len() == 0 { return  hm; }

    (0..worker_count).for_each(|i| {
        let mut rows = Vec::new();
        for s in input.iter().skip(i).step_by(worker_count) {
            rows.push(s.to_string().to_lowercase());
        }
        let txc = tx.clone();
        tv.push(thread::spawn(move || {
            let mut local_map = HashMap::new();
            rows.iter().for_each(|s| {
                s.chars().filter(|c| c.is_alphabetic()).for_each(|c| {
                    local_map.entry(c)
                        .and_modify(|v| *v += 1)
                        .or_insert(1 as usize);
                })
            });
            txc.send((local_map, i)).unwrap();
        }));
    });

    for _ in 0..worker_count {
        let tr = rx.recv().unwrap();
        (tr.0).iter().for_each(|e| {
            hm.entry(*e.0)
                .and_modify(|v| *v += e.1)
                .or_insert(*e.1);
        })
    }

    hm
}
