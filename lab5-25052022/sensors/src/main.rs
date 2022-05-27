mod cyclic_barrier;

use std::sync::Arc;
use crate::cyclic_barrier::cb;
use std::sync::mpsc::channel;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use rand::Rng;
use crate::cb::CyclicBarrier;

fn read_value() -> i32 {
    let wait_secs = rand::thread_rng().gen_range(0..20);
    let sensor_value = rand::thread_rng().gen_range(0..10);
    sleep(Duration::from_secs(wait_secs));
    sensor_value
}

fn set_speed(tot: i32, threshold: i32) {
    let mut range = 0..10;
    if tot >= threshold { range =  10..20; }
    let wait_secs = rand::thread_rng().gen_range(range);
    println!("Total is {} - Speed set to {}", tot, wait_secs);
    sleep(Duration::from_secs(wait_secs));
}

fn channel_impl() {
    let n = 10;
    let mut tv = Vec::new();

    loop {
        let (tx, rx) = channel();
        let mut tot = 0;

        (0..n).for_each(|i| {
            let txc = tx.clone();
            tv.push(thread::spawn(move || {
                let r = read_value();
                println!("Thread {} sent {}", i, r);
                txc.send(r).unwrap();
            }))
        });

        for _ in 0..n {
            let r = rx.recv().unwrap();
            println!("Consumed {} from channel", r);
            tot += r;
        }
        set_speed(tot, 50);
    }
}

fn barrier_impl() {
    let n = 10;

    loop {
        let mut tv = Vec::new();
        let barrier = Arc::new(CyclicBarrier::new(n));
        let mut tot = 0;

        (0..n).for_each(|i| {
            let barrier_clone = barrier.clone();
            tv.push(thread::spawn(move || {
                let r = read_value();
                println!("Thread {} sent {}", i, r);
                barrier_clone.wait();
                r
            }))
        });

        for t in tv {
            let r = t.join().unwrap();
            println!("Consumed {} from channel", r);
            tot += r;
        }
        set_speed(tot, 50);
    }

}

fn main() {
    barrier_impl();
    // channel_impl();
}
