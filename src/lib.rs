pub mod batched {
    use std::fmt::Debug;
    use itertools::Itertools;

    pub fn with_crossbeam<I, T, F>(iter: I, batch_size: usize, f: F)  
    where  
        I: Iterator<Item = T>,
        T: Debug + Send + Sync,
        F: Fn(&T) -> () + Send + Sync + Copy,
    {
        
        let _ = crossbeam::scope(|s| {
            for data in &iter.chunks(batch_size) {
                let mut threads = vec![];
                for (i, d) in data.enumerate() {
                    let t = s.spawn(move |_| {
                        println!("{:?}: processing {:?}", i, d);
                        f(&d)
                    });
                    threads.push(t);
                }
                for t in threads {
                    t.join().unwrap();
                }
            }
        });
    }
}

