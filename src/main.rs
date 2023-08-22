fn main() {
    println!("Hello, world!");
    // write a basic for loop in rust
    for i in 0..=2 {
        println!("The value of i is: {}", i);
    }

    let arr = [10, 20, 30, 40, 50];
    for x in &arr {
        println!("The value of x is: {}", x);
    }

    let vec = vec![1, 4, 6, 7, 8]
        .iter()
        .map(|x| x * 2)
        .collect::<Vec<i32>>();
    // loop over the vec
    for x in &vec {
        println!("The value of x is: {}", x);
    }

    let val = foo(1.0, 2.0).powf(2.0).sqrt();
    println!("The value of val is: {}", val);

    // closure with type anotation that is not unused
    let closure = |x: i32| -> i32 { x * 2 };
    println!("The value of closure is: {}", closure(2));

    loop {
        println!("Hello, world!");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn foo(arg1: f64, arg2: f64) -> f64 {
    println!("The value of arg1 is: {}", arg1);
    println!("The value of arg2 is: {}", arg2);
    return arg1 + arg2;
}

// Rust class: data and behavior are defined in two blocks

// struct Foo {
//     x: i32,
//     y: i32,
//     pub z: i32,
// }

// impl Foo {
//     fn foo() -> Foo {
//         return Foo { x: 1, y: 2, z: 3 }
//     }
//     pub fn pub_foo() -> Foo {
//      return   Foo { x: 1, y: 2, z: 3 }
//     }
//     fn bar(&self) -> i32 {
//         return self.x + self.y + self.z;
//     }
//     fn bar_mut(&mut self) -> i32 {
//         self.x = 10;
//         return self.x + self.y + self.z;
//     }
// }