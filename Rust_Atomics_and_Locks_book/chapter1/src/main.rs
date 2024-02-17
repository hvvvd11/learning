use std::{sync::Mutex, thread, time::Duration};

fn main() {
  let n = Mutex::new(0);
  thread::scop(|s| {
    for _ in 0..10 {
      s.spawn(|| {
        let mut guard = n.lock().unwrap();
        for _ in 0..100 {
          *guard += 1;

          println!("{:?}", guard);
        }
        let thread_id = thread::current().id();
        println!("{:?}", thread_id);

        // UNLOCKS, because applied on MutexGuard type
        drop(guard);

        // BASICALLY ALL THIS 10 THREADS WOULD WAIT FOR 1 SECOND AND FREE THE SCOPE
        thread::sleep(Duration::from_secs(1))
      });
    }
  });

  assert_eq!(n.into_inner().unwrap(), 1000);
}
