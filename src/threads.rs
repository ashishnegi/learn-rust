use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn thread1() {
    let v = vec![1,2,3,4];
    let handle = thread::spawn(move || {
        println!("vector is : {:?}", v)
    });

    match handle.join() {
        Ok(_) => println!("Thread finished"),
        Err(e) => println!("Error from thread : {:?}", e)
    }
}

fn channels() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            "hello",
            "from",
            "brave",
            "new",
            "world"
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::new(0,99999));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            "bye",
            "from",
            "weak",
            "old",
            "hell"
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::new(1,99999));
        }
    });

    for v in rx {
        println!("Got: {}", v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread1_test() {
        thread1();
    }

    #[test]
    fn channels_test() {
        channels();
    }
}
