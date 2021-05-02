extern crate myrust;
use myrust::compilation_error;
use myrust::Verbose;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::mpsc::Sender;
use std::collections::HashSet;

fn main() {
    {
        let t = thread::spawn(|| {
            for i in 1..=10 {
                println!("thread loop war is {}", i);
            }
        });
        for i in 1..=10 {
            println!("main loop war is {}", i);
        }
        t.join().unwrap(); // join Guarantees visibility in of changes in thread after join call
    }
    {
        // move values in thread
        let v = vec![1, 2, 3];
        let t = thread::spawn(move || {
            // Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid.
            // So we need this closure to take ownership of the captured value
            println!("vector in stack of main function is {:?}", v);
        });

        compilation_error!(
            std::mem::drop(v); // use of moved value: `v`
        );

        t.join().unwrap();
    }
    {
        // get result value from thread
        let computation = thread::spawn(||{
            42
        });
        let result = computation.join().unwrap();
        println!("Computed in other thread {}", result);
    }
    {
        // A channel in programming has two halves: a transmitter and a receiver.
        // A channel is said to be closed if either the transmitter or receiver half is dropped.
        let (tx, rx) = mpsc::channel(); // mpsc stands for "multiple producer, single consumer"
        thread::spawn(move || {                       // 'move' for 'tx'
            match tx.send(Verbose::new(1)) {   // 'send' consumes
                Err(e) => println!("Cannot send data {}", e),
                Ok(_) => println!("Data is sent")
            }
        });
        println!("Now block on getting result");
        let v = rx.recv().unwrap();
        println!("Receive {:?} from other thread", v);
        // output:
        // Now block on getting result
        // Data is sent                                    # Data is send after blocking on waiting for result
        // Receive Verbose { id: 1 } from other thread
        // Dropping Verbose { id: 1 }                      # Object is dropped when it had gone out of scope, it was moved to outer scope
    }
    {
        // sending multiple values via channel
        let (tx, rx) = mpsc::channel();
        let computation = thread::spawn(move || {
            let values_to_send = vec![
                "first message", "second message", "third message"
            ];
            for value in values_to_send {
                match tx.send(value) {
                    Err(e) => println!("Cannot send: {}", e),
                    Ok(_) => println!("Sent successfully")
                }
                thread::sleep(Duration::from_secs(1));
            }
            tx
        });

        let tx = computation.join().unwrap(); // 'tx' is still valid
        thread::spawn(move || {
            tx.send("こんにちは、世界！")
        });

        for received in rx { // 'rx.into_iter()' consumes 'rx', 'rx.inter()' will not close 'rx'
            println!("{}", received);
        }
        // 'rx' is not valid any more
    }
    {
        // multiple producers
        let (tx, rx) = mpsc::channel();

        fn send_an_array<T>(ar: Vec<T>, tx: Sender<T>) {
            for x in ar {
                tx.send(x).unwrap();
            }
        }

        thread::spawn((|copy_tx| { move || send_an_array(vec![1, 3, 5, 7, 9 ], copy_tx) }) (tx.clone()));
        thread::spawn((|copy_tx| { move || send_an_array(vec![2, 4, 6, 8, 10], copy_tx) }) (tx)); // Not 'tx.clone()', only 2 transmitters needed

        let mut storage = HashSet::new();
        for sent_value in rx { // 'rx.into_iter()' blocks until all transmitters are not closed
            println!("Get: {}", sent_value);
            storage.insert(sent_value);
        }
        assert_eq!(storage, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter().cloned().collect());
    }
}
