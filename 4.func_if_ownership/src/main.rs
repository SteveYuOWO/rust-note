fn func(x: i32) -> i32 {
  println!("func() {}", x);
  100
}
fn main() {
  // func
  let res = func(5); // call func()

  println!("The value of res is: {}", res);

  let y = { // block
    let x = 1;
    x + 3
  };

  println!("The value of y is: {}", y);


  // if else
  let a = 3;
  if a == 1 {
    println!("a is 1");
  } else if a == 2 {
    println!("a is 2");
  } else {
    println!("a is not 1 or 2");
  }

  let condition = true;
  let number = if condition { 5 } else { 6 };
  println!("The value of number is: {}", number);

  // loop
  let mut counter = 0;
  loop {
    counter += 1;
    println!("loop");
    if counter == 4 {
      break;
    }
  }

  // while
  while counter < 8 {
    println!("while");
    counter += 1;
  }

  // for
  let arr = [1, 2, 3, 5, 8, 13];
  for element in arr.iter() {
    println!("for: {}", element);
  }

  for num in (1..4).rev() {
    println!("for range: {}", num);
  }

  // stack vs heap
  let mut s = String::from("hello");
  s.push_str(", world!");
  println!("The value of s is: {}", s);

  let s = String::from("HW");
  take_ownership(s); // ownership
  let x = 5;
  take_copy(x); // x is copy
  println!("The value of x is: {}", x);


  let x = String::from("Hello");
  take_refrence(&x); // refrence
  println!("After take refrence is: {}", x);

  let mut x = String::from("Hello");
  take_mut_refrence(&mut x); // mut refrence
  println!("After take mut refrence is: {}", x);

  let a = String::from("Hello");
  let s0 = &a;
  let s1 = &a;

  println!("The value of s2 is: {} {}", s0, s1);

  let mut a = String::from("Hello");
  let s2 = &mut a;
  println!("The value of s2 is: {}", s2);
}

fn take_refrence(x: &String) { // borrowing
  println!("Take refrence is: {}", x);
}

fn take_mut_refrence(x: &mut String) { // borrowing
  x.push_str(", world!");
}

fn take_ownership(some_string: String) {
  println!("{}", some_string);
}

fn take_copy(some_integer: i32) {
  println!("{}", some_integer);
}
