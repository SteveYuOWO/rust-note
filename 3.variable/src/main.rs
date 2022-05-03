fn main() {
  let mut x = 5; // mut means mutable
  let y = 6; // immutable
  const CONSTANT_Z: i32 = 7; // const means constant

  println!("The value of x, y, CONSTANT_Z is: {}, {}, {}", x, y, CONSTANT_Z);

  x = 6;
  println!("The value of x is: {}", x);

  let x = x + 1; // shadowing x before
  println!("The value of x is: {}", x);

  let spaces = "    ";
  let spaces = spaces.len();
  println!("Space length: {}", spaces);

  // need u32 to match the type of guess
  let guess: u32 = "42".parse().expect("Not a number!");
  println!("The value of guess is: {}", guess);

  // 8 16 64 size
  // i u
  let x: i8 = -10;
  let y: u8 = 5;
  let z: i16 = 100;
  let a: usize = 100;

  println!("The value of x is: {} {} {} {}", x, y, z, a);

  let x = 98_222;
  println!("The value of x is: {}", x);

  let x = 0xff;
  println!("The value of x is: {}", x);

  let x = 0o7_7;
  println!("The value of x is: {}", x);

  let x = 0b1111_0000;
  println!("The value of x is: {}", x);

  let x = b'A'; // binary
  println!("The value of x is: {}", x);

  // boolean
  let t = true;
  let f: bool = false;

  println!("The value of t is: {}, f is: {}", t, f);

  // char
  let m: char = 'z';
  let emoji: char = '🤔';
  println!("The value of m is: {}, emoji is: {}", m, emoji);

  // tuple
  let tuple: (char, char) = ('😂', '🏠');
  println!("The value of tuple is: {}, {}", tuple.0, tuple.1);

  let n:(i32, i32, i32) = (1, 2, 3);
  let (a, b, c) = n;
  println!("The value of n is: {}, {}, {}", a, b, c);

  // array
  let array: [i32; 6] = [1, 2, 3, 5, 8, 13];
  println!("The value of array is: {:?}", array);
  println!("length: {}", array.len());

  let mut vec = vec![1, 2, 3, 5, 8, 13];
  vec.push(21);
  println!("The value of vec is: {:?}", vec);
}
