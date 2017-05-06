use std::thread;

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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread1_test() {
        thread1();
    }
}
