use std::sync::{Mutex, Arc};
use std::thread;
use std::ops::Div;

// Any type T is 'Sync' if &T (a reference to T) is 'Send'.
// Similar to 'Send', primitive types are 'Sync', and types composed entirely of types that are 'Sync' are also 'Sync'.

fn main() {
    {
        let m = Mutex::new(5);
        {
            let mut data = m.lock().unwrap(); // 'lock()' panics if mutex is locked
            *data = 6;
            let x = data.div(2); // "deref coercion" works
            println!("{} divided by 2 is {}", data, x);
            // data of mutex is guarded
        }
        println!("mutex {:?}", m);
    }
    {
        let shared_counter = Arc::new(Mutex::new(0));
        let threads: Vec<_> = (0..10).map(|i| {
            let counter = Arc::clone(&shared_counter);
            thread::spawn(move ||{
                println!("thread {}", i);
                let mut value = counter.lock().unwrap(); // Mutex<T> provides interior mutability
                *value += 1;
            })
        }).collect();

        for t in threads {
            t.join().unwrap();
        }

        println!("counter is {:?}", shared_counter.lock().unwrap());
    }
}
