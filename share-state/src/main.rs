use std::{sync::{Arc, Mutex}, thread};

fn main() {

    let mutex = Mutex::new(5);
    
    {
        let mut num = mutex.lock().unwrap(); //when goes out of the scope, free the lock
        *num = 6;
    }
    println!("mutex = {:?}", mutex);
    
    let counter = Arc::new(Mutex::new(0)); //Arc is atomic ref, like AtomicReference<T> in Java
    let mut handles = vec![];
    //Mutex<T> can be comparers with RefCell<T>
    //and Arc<T> can be comparers with Rc<T>
    //but Mutex<T> and Arc<T> is thread safe

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
