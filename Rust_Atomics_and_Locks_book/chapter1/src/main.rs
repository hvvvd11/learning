use core::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
  // Function 10 - RefCell

  fn f4(a: &RefCell<Vec<i32>>) {
    a.borrow_mut().push(1);
  }

  let test_value: i32 = 100;
  let cell_test_value = Cell::new(test_value);

  f3(&cell_test_value, &cell_test_value);
  // Function 9 - Cell

  fn f3(a: &Cell<i32>, b: &Cell<i32>) {
    let before = a.get();
    b.set(b.get() + 1);

    let after = a.get();
    if before != after {
      println!("A and B is the same value");
      return ();
    }
  }

  // Function 8
  //
  // f2(&test_value, &mut test_value); !!! IMPOSIBLE

  fn f2(a: &i32, b: &mut i32) {
    let before = *a;
    *b += 1;
    let after = *a;

    if before != after {
      println!("This should not be reachable");
      return ();
    }
  }

  //Function 7

  let a = Arc::new([1, 2, 3]);
  thread::spawn(move || dbg!(a)).join().unwrap();

  println!("_______________");
  //Function 6 - Atomical Reference Counting
  let a = Arc::new([1, 2, 3]);
  let b = a.clone();

  assert_eq!(a.as_ptr(), b.as_ptr());

  //Function 5 - Reference Counting
  let a = Rc::new([1, 2, 3]);
  let b = a.clone();

  assert_eq!(a.as_ptr(), b.as_ptr());

  //Function 4 - Scoped
  let numbers = vec![1, 2, 3];

  thread::scope(|s| {
    s.spawn(|| {
      println!("{}", numbers.len());
    });

    s.spawn(|| {
      for n in &numbers {
        println!("{n}");
      }
    });
  });

  println!("_______________");
  //Function 3
  let numbers = Vec::from_iter(0..=800);

  let t = thread::spawn(move || {
    let len = numbers.len();
    let sum = numbers.into_iter().sum::<usize>();
    sum / len
  });

  let average = t.join().unwrap();
  println!("{average}");

  println!("_______________");
  //Function 2:
  let numbers = vec![1, 2, 3];

  thread::spawn(move || {
    for num in numbers {
      println!("{num}")
    }
  })
  .join()
  .unwrap();

  println!("_______________");
  //Function 1:
  let t1 = thread::spawn(f1); //used a differenct function, instead of a closures.
  let t2 = thread::spawn(f1);

  t1.join().unwrap();
  t2.join().unwrap();
}

fn f1() {
  println!("Hello from thread");

  let id = thread::current().id();
  println!("Thread id: {id:?}");
}
