fn main() {
  fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
        return &s[..i];
      }
    }
    &s[..]
  }
  // slice
  let s = String::from("hello world");
  let h = &s[..5];
  let w = &s[6..];
  let hw = &s[..];
  println!("{} {} {}", h, w, hw);
  let word_index = first_word(&s);
  println!("The first word is: {}", word_index);

  let s = String::from("helloworld");
  let word_index = first_word(&s);
  println!("The first word is: {}", word_index);

  let word_index = first_word(&"guys you are handsome");
  println!("The first word is: {}", word_index);

  let a = [1, 2, 3, 4, 5];
  let slice = &a[1..3];
  println!("{:?}", slice);

  struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
  }

  let mut user1 = User {
    email: String::from("a@a.com"),
    username: String::from("a"),
    sign_in_count: 1,
    active: true,
  };

  let user2 = User { 
    email: String::from("h@h.com"),
    username: String::from("h"),
    ..user1 
  };

  user1.email = String::from("steve@steve.com");
  println!("{} {} {} {}", user1.email, user1.username, user1.sign_in_count, user1.active);
  println!("{} {} {} {}", user2.email, user2.username, user2.sign_in_count, user2.active);

  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
  println!("{} {} {}", black.0, black.1, black.2);
  println!("{} {} {}", origin.0, origin.1, origin.2);

  // derive debug
  #[derive(Debug)]
  struct Rectangle {
    width: u32,
    height: u32,
  }

  impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }
  }

  impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
      self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
      Rectangle { width: size, height: size }
    }
  }

  let rect0 = Rectangle {
    width: 30,
    height: 50,
  };
  let rect1 = Rectangle {
    width: 10,
    height: 40,
  };
  println!("{:?}", rect0);
  println!("{:#?}", rect0);
  println!("{} {} {}", rect0.width, rect0.height, rect0.area());
  println!("{}", rect0.can_hold(&rect1));

  let rec = Rectangle::square(3);
  println!("{:?}", rec);


  // enum
  #[derive(Debug)]
  enum IpAddrKind {
    V4, V6
  }

  #[derive(Debug)]
  struct IpAddr {
    kind: IpAddrKind,
    address: String,
  }
  
  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("Home"),
  };

  println!("{:?}", home);
  println!("{:?} {:?} {:?} {:?}", IpAddrKind::V4, IpAddrKind::V6, home.kind, home.address);
  
  // enum with data
  #[derive(Debug)]
  enum Message {
    Write(String),
  }

  impl Message {
    fn call(&self) {
      println!("{:?}", self);
    }
  }

  let msg = Message::Write(String::from("Hello"));
  msg.call();

  // option
  let some_number = Some(5);
  let some_string = Some("a string");
  let absent_number: Option<i32> = None;

  print!("{:?} {:?} {:?}", some_number, some_string, absent_number);

  let x: i8 = 5;
  let y: Option<i8> = Some(5);
  let z: Option<i8> = None;
  
  println!("{:?} {:?} {:?}", x, y, z);

  // match
  #[derive(Debug)]
 enum UsState {
    Alabama,
    Alaska,
  }

  #[derive(Debug)]
  enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
  }

  fn value_in_cents(coin: Coin) -> u8 {
    match coin {
      Coin::Penny => 1,
      Coin::Nickel => 5,
      Coin::Dime => 10,
      Coin::Quarter(state) => {
        println!("State quarter from {:?}!", state);
        25
      },
    }
  }

  let coin = Coin::Quarter(UsState::Alaska);
  let coin5 = Coin::Quarter(UsState::Alabama);
  let coin2 = Coin::Penny;
  let coin3 = Coin::Dime;
  let coin4 = Coin::Nickel;

  println!("{} {} {} {} {}", value_in_cents(coin), value_in_cents(coin2), value_in_cents(coin3), value_in_cents(coin4), value_in_cents(coin5));

  let five = Some(5);

  fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
      None => None,
      Some(i) => Some(i + 1),
    }
  }

  plus_one(five);


  let v = Some(3u8);

  match v {
    Some(3) => println!("three"),
    _ => (),
  }

  // if let
  if let Some(3) = v {
    println!("three");
  } else if let Some(4) = v {
    println!("four");
  } else {
    println!("not three or four");
  }
}

