use rust_tcp_sever::*;

fn main() {
    {
        let thread_pool = ThreadPool::new(5);

        // Will work.
        thread_pool.add(|| println!("test/1 1"));
        thread_pool.add(|| println!("test/1 2"));
        thread_pool.add(|| println!("test/1 3"));
        thread_pool.add(|| println!("test/1 4"));
        thread_pool.add(|| println!("test/1 5"));
        
        // Will wait for the code above to execute.
        thread_pool.add(|| println!("test/1 6"));
    }

    {
        // Everything will work.
        ThreadPool::launch(vec![
            || println!("test/2 1"),
            || println!("test/2 2"),
            || println!("test/2 3"),
            || println!("test/2 4"),
            || println!("test/2 5"),
            || println!("test/2 6"),
        ]);
    }
}
