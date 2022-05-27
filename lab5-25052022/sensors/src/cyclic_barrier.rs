pub mod cb {
    use std::sync::{Condvar, Mutex};

    pub struct CyclicBarrier {
        th_number: usize,
        wait_count: Mutex<(usize, bool)>,
        cv: Condvar,
    }

    impl CyclicBarrier {
        pub fn new(n: usize) -> Self {
            Self {
                th_number: n,
                wait_count: Mutex::new((0, true)),
                cv: Condvar::new()
            }
        }

        /* === RUST IMPLEMENTATION OF BARRIER ===
        pub fn wait(&self) -> BarrierWaitResult {
            let mut lock = self.lock.lock().unwrap();
            let local_gen = lock.generation_id;
            lock.count += 1;
            if lock.count < self.num_threads {
                while local_gen == lock.generation_id {
                    lock = self.cvar.wait(lock).unwrap();
                }
                BarrierWaitResult(false)
            } else {
                lock.count = 0;
                lock.generation_id = lock.generation_id.wrapping_add(1);
                self.cvar.notify_all();
                BarrierWaitResult(true)
            }
        }
        */

        pub fn wait(&self) {
            let mut wc = self.wait_count.lock().unwrap();

            // Waiting room for incoming threads during freeing phase
            if (*wc).1 == false {
                wc = self.cv.wait_while(wc, |p| { !(*p).1 }).unwrap();
            }

            // The party
            (*wc).0 += 1;
            if (*wc).0 == self.th_number {
                // Everyone's here, so let's wake threads
                (*wc).1 = false;
                (*wc).0 -= 1;
                self.cv.notify_all();
            } else {
                // Waiting for everyone
                wc = self.cv.wait(wc).unwrap();
                (*wc).0 -= 1;
                if (*wc).0 == 0 {
                    // Opening the waiting room
                    (*wc).1 = true;
                    self.cv.notify_all();
                }
            }
        }
    }
}