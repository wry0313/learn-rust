// use std::collections::{HashMap, HashSet};

// fn moo() {
//     println!("Hello, world!");
//     // write a basic for loop in rust
//     for i in 0..=2 {
//         println!("The value of i is: {}", i);
//     }

//     let arr = [10, 20, 30, 40, 50];
//     for x in &arr {
//         println!("The value of x is: {}", x);
//     }

//     let vec = vec![1, 4, 6, 7, 8]
//         .iter()
//         .map(|x| x * 2)
//         .collect::<Vec<i32>>();
//     // loop over the vec
//     for x in &vec {
//         println!("The value of x is: {}", x);
//     }

//     let val = foo(1.0, 2.0).powf(2.0).sqrt();
//     println!("The value of val is: {}", val);

//     // closure with type anotation that is not unused
//     let closure = |x: i32| -> i32 { x * 2 };
//     println!("The value of closure is: {}", closure(2));

//     loop {
//         println!("Hello, world!");
//         std::thread::sleep(std::time::Duration::from_secs(1));
//     }
// }

// fn foo(arg1: f64, arg2: f64) -> f64 {
//     println!("The value of arg1 is: {}", arg1);
//     println!("The value of arg2 is: {}", arg2);
//     return arg1 + arg2;
// }

// struct Foo {
//     x: u32,
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

// fn voo() {
//     let a = vec![1, 2, 3, 4, 5]; // ! is a macro
//                                  // let item = a[2]; // if out of bounds panic
//                                  // let item = a.get(2); // if out of bounds return Option<t>
//     let mut a = a;
//     a.push(6);
//     a.pop();
// }

// String in rust have a pointer a length and a capacity

// fn too() {
//     let a = (5, String::from("hello"));
//     println!("The value of a is: {:?}", a);
//     let (my_num, my_str) = a;
// }

// fn only_evens(x: usize) {
//     if x % 2 == 1 {
//         unreachable!("this shouldn't happen");
//     }
//     todo!("finish this later");
// }

// fn unwrap_demo() {
//     let foo = Some(5);

//     let bar = foo.unwrap();

//     let moo = foo;
// }

// fn mutable() {
//     let mut x  = 5;
//     let y = &mut x;
//     *y += 1;
// }

// fn iterate() {
//     //     let data: Vec<_> = vec![1, 2, 3]; // <_> means infer the type
//     //                                    // let v own 123
//     //     let mut new_v = data.iter().map(|x| x + 1);
//     let new_v = vec![1, 2, 3].iter().map(|x| x + 1);
//     let new_v: Vec<_> = vec![2, 2, 3].iter().map(|x| x + 1).collect(); // TEMPORARY VALUE DROP because the iter refers the the vector and then no one owns the vector now so the vector is ownerless
//                                                                        // As we've discussed previously, this is safe because the temporary vector's lifetime is extended to last the entire statement. By the end of the statement, the temporary vector has been fully consumed by the iterator and the collect() function, so there's no lifetime issue.
//                                                                        //     let mut new_new_v = vec![];
//     // collect solves the problem with calling iter() on temp 123 because collect just consumes all the iter pointer to the 123 so 123 can be safely dropped

//     // problem only arise when you call iter() and then keep the iter around after the original data is dropped
// // for example let iterator = vec![1,2,3].iter()
// // this is a prblem because the iterator will outlive the temporary vec
// // when the statement is moved on by the temp [1,2,3] will have no owner which will get dropped
// // but the iter is still referring to it

//     //     while let Some(x) = new_v.next() {
//     //         new_new_v.push(x);
//     //     }
//     print!("{:?}", new_v);
// }

// fn collect() {
//     let foo: String = vec!["hello", " ", "world"].into_iter().collect();
//     // just because the type is a string rust is smart enough to bind this string
//     println!("{:?}", foo);

//     let foo: HashSet<_> = vec![1, 2, 3, 4, 2, 3, 1].into_iter().collect();
//     // bc the type is hashset collect is smart enough to convert the target into a hashset
//     println!("{:?}", foo);

//     let foo: HashMap<&str, usize> = vec!["this", "is", "a", "map", "is", "a"]
//         .into_iter()
//         .enumerate()
//         .map(|(idx, item)| (item, idx))
//         .collect();

//     // into_iter(), for vec<T> returns T because it takes ownership
//     // iter(), for vec<T> returns &T

//     // BORROW CHECKER: into iter escapes this problem
//     // because it is not referring to the data, it TAKES OWNERSHIP OF THE DATA

//     println!("{:?}", foo);
// }

// fn iter_stuff() {
//     // let value: usize = vec![1, 2, 3].iter().sum();

//     // let value = vec![1, 2, 5, 9, 4]
//     //     .iter()
//     //     .skip(2)
//     //     .take_while(|&&x| x > 4)
//     //     .for_each(|x| println!("{}", x));
//     // Because the iterator yields &i32 (references to integers), the closure you provide to take_while gets a &&i32. The first & is because closures capture items by reference. The second & is because the item itself is a reference (as given by the iterator).
//     // So, when you write |&&x|, you are pattern matching to destructure the double reference and get to the inner i32 value. This way, x inside the closure is of type i32 and you can directly compare it with 4.

//     let value = vec![1, 2, 3].iter().filter(|&&x| x % 2 == 0).count();
//     // in rust reference is automatically deferenced with many operations when its ambiguous

//     // let value = vec![1, 2, 3].iter().skip(2).count();

//     let map = HashMap::from([("hi", 1), ("yo", 2)]);
//     let map_iter = map.iter().for_each(|(&k, v)| println!("{}:{}", k, v));
//     // str vs & str
//     // str is unsized and by that we mean we do not know the size of str at compile time
//     // only at run time we know the size
//     // by itself str is a view so it should be flexible and compile time cannot allow that
//     // so we use &str instead
//     // everything is an expression in rust so default is return union () which is nothing

//     println!("{}", value);
// }

// fn read_file() {
//     let file = std::fs::read_to_string("lines").unwrap();

//     file
//         .lines()
//         .enumerate()
//         .filter(|(idx, _)| idx % 2 == 0)
//         .skip(2)
//         .take(2)
//         .for_each(|(_, line)| println!("{}", line));
// }

// enum Color {
//     Red,
//     Green,
//     Blue,
//     Yellow,
// }

// impl Color {
//     fn is_green(&self) -> bool {
//         if let Color::Green = self {
//             return true;
//         }
//         return false;
//     }

//     fn is_green_parts(&self) -> bool {
//         match self {
//             Color::Red => return false,
//             Color::Green => return false,
//             Color::Blue => return true,
//             Color::Yellow => return true,
//         }
//     }
// }

// fn print_color(color : Color) {
//     match color {
//         Color::Red => println!("red"),
//         Color::Green => println!("green"),
//         Color::Blue => println!("blue"),
//         Color::Yellow => println!("yellow"),
//     }
// }

// struct Custom {
//     age: usize,
//     name: String,
// }

// enum Item {
//     Number(usize),
//     String(String),
//     Custom(Custom),
// }

// fn append(items: &mut Vec<Item>) {
//     items.push(Item::String("hello".into()));
// }

// fn enum_demo() {
//     let mut items: Vec<Item> = vec![];
//     append(&mut items);

//     let foo = Item::Number(5);

//     match &foo {
//         // & means im not trynna take ownership of foo but just borrowing the value
//         Item::Number(num) => println!("{}", num),
//         Item::String(str) => println!("{}", str),
//         Item::Custom(custom) => println!("name: {}, age: {}", custom.name, custom.age),
//     }

//     match &foo {
//         Item::Custom(custom) if custom.name == "Ricky" => println!("hey rick"),
//         _ => {}
//     }
// }

// enum Option<T> {
//     None,
//     Some(T),
// }

// &str (string slice)
//     |   |   |-- Characteristics:
//     |   |       |
//     |   |       |-- Immutable
//     |   |       |-- Reference to a sequence in another string
//     |   |       |-- Often used for fixed string literals
//     |   |       |-- Doesn't have its own heap-allocated buffer

//  |-- String (type)
//  |-- Characteristics:
//     |
//     |-- Heap-allocated
//     |-- Growable
//     |-- Mutable
//     |-- Owns its content
//     |-- Has a performance cost for its dynamic capabilities

// fn multiply(num: Option<usize>) -> usize {
//     if num.is_some() {
//         return num.unwrap() * 5;
//     }
//     return 0;
// }

// fn multiply(num: Option<usize>) -> usize {
//     if let Some(x) = num {
//         return x * 5;
//     }
//     return 0;
// }

// fn multiply(num: Option<usize>) -> usize {
//     return num.unwrap_or(0) * 5;
// }

// fn multiply(num: Option<usize>) -> Option<usize> {
//     return num.map(|x| x*5)
// }

// fn multiply(num: Option<usize>) -> Option<usize> {
//     let num = num?; // this does the go if err != nil check

//     // let num = match num {
//     //     Some(x) => x, 
//     //     None => return None,
//     // }
    
//     return Some(num * 5);

//     // or 

//     return Some(num? * 5);
// }

// fn practice(num: Vec<usize>, idx : usize) -> usize {
//     // let num = num.get(idx).unwrap();
//     return num.get(idx).unwrap_or(&idx) * 5;
// }

fn main() {
    // iterate();
    // collect();
    // iter_stuff();
    // read_file();
    //
}
