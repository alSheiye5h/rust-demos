// struct Point<T, U> {
//     x: T,
//     y: U
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self,other: Point<V, W>) -> Point<T, W> {
//         Point { x: self.x, y: other.y}
//     }
// }




// fn main() {
//     let p1 = Point { x: 5, y: 10 };
//     let p2 = Point { x: "Hello", y: '中'};

//     let p3 = p1.mixup(p2);

//     assert_eq!(p3.x, 5);
//     assert_eq!(p3.y, '中');

//     println!("Success!");
// }

// Fix the errors to make the code work.
// struct Point<T> {
//     x: T,
//     y: T
// }

// impl Point<f64> {
//     fn distance(&self) -> f64 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

// fn main() {
//     let p: Point<f64> = Point {x: 5f64, y: 10f64};
//     println!("{}", p.distance())
// }

// #[warn(non_camel_case_types)]
// struct Array<T, const N: usize> {
//     data: [T; N], 
// }

// fn main() {
//     const integers: [Array<i32, 3>; 3] = [
//         Array {
//             data: [1, 5, 3],
//         },
//         Array {
//             data: [1, 5, 3],
//         },
//         Array {
//             data: [1, 5, 3],
//         }
//     ];

//     const floats: [Array<f32, 3>; 3] = [
//         Array {
//             data: [2.5, 2.5, 2.5],
//         },
//         Array {
//             data: [2.5, 2.5, 2.5],
//         },
//         Array {
//             data: [2.5, 2.5, 2.5],
//         }
//     ];

//     const strings: [Array<&str, 2>; 2] = [
//         Array {
//             data: ["hello", "hello"],
//         },
//         Array {
//             data: ["hello", "hello"],
//         }
//     ];
// }

// struct Sheep { naked: bool, name: String }

// trait Animal {

//     fn new(name: String) -> Self;

//     fn name(&self) -> String;

//     fn noise(&self) -> String;

//     fn talk(&self) {
//         println!("dwat :{}, w galt :{}", self.name(), self.noise());
//     }
// }

// impl Sheep {
//     fn isNaked(&self) -> bool {
//         self.naked
//     }

//     fn shear(&mut self) {
//         if self.isNaked() {
//             println!("{} is already naked ...", self.name());
//         } else {
//             println!("{} is geeting a haircut", self.name());
//             self.naked = true;
//         }
//     }
// }

// impl Animal for Sheep {
//     fn new(name: String) -> Sheep {
//         Sheep { naked: false, name}
//     }

//     fn name(&self) -> String {
//         self.name.clone()
//     }

//     fn noise(&self) -> String {
//         if self.isNaked() {
//             String::from("baaah?")
//         } else {
//             String::from("boooh!")
//         }
//     }

//     fn talk(&self) {
//         println!("{} pauses briefly... {}", self.name, self.noise());
//     }
// }

// fn main() {
//     let mut shep: Sheep = Sheep::new("tga3".to_owned());

//     let mut dolly: Sheep = Animal::new("Dolly".to_string());

//     dolly.talk();
//     dolly.noise();
//     dolly.shear();

//     shep.talk();
//     shep.noise();
//     shep.shear();

//     println!("success")
    
// }


// make a struct sheep
// make a trait animal with function : new name noise talk
// impl to sheep : isnaked, shear: is already naked getting a haircut
// impl animal for sheep


// struct Sheep {
//     naked: bool,
//     name: String
// }

// trait Animal {
//     fn new(name: String) -> Self ;

//     fn name(&self) -> String ;

//     fn noise(&self) -> String ;

//     fn talk(&self) {
//         println!("dwa: {}, w 9al: {}", self.name(), self.noise());
//     }
// }
// impl Sheep {
//     fn isNaked(&self) -> bool {
//         self.naked
//     }

//     fn shear(&mut self) {
//         if self.isNaked() {
//             println!("{} is already naked ...", self.name());
//         } else {
//             println!("{} is getting a haircut ...", self.name());
//             self.naked = true;
//         }
//     }
// }

// impl Animal for Sheep {
//     fn new(name: String) -> Sheep {
//         Sheep { naked: false, name}
//     }

//     fn name(&self) -> String {
//         self.name.clone()
//     }

//     fn noise(&self) -> String {
//         if self.isNaked() {
//             "baaaah!".to_owned()
//         } else {
//             "booooh?".to_owned()
//         }
//     }

//     fn talk(&self) {
//         println!("galk {}, {}", self.name(), self.noise())
//     }
// }

// fn main() {
//     let mut dolly: Sheep = Animal::new("Dolly".to_string());

//     let mut otak: Sheep = Sheep::new("Otak".to_owned());

//     dolly.talk();
//     dolly.shear();
//     dolly.noise();

//     otak.talk();
//     otak.shear();
//     otak.noise();
// }

// use std::ops;
// struct Foo;
// struct Bar;

// #[derive(PartialEq, Debug)]
// struct FooBar;
// #[derive(PartialEq, Debug)]
// struct BarFoo;

// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;
    
//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }

// impl ops::Sub<Foo> for Bar {
//     type Output = BarFoo;
    
//     fn sub(self, _rhs: Foo) -> BarFoo {
//         BarFoo
//     }
// }

// fn main() {
    
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Bar - Foo, BarFoo);

//     println!("seccess")
// }

// use std::ops;

// struct Foo;
// struct Bar;

// #[derive(PartialEq, Debug)]
// struct FooBar;

// #[derive(PartialEq, Debug)]
// struct BarFoo;

// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;

//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }

// impl ops::Sub<Foo> for Bar {
//     type Output = BarFoo;

//     fn sub(self, _rhs: Foo) -> BarFoo {
//         BarFoo
//     }
// }


// fn main() {

//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);

//     println!("ok")
// }

// trait Summary {
//     fn summarize(&self) -> String;
// }

// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String, 
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }

// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }

// fn main() {
//     let post: Post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };
//     let weibo: Weibo = Weibo {
//         username: "sunface".to_string(),
//         content: "Weibo seems to be worse than Tweet".to_string(),
//     };

//     summary(&post);
//     summary(&weibo);

//     println!("{:?}", post);
//     println!("{:?}", weibo);

// }
// // #[derive(Debug)]
// fn summary(x: &impl Summary) {
//     let output: String = x.summarize();
//     println!("{}", output)
// }



// struct Sheep {}
// struct Cow {}

// trait Animal {
//     fn noise(&self) -> String;
// }

// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }

// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }

// // Returns some struct that implements Animal, but we don't know which one at compile time.
// // FIX the errors here, you can make a fake random, or you can use trait object.
// fn random_animal<T, U>(random_number: f64) -> T | U {
//     if random_number < 0.5 {
//         Sheep {}
//     } else {
//         Cow {}
//     }
// }

// fn main() {
//     let random_number = 0.234;
//     let animal: T = random_animal(random_number);
//     println!("You've randomly chosen an animal, and it says {}", animal.noise());
// }



// trait Animal {
//     fn noise(&self) -> String;
// }

// struct Cow {}
// struct Sheep {}

// impl Animal for Cow {
//     fn noise(&self) -> String {
//         format!("moooooo!")
//     }
// }

// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         format!("baaaah!")
//     }
// }

// fn from_random(num: f64) -> Box<dyn Animal> {
//     if num < 3.3 {
//         Box::new(Sheep {})
//     } else {
//         Box::new(Cow {})
//     }
// }

// fn main() {
//     let rand: f64 = 3.1;
//     let animal = from_random(rand);

//     println!("{}", animal.noise())
// }


// trait Animal {
//     fn noise(&self) -> String;
// }

// struct Cow {}
// struct Sheep {}

// impl Animal for Cow {
//     fn noise(&self) -> String {
//         format!("moooooh")
//     }
// }

// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         format!("baaaaah")
//     }
// }

// fn from_random(n: i32) -> dyn Animal {
//     if n < 3 {
//         &Sheep {}
//     } else {
//         &Cow {}
//     }
// }


// fn main() {
//     let rand: i32 = 2;
//     let animal: &dyn Animal = from_random(rand);

//     println!("{}", animal.noise())

// }

// trait Animal {
//     fn noise(&self) -> String;
// }

// struct Cow {}
// struct Sheep {}

// impl Animal for Cow {
//     fn noise(&self) -> String {
//         format!("mooooh!")
//     }
// }

// impl Animal for Sheep {
//     fn noise(&self) -> String  {
//         format!("baaaah")
//     }
// }

// fn from_random(n: i32) -> Box<dyn Animal> { // dynamic animal: type know , size not .
//     if n < 3 {
//         Box::new(Sheep {})
//     } else {
//         Box::new(Cow {})
//     }
// }

// fn main() {
//     let rand: i32 = 3;
//     let animal: Box<dyn Animal> = from_random(rand);

//     println!("{}", animal.noise())
// }



// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y}
//     }
// }

// impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }

// #[derive(Debug, PartialOrd, PartialEq)]
// struct Unit(i32);

// fn main() {
//     let pair: Pair<Unit> = Pair::new(Unit(1), Unit(2));

//     pair.cmp_display()
// }




// trait Bird {
//     fn quack(&self) -> String;
// }

// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) -> String {
//         "duck duck".to_string()
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) -> String {
//         "swan swan".to_string()
//     }
// }

// fn main() {
//     // FILL in the blank.
//     let duck: Duck = Duck {};
//     duck.swim();

//     let bird = hatch_a_bird(2);
//     // This bird has forgotten how to swim, so below line will cause an error.
//     // bird.swim();
//     // But it can quak.
//     if bird.quack() != "duck duck" {
//         println!("duck er")
//     };

//     let bird = hatch_a_bird(1);
//     // This bird has forgotten how to fly, so below line will cause an error.
//     // bird.fly();
//     // But it can quak too.
//     if bird.quack() != "swan swan" {
//         panic!()
//     }

//     println!("Success!");
// }   

// // IMPLEMENT this function.
// fn hatch_a_bird(x: i32) -> Box<dyn Bird> {
//     match x {
//         1 => Box::new(Swan{}),
//         2 => Box::new(Duck{}),
//         _ => panic!()
//     }
// }



// trait Bird {
//     fn quack(&self);
// }

// struct Duck;
// impl Duck {
//     fn fly(&self) {
//         println!("Look, the duck is flying")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) {
//         println!("{}", "duck duck");
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) {
//         println!("{}", "swan swan");
//     }
// }

// fn main() {
//     // FILL in the blank to make the code work.
//     let birds: [&dyn Bird; 2] = [&Duck, &Swan];

//     for bird in birds {
//         bird.quack();
//         // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
//         // So, the code below will cause an error.
//         // bird.fly();
//     }
// }

// trait Bird {
//     fn quack(&self);
// }

// struct Duck;
// impl Duck {
//     fn node(&self) {
//         println!("duck")
//     }
// }

// struct Swan;
// impl Swan {
//     fn node(&self) {
//         println!("swan")
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) {
//         println!("swan is quacking")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) {
//         println!("duck is quacking")
//     }
// }

// fn main() {
//     let birds: [&dyn Bird; 2] = [&Swan, &Duck];

//     for bird in birds {
//         bird.quack()
//     }
// }






// impl Bird for Duck {
//     fn quack(&self) -> String {
//         "duck duck".to_string()
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) -> String {
//         "swan swan".to_string()
//     }
// }

// fn main() {

//     let duck: Duck = Duck {};
//     duck.swim();

//     let bird: Box<dyn Bird> = hatch_a_bird(2);
//     assert_eq!(bird.quack(), String::from("duck duck"));

//     let bird: Box<dyn Bird> = hatch_a_bird(1);
//     assert_eq!(bird.quack(), String::from("swan swan"));

//     println!("success")
// }

// fn hatch_a_bird(n: i32) -> Box<dyn Bird> {
//     match n {
//         1 => Box::new(Swan {}),
//         2 => Box::new(Duck {}),
//         _ => panic!()
//     }
// }   


// trait Draw {
//     fn draw(&self) -> String;
// }

// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }

// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64: {}", *self)
//     }
// }

// fn main() {
//     let x: f64 = 1.1f64;
//     let y: u8 = 8u8;

//     println!("{}", draw_with_box(Box::new(x)));

//    println!("{}", draw_with_ref(&y));

//     println!("Success");
// }

// fn draw_with_box(x: Box<dyn Draw>) -> String {
//     x.draw()
// }

// fn draw_with_ref(x: &dyn Draw) -> String {
//     x.draw()
// }


// trait Foo {
//     fn method(&self) -> String;
// }

// impl Foo for u8 {
//     fn method(&self) -> String {
//         format!("u8: {}", *self)
//     }
// }

// impl Foo for String {
//     fn method(&self) -> String {
//         format!("String: {}", *self)
//     }
// }

// fn static_dispatch<T: Foo>(x: T) {
//     x.method();
// }

// fn dynamic_dispatch(x: &dyn Foo) {
//     x.method();
// }

// fn main() {
//     let x = 5u8;
//     let y = "Hello".to_string();

//     static_dispatch(x);
//     dynamic_dispatch(&y);

//     println!("Success");
// }

// trait MyTrait {
//     fn f(&self) -> Self;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Self { 42 }
// }

// impl MyTrait for String {
//     fn f(&self) -> Self { self.clone() }
// }

// fn my_function(x: Box<dyn MyTrait>)  {
//     x.f()
// }

// fn main() {
//     my_function(Box::new(13_u32));
//     my_function(Box::new(String::from("abc")));

//     println!("Success!");
// }

// trait MyTrait {
//     fn f(&self) -> Self;
// }

// impl MyTrait for u32 {
//     fn f(&self) -> Self {
//         42
//     }
// }

// impl MyTrait for String {
//     fn f(&self) -> Self {
//         self.clone()
//     }
// }

// fn my_function<T: MyTrait>(x: T) -> T {
//     x.f()
// }


// fn main() {
//     my_function(13_u32);
//     my_function(String::from("abc"));

//     println!("Success");
// }

// fn main() {
//     // let mut s: String = String::from("hello, ");
//     let mut s: String = "hello, ".to_owned();
//     s.push_str("world");
//     s.push('!');

//     move_ownership(s.clone());

//     assert_eq!(s, "hello, world!");

//     println!("Success");
// }

// fn move_ownership(s: String) {
//     println!("ownership of {} is moved here!", s);
// }

// fn main() {
//     let mut s: String = String::from("hello, world");

//     let slice1: &str = &s;
//     assert_eq!(slice1, "hello, world");
    
//     let slice2: &str = &s[..5];
//     assert_eq!(slice2, "hello");

//     let slice3: &mut String = &mut s;
//     slice3.push('!');
//     assert_eq!(slice3, "hello, world!");

//     println!("Success");
// }

// fn main() {

//     let s: String = String::from("hello, world!");

//     let slice: &str = &s;

//     let s: String = slice.to_string();

//     assert_eq!(s, "hello, world!");

//     println!("success")
// }

// fn main() {
//     let s: String = String::from("hello, 世界");
//     let slice1 = &s[0..1];
//     assert_eq!(slice1, "h");

//     let slice2 = &s[7..13];
//     assert_eq!(slice2, "世界");

//     for (i, c) in s.chars().enumerate() {
//         if i == 7 {
//             assert_eq!(c, '世')
//         }
//     }

//     println!("success");
// }


// fn main() {
//     let mut s: String = String::new();
//     s.push_str("hello");

//     let v: Vec<u8> = vec![104, 101, 108, 108, 111];

//     let s1: String = String::from_utf8(v).unwrap();

//     assert_eq!(s, s1);

//     println!("Success");
// }

// fn main() {
//     let mut s = String::with_capacity(25);

//     println!("{}", s.capacity());

//     for _ in 0..2 {
//         s.push_str("hellohellohello");
//         println!("{}", s.capacity());
//     }

//     println!("Success");
// }

// use std::mem;

// fn main() {
//     let story = String::from("rust");

//     let mut story = mem::ManuallyDrop::new(story);

//     let ptr = &story;
//     let len = story.len();
//     let capacity = story.capacity();

//     assert_eq!(16, len);

//     let s: unsafe { String::from_raw_parts(ptr, len, capacity)};


// }

// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];

//     let v: Vec<u8> = Vec::from(arr);
//     is_vec(&v);

//     let v: Vec<u8> = vec!(1, 2, 3);
//     is_vec(&v);

//     let v: Vec<u8> = vec![1, 2, 3];
//     is_vec(&v);


//     let v1: Vec<[u8; 3]> = vec!(arr);
//     let mut v1: Vec<u8> = vec!();

//     for i in &arr {
//         v1.push(*i);
//     }
//     use std::fmt::Display;
//     println!("{:#?}", v1);


//     assert_eq!(v, v1);

//     println!("Success");
// }
// fn is_vec(v: &Vec<u8>) {}



// fn main() {
//     let arr: [u8; 3] = [1, 2, 3];

//     let v = Vec::from(arr);
//     is_vec(&v);

//     let v: Vec<u8> = vec![1, 2, 3];
//     is_vec(&v);

//     let v: Vec<u8> = vec!(1, 2, 3);
//     is_vec(&v);

//     let mut v1: Vec<u8> = Vec::new();

//     for i in arr {
//         v1.push(i);
//     }

//     assert_eq!(v, v1);

//     println!("Success");
// }

// fn is_vec(v: &Vec<u8>) {}

// fn main() {
//     let mut v1 = Vec::from([1, 2, 3]);
//     v1.pop();
//     v1.push(3);

//     let mut v2 = Vec::new();
//     v2.extend(v1.clone());

//     assert_eq!(v1, v2);
//     println!("Success");
// }

// fn main() {
//     let arr = [1, 2, 3];
//     let v1 = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();

//     assert_eq!(v1, v2);

//     let s: String = "hello".to_string();
//     let v1: Vec<u8> = s.into();

//     let s: String = "hello".to_string();
//     let v2: Vec<u8> = s.into_bytes();
//     assert_eq!(v1, v2);

//     let s: &str = "hello";
//     let v3: Vec<u8> = Vec::from(s);
//     assert_eq!(v2, v3);

//     let v4: Vec<i32> = [0; 10].iter().copied().collect();
//     assert_eq!(v4, vec![0; 10]);

//     println!("Success"); 
// }


// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     let v1: Vec<i32> = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();
//     assert_eq!(v1, v2);

//     let s: String = "hello".to_string();
//     let v1: Vec<u8> = s.into();

//     let s: String = "hello".to_string();
//     let v2: Vec<u8> = s.into();
//     assert_eq!(v1, v2);

//     let s: &str = "hello";
//     let v3 = Vec::from(s);
//     assert_eq!(v2, v3);

//     let v4: Vec<i32> = [0; 10].iter().copied().collect();
//     assert_eq!(v4, vec![0; 10]);

//     println!("Success");
// }


// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     let v1: Vec<i32> = Vec::from(arr);
//     let v2: Vec<i32> = arr.into();

//     assert_eq!(v1, v2);

//     let s: String = "hello".to_string();
//     let v1: Vec<u8> = s.into();

//     let s: String = "hello".to_string();
//     let v2: Vec<u8> = s.into_bytes();
//     assert_eq!(v1, v2);

//     let s: &str = "hello";
//     let v3: Vec<u8> = Vec::from(s);
//     assert_eq!(v2, v3);

//     let v4: Vec<i32> = [0; 10].iter().copied().collect();
//     assert_eq!(v4, vec![0; 10]);

//     println!("Success");
// }


// fn main() {
//     let mut v: Vec<i32> = Vec::from([1, 2, 3]);
//     for i in 0..5 {
//         println!("{:?}", v.get(i))
//     }

//     for i in 0..5 {
//         match v.get(i) {
//             Some(e) => e++,
//             None => i + 2
//         }
//     }

//     assert_eq!(v, vec![2, 3, 4, 5, 6]);

//     println!("Success");
// }


// fn main() {
//     let mut v: Vec<i32> = Vec::from([1, 2, 3]);

//     for i in 0..5 {
//         println!("{:?}", v.get(i))
//     }

//     for i in 0..5 {
//         match v.get(i) {
//             Some(e) => v[i] = e + 1,
//             None => v.push(i as i32 + 2),
//         }
//     }


//     assert_eq!(v, vec![2, 3, 4, 5, 6]);

//     println!("Success!");
// }

// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];

//     let slice1 = &v[..v.len()];

//     let slice2 = &v[0..4];

//     assert_eq!(slice1, slice2);

//     let vec_ref: &mut Vec<i32> = &mut v;
//     (*vec_ref).push(4);
//     let mut slice3 = &mut v[0..3];
//     slice3.push(4); //error

//     assert_eq!(slice3, &[1, 2, 3, 4]);

//     println!("Success");
// }


// fn main() {
//     let mut v: Vec<i32> = vec![1, 2, 3];

//     let slice1: &[i32] = &v[..];

//     let slice2: &[i32] = &v[0..v.len()];

//     assert_eq!(slice1, slice2);

//     let vec_ref: &mut Vec<i32> = &mut v;
//     (*vec_ref).push(4);
//     let slice3: &mut [i32] = &mut v[0..4];

//     assert_eq!(slice3, &[1, 2, 3, 4]);

//     println!("Success!");
// }

/*
    - the capacity of a vector is the amount of space allocated for any future element
    - while the length is the amount of element that are actually in the vector
*/

// fn main() {
//     let mut vec: Vec<u8> = Vec::with_capacity(10);

//     assert_eq!(vec.len(), 0);
//     assert_eq!(vec.capacity(), 10);

//     for i in 0..10 {
//         vec.push(i);
//     }

//     assert_eq!(vec.len(), 10);
//     assert_eq!(vec.capacity(), 10);

//     vec.push(11);
//     assert_eq!(vec.len(), 11);
//     assert_eq!(vec.capacity(), 20);


//     let mut vec: Vec<i32> = Vec::with_capacity(0);

//     for i in 0..100 {
//         vec.push(i);
//     }

//     assert_eq!(vec.len(), 100);
//     assert_eq!(vec.capacity(), 128);

//     println!("Success");
// }

// fn main() {
//     let mut vec: Vec<i32> = Vec::with_capacity(10);

//     assert_eq!(vec.len(), 0);
//     assert_eq!(vec.capacity(), 10);

//     for i in 0..10 {
//         vec.push(i);
//     }

//     assert_eq!(vec.len(), 10);
//     assert_eq!(vec.capacity(), 10);

//     vec.push(11);
//     assert_eq!(vec.len(), 11);
//     assert!(vec.capacity() >= 11);

//     let mut vec: Vec<i32> = Vec::with_capacity(100);
//     for i in 0..100 {
//         vec.push(i);
//     }

//     assert_eq!(vec.len(), 100);
//     assert_eq!(vec.capacity(), 100);

//     println!("Success");
// }

// #[derive(Debug, PartialEq)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }

// fn main() {
//     let v: Vec<IpAddr> = vec![
//         IpAddr::V4("192.168.1.1".to_string()),
//         IpAddr::V6("::1".to_string()),
//     ];

//     assert_eq!(v[0], IpAddr::V4(String::from("192.168.1.1")));
//     assert_eq!(v[1], IpAddr::V6(String::from("::1")));

//     println!("Success");
// }

// trait IpAddr {
//     fn display(&self);
// }

// struct V4(String);
// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4 : {:?}", self.0);
//     }
// }

// struct V6(String);
// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6 : {:?}", self.0);
//     }
// }

// fn main() {
//     let v: Vec<Box<dyn IpAddr>> = vec![
//         Box::new(V4(String::from("127.0.0.1"))),
//         Box::new(V6(String::from("::1"))),
//     ];

//     for ip in v {
//         ip.display();
//     }
// }

// trait IpAddr {
//     fn display(&self);
// }

// struct V4(String);
// impl IpAddr for V4 {
//     fn display(&self) {
//         println!("ipv4 : {:?}", self.0)
//     }
// }

// struct V6(String);
// impl IpAddr for V6 {
//     fn display(&self) {
//         println!("ipv6 : {:?}", self.0)
//     }
// }

// fn main() {
//     let v: Vec<Box<dyn IpAddr>> = vec![
//         Box::new(V4("192.168.1.1".to_string())),
//         Box::new(V6(String::from("::1"))),
//     ];

//     for ip in v {
//         ip.display();
//     }
// }

// use std::collections::HashMap;

// fn main() {
//     let mut scores: HashMap<&str, i32> = HashMap::new();

//     scores.insert("Sunface", 98);
//     scores.insert("Daniel", 95);
//     scores.insert("Ashley", 69);
//     scores.insert("Katie", 58);

//     let score: Option<&i32> = scores.get("Sunface");

//     assert_eq!(score, Some(&98));

//     if scores.contains_key("Daniel") {
        
//         let score: i32 = scores["Daniel"];
//         assert_eq!(score, 95);
//         scores.remove("Daniel");
//     }

//     assert_eq!(scores.len(), 3);

//     for (name, score) in scores {
//         println!("the score of {} is {}", name, score);
//     }
// }

// use std::collections::HashMap;

// fn main() {
//     let teams: [(&str, i32);3] = [
//         ("Chinese Team", 100),
//         ("American Team", 10),
//         ("France Team", 50),
//     ];

//     let mut teams_map1: HashMap<&str, i32> = HashMap::new();
//     for team in &teams {
//         teams_map1.insert(team.0, team.1);
//     }

//     // let teams_map2: HashMap<&str, i32> = HashMap::from(teams);
//     let teams_map2: HashMap<&str, i32> = teams.iter().copied().collect();

//     assert_eq!(teams_map1, teams_map2);

//     println!("Success");
// }


// use std::collections::HashMap;

// fn main() {
    
//     let mut player_stats: HashMap<&str, u8> = HashMap::new();

//     player_stats.entry("health").or_insert(100);

//     assert_eq!(player_stats["health"], 100);

//     player_stats.entry("health").or_insert_with(random);
//     assert_eq!(player_stats["health"], 100);

//     let health: &mut u8 = player_stats.entry("health").or_insert(50);
//     assert_eq!(health, &mut 100);
//     *health -= 50;
//     assert_eq!(*health, 50);

//     println!("Success");
// }

// fn random() -> u8 {
//     42
// }

// use std::collections::HashMap;

// #[derive(Debug, Hash, Eq, PartialEq)]
// struct Viking {
//     name: String,
//     country: String,
// }

// impl Viking {
//     fn new(name: &str, country: &str) -> Viking {
//         Viking {
//             name: name.to_string(),
//             country: country.to_string(),
//         }
//     }
// }

// fn main() {
//     let vikings : [(Viking, i32);3] = [
//         (Viking::new("Einar", "Norway"), 25),
//         (Viking::new("Olaf", "Denmark"), 24),
//         (Viking::new("Harald", "Iceland"), 12),
//     ];

//     for (viking, hp) in vikings {
//         println!("{:?} from {:?} has {}", viking.name, viking.country, hp);
//     }
// }

// use std::collections::HashMap;

// fn main() {
//     let mut map: HashMap<i32, i32> = HashMap::with_capacity(100);
//     map.insert(1, 2);
//     map.insert(3, 4);


//     assert!(map.capacity() >= 100);

//     map.shrink_to(50);
//     assert!(map.capacity() >= 50);

//     map.shrink_to_fit();
//     assert!(map.capacity() >= 2);
//     println!("Success");
// }

// use std::collections::HashMap;

// fn main() {
//     let v1 = 10;
//     let mut m1 = HashMap::new();
//     m1.insert(v1, v1);
//     println!("v1 is still usable after inserting to hashmap : {}", v1);

//     let v2 = "hello".to_string();
//     let mut m2 = HashMap::new();
//     m2.insert(v2.clone(), "hello");

//     assert_eq!(v2, "hello");

//     println!("Success");
// }

// fn main() {
//     let decimal: f32 = 97.123_f32;

//     let integers: u8 = decimal as u8;

//     let c1: char = decimal as u8 as char;
//     let c2: char = integers as char;

//     assert_eq!(integers + 1, 'b' as u8);

//     println!("Success");
// }

// #[allow(overflowing_literals)]
// fn main() {
//     assert_eq!(u8::MAX, 255);

//     let v: u8 = 1000 as u8;

//     println!("{}", v);

//     println!("Success");
// }

// #[allow(overflowing_literals)]
// fn main() {
//     assert_eq!(1000 as u16, 1000);

//     assert_eq!(1000 as u8, (1000 % 256) as u8);

//     println!("1000 mod 256 is : {}", (1000i32 % 256i32) as u8);

//     assert_eq!(-1_i8 as u8, 255);

//     assert_eq!(300.1_f32 as u8,u8::MAX);
//     assert_eq!(-100.1_f32 as u8, u8::MIN);

//     unsafe {
//         println!("300.0 is {}", 300.0_f32.to_int_unchecked::<u8>());
//         println!("-100.0 as u8 is {}", (-100.0_f32).to_int_unchecked::<u8>());
//         println!("nan as u8 is {}", f32::NAN.to_int_unchecked::<u8>());
//     }
// }

// fn main() {
//     let i1: i32 = false.into();
//     let i2: i32 = i32::from(false);
//     assert_eq!(i1, i2);
//     assert_eq!(i1, 0);

//     let i3: u32 = 'a'.into();

//     let s: String = String::from('a');

//     println!("{}", i3);

//     println!("Success");
// }

// #[derive(Debug)]
// struct Number {
//     value: i32,
// }

// impl From<i32> for Number {
//     fn from(a: i32) -> Number {
//         Number {
//             value: a,
//         }
//     }
// }

// fn main() {
//     let num: Number = Number::from(30);
//     assert_eq!(num.value, 30);

//     let num: Number = 30.into();
//     assert_eq!(num.value, 30);

//     println!("Success");
// }

// use std::fs;
// use std::io;
// use std::num;

// enum CliError {
//     IoError(io::Error),
//     ParseError(num::ParseIntError),
// }

// impl From<io::Error> for CliError {
//     fn from(er: io::Error) -> Self {
//         CliError::IoError(er)
//     }
// }

// impl From<num::ParseIntError> for CliError {
//     fn from(er: num::ParseIntError) -> Self {
//         CliError::ParseError(er)
//     }
// }

// fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
//     // ? automatically converts io::Error to CliError
//     let contents = fs::read_to_string(&file_name)?;
//     // num::ParseIntError -> CliError
//     let num: i32 = contents.trim().parse()?;
//     Ok(num)
// }

// fn main() {
//     println!("Success!");
// }

// use std::fs;
// use std::io;
// use std::num;

// enum CliError {
//     IoError(io::Error),
//     ParseError(num::ParseIntError),
// }

// impl From<io::Error> for CliError {
//     fn from(e: io::Error) -> Self {
//         CliError::IoError(e)
//     }
// }

// impl From<num::ParseIntError> for CliError {
//     fn from(e: num::ParseIntError) -> Self {
//         CliError::ParseError(e)
//     }
// }

// fn open_and_parse_file(file_name: &str) -> Result<i32, CliError> {
//     // ? automatically converts io::Error to CliError
//     let contents = fs::read_to_string(&file_name)?;
//     // num::ParseIntError -> CliError
//     let num: i32 = contents.trim().parse()?;
//     Ok(num)
// }

// fn main() {
//     println!("Success!");
// }

// fn main() {
//     let n: i16 = 255;

//     let n: u8 = match n.try_into() {
//         Ok(n) => n,
//         Err(e) => {
//             println!("error : {:?}", e.to_string());
//             0
//         }
//     };

//     assert_eq!(n, 255);

//     println!("Success");
// }

// use std::convert::TryFrom;
// use std::convert::TryInto;

// #[derive(Debug, PartialEq)]
// struct EvenNum(i32);

// impl TryFrom<i32> for EvenNum {
//     type Error = ();

//     fn try_from(value: i32) -> Result<Self, Self::Error> {
//         if value % 2 == 0 {
//             Ok(EvenNum(value))
//         } else {
//             Err(())
//         }
//     }
    
// }

// fn main() {
//     assert_eq!(EvenNum::try_from(8), Ok(EvenNum(8)));
//     assert_eq!(EvenNum::try_from(5), Err(()));

//     // FILL in the blanks
//     let result: Result<EvenNum, ()> = 8i32.try_into();
//     assert_eq!(result, Ok(EvenNum(8)));
//     let result: Result<EvenNum, ()> = 5i32.try_into();
//     assert_eq!(result, Err(()));

//     println!("Success!");
// }

// use std::fmt;

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "The point is ({}, {})", self.x, self.y)
//     }
// }

// fn main() {
//     let origin: Point = Point { x: 0, y: 0 };
//     // FILL in the blanks
//     assert_eq!(origin.to_string(), "The point is (0, 0)");
//     assert_eq!(format!("{}", origin), "The point is (0, 0)");

//     println!("Success!");
// }

// use std::fmt;

// struct Point {
//     x: i32,
//     y: i32,
// }

// impl fmt::Display for Point {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "The point is ({}, {})", self.x, self.y)
//     }
// }

// fn main() {
//     let origin: Point = Point { x: 0, y: 0 };
//     // FILL in the blanks
//     assert_eq!(origin.to_string(), "The point is (0, 0)");
//     assert_eq!(format!("{}", origin), "The point is (0, 0)");

//     println!("Success!");
// }

// use std::str::FromStr;
// fn main() {
//     let parsed: i32 = "5".parse().unwrap();
//     let turbo_parsed = "10".parse::<i32>().unwrap();
//     let from_str = i32::from_str("20").unwrap();
//     let sum: i32 = parsed + turbo_parsed + from_str;
//     assert_eq!(sum, 35);

//     println!("Success");
// }

// use std::str::FromStr;
// use std::num::ParseIntError;

// #[derive(Debug, PartialEq)]
// struct Point {
//     x: i32,
//     y: i32,
// }

// impl FromStr for Point {
//     type Err = ParseIntError;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let coords: Vec<&str> = s.trim_matches()
//     }
// }

// fn drink(beverage: &str) {
//     if beverage == "lemonade" {
//         println!("Success");
//         panic!();
//     }
//     println!("Exercise Failed if printing out this line!");
// }

// fn main() {
//     drink("lemonade");

//     println!("Exercise Failed if printing out this line!");
// }


// fn main() {
//     assert_eq!("abc".as_bytes(), [ 97, 98, 99]);

//     let v: Vec<i32> =  vec![1, 2, 3];
//     let ele: i32 = v[2];

//     let ele: i32 = *v.get(2).unwrap();

//     let v: f64 = production_rate_per_hour(2u8);

//     divide(15, 0);

//     println!("Success");
// }

// fn divide(x: u8, y: u8) {
//     println!("{}", x / y);
// }

// fn production_rate_per_hour(speed: u8) -> f64 {
//     let cph: u8 = 221;
//     match speed {
//         1..=4 => (speed * cph) as f64,
//         5..=8 => (speed * cph) as f64 * 0.9,
//         9..=10 => (speed * cph) as f64 * 0.77,
//         _ => 0 as f64,
//     }
// }

// use std::num::ParseIntError;
// fn main() -> Result<(), ParseIntError> {
//     let number_str: &str = "10";
//     let number: i32 = number_str.parse::<i32>()?;
//     println!("{}", number);
//     Ok(())
// }

// use std::num::ParseIntError;

// fn multiply(n1_str: &str, n2_str: &str) ->  Result<i32, ParseIntError> {
//     let n1: Result<i32, ParseIntError> = n1_str.parse::<i32>();
//     let n2: Result<i32, ParseIntError> = n2_str.parse::<i32>();

//     Ok(n1.unwrap() * n2.unwrap())
// } 

// fn main() {
//     let result: Result<i32,  ParseIntError> = multiply("10", "20");
//     assert_eq!(result.unwrap(), 200);

//     println!("Success");
// }


// use std::num::ParseIntError;

// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     let n1: i32 = n1_str.parse::<i32>()?;
//     let n2: i32 = match n2_str.parse::<i32>() {
//         Ok(n) => n,
//         Err(e) => {
//             println!("Hello Error : \n");
//             return Err(e);
//         }
//     };

//     Ok(n1 * n2)    
// }

// fn main() {
//     assert_eq!(multiply("10", "a").unwrap(), 20);
//     println!("Success");
// }


// use std::fs::File;
// use std::io::{self, Read};

// fn read_file1() -> Result<String, io::Error> {
//     let f: Result<File, io::Error> = File::open("hello.txt");
//     let mut f: File = match f {
//         Ok(file) => file,
//         Err(e) => return Err(e),
//     };

//     let mut s: String = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }

// fn read_file2() -> Result<String, io::Error> {
//     let mut s: String = String::new();

//     File::open("hello.txt")?.read_to_string(&mut s)?;

//     Ok(s)
// }

// fn main() {
//     assert_eq!(read_file1().unwrap_err().to_string(), read_file2().unwrap().to_string());
//     println!("Success");
// }

// use std::num::ParseIntError;

// fn add_two_map(n_str: &str) -> Result<i32, ParseIntError> {
//     n_str.parse::<i32>().map(|x| x + 2)
// }

// fn add_two_then(n_str: &str) -> Result<i32, ParseIntError> {
//     n_str.parse::<i32>().and_then(|x| Ok(x + 2))
// }

// fn main() {
//     assert_eq!(add_two_map("4").unwrap(), 6);
//     assert_eq!(add_two_then("5").unwrap(), 7);

//     println!("Success");
// }

// use std::num::ParseIntError;

// fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     match n1_str.parse::<i32>() {
//         Ok(n1) => match n2_str.parse::<i32>() {
//             Ok(n2) => Ok(n1 * n2),
//             Err(e) => Err(e),
//         },
//         Err(ee) => Err(ee),
//     }    
// } 

// fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
//     n1_str.parse::<i32>().and_then(|x| n2_str.parse::<i32>().map(|y| y * x))
// }

// fn print(arg: Result<i32, ParseIntError>) {
//     match arg {
//         Ok(n) => println!("n is {}", n),
//         Err(e) => println!("Error is : {}", e),
//     }
// }

// fn main() {
//     let twenty: Result<i32, ParseIntError> = multiply1("20", "2");
//     print(twenty);

//     let tt: Result<i32, ParseIntError> = multiply("t", "2");
//     print(tt);

//     println!("Success");
// }

// use std::num::ParseIntError;

// fn main() -> Result<(), ParseIntError> {
//     let number_str: &str = "1t";
//     let number: i32 = match number_str.parse::<i32>() {
//         Ok(n) => n,
//         Err(e) => return Err(e),
//     };
//     println!("{}", number);
//     Ok(())
// }

// use std::fmt;
// struct Structure(i32);

// struct Deep(Structure);
// impl fmt::Debug for Deep {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.0.0)
//     }
// }

// fn main() {
//     println!("Now {:?} will print", Deep(Structure(7)));
// }

// use std::fmt;

// struct Point2D {
//     x: f64,
//     y: f64,
// }

// impl fmt::Display for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Display: {} + {}i", self.x, self.y)
//     }
// }

// impl fmt::Debug for Point2D {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Debug: Complex {{ real: {}, imag: {} }}", self.x, self.y)
//     }
// }

// fn main() {
//     let point = Point2D {x: 3.3, y: 7.2};
//     assert_eq!(format!("{}",point), "Display: 3.3 + 7.2i");
//     assert_eq!(format!("{:?}",point), "Debug: Complex { real: 3.3, imag: 7.2 }");
    
//     println!("Success!");
// }

// use std::fmt;

// struct List(Vec<i32>);

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
//         let vec = &self.0;
        
//         write!(f, "[")?;

//         for (count, v) in vec.iter().enumerate() {

//             if count != 0 { write!(f, ", ");};
//             write!(f, "{}: {}", count, v);
//         }
//         write!(f, "]")
//     }
// }

// fn main() {
//     let v: List = List(vec!(1, 2, 3));
//     assert_eq!(format!("{}",v), "[0: 1, 1: 2, 2: 3]");
//     println!("Success!");
// }

// fn main() {
//     let i: i32 = 3;
//     {
//         let borrow1: &i32 = &i;
//         println!("borrow1 : {}", borrow1);
//     }
//     {
//         let borrow2: &i32 = &i;
//         println!("borrow2 : {}", borrow2);
//     }
// }

// fn print_one<'a>(x: &'a i32) {
//     println!("`print_one`: x is {}", x);
// }

// fn add_one<'a>(x: &'a mut i32) {
//     *x += 1;
// }

// fn print_multi<'a, 'b>(x: &'a i32, y:&'b i32) {
//     println!("`print_multi`: x is {}, y is {}", x, y);
// }

// fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
//     x
// }

// fn main() {
//     let x = 7;
//     let y = 9;

//     print_one(&x);
//     print_multi(&x, &y);

//     let z = pass_x(&x, &y);
//     print_one(z);

//     let mut t = 3;
//     add_one(&mut t);
//     print_one(&t);
// }

// fn longest(x: &str, y: &str) -> String {
//     if x.len() > y.len() {
//         String::from(x)
//     } else {
//         String::from(y)
//     }
// }

// fn main() {
//     let x = "hello";
//     let y = "helloo";

//     println!("longest is : {}", longest(x, y));
// }

// fn invalid_output() -> &'static str {
//     "foo"
// }


// fn main() {
//     let s: &str = "hello";

//     println!("the str is : {}", invalid_output());
// }

// fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("x is {} and y is {}", x, y);
// }

// fn failed_borrow<'a>() {
//     let _x = 12;
//     let y: &'a i32 = &_x;
// }

// fn print_one<'a>(x: &'a i32) {
//     println!("`print_one`: x is {}", x);
// }

// fn add_one<'a>(x: &'a mut i32) {
//     *x += 1;
// }

// fn print_multi<'a, 'b>(x: &'a i32, y:&'b i32) {
//     println!("`print_multi` : is {}, y is {}", x, y);
// }

// fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
//     x
// }

// fn main() {
//     let x: i32 = 7;
//     let y: i32 = 9;

//     print_one(&x);
//     print_multi(&x, &y);

//     let z: &i32 = pass_x(&x, &y);
//     print_one(&z);
//     let mut t: i32 = 3;
//     add_one(&mut t);
//     print_one(&t);
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {}


// fn invalid_output<'a>() -> &'a str { 
//     &"foo"
// }

// fn main() {
// }

// fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
//     println!("x is {} and y is {}", x, y);
// }

// fn fialed_borrow<'a>() {
//     let _x: i32 = 12;

//     let y: & i32 = &_x;
// }

// fn main() {
//     let (four, nine) = (4, 9);
//     print_refs(&four, &nine);
//     fialed_borrow();
// }

// #[derive(Debug)]
// struct Borrowed<'a>(&'a i32);

// #[derive(Debug)]
// struct NamedBorrowed<'a, 'b> {
//     x: &'a i32,
//     y: &'b i32,
// }

// #[derive(Debug)]
// enum Either<'b> {
//     Num(i32),
//     Ref(&'b i32),
// }

// fn main() {
//     let x = 18;
//     let y = 15;

//     let single: Borrowed = Borrowed(&x);
//     let double: NamedBorrowed = NamedBorrowed { x: &x, y: &y};
//     let reference: Either = Either::Ref(&x);
//     let number: Either = Either::Num(y);

//     println!("x is borrowed in {:?}", single);
//     println!("x and y are borrowed in {:?}", double);
//     println!("x is borrowed in {:?}", reference);
//     println!("y is *not* borrowed in {:?}", number);
// }

// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType
// }

// fn main() {
//     let var_a: u32 = 35;
//     let example: Example;

//     {
//         let var_b: NoCopyType = NoCopyType {};

//         example = Example { a: &var_a, b: &var_b};
//     }
//     println!("(Success) {:?}", example);
// }


// #[derive(Debug)]
// struct NoCopyType {}

// #[derive(Debug)]
// #[allow(dead_code)]
// struct Example<'a, 'b> {
//     a: &'a u32,
//     b: &'b NoCopyType,
// }

// fn fix_me<'a>(foo: &'a Example) -> &'a NoCopyType {
//     foo.b
// }
// fn main() {
//     let no_copy: NoCopyType = NoCopyType {};
//     let example: Example = Example { a: &1, b: &no_copy };
//     fix_me(&example);
//     println!("Success");
// }

// struct ImportantExcerpt<'a> {
//     part: &'a str,
// }

// impl ImportantExcerpt<'_> {
//     fn level<'a>(&'a self) -> i32 {
//         3
//     }
// }


// fn main() {}

// fn input(x: &i32) {
//     println!("`annotated_input`: {}", x);
// }

// fn pass(x: &i32) -> &i32 {
//     x
// }

// fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     x
// }

// struct Owner(i32);
// impl Owner {
//     fn add_one(&mut self) { self.0 += 1; }
//     fn print(&self) {
//         println!("`print: {}`", self.0);
//     }
// }

// struct Person<'a> {
//     age: u8,
//     name: &'a str,
// }

// enum Either<'a> {
//     Num(i32),
//     Ref(&'a i32),
// }

// fn main() {}

// fn main() {

//     let v: &'static str = "hello";
//     need_static(v);
//     println!("Success");
// }

// fn need_static(r: &'static str) {
//     assert_eq!(r, "hello");
// }

// fn main() {
//     let static_string: &'static str = "I'm in read-only memory";
//     {
//         println!("static_string: {}", static_string);
//     }
//     println!("static_string reference remains alive: {}", static_string);   
// }

// static NUM: i32 = 18;

// fn coerce_static<'a>(_: &'a i32) -> &'a i32 {
//     &NUM
// }

// fn main() {
//     {
//         let lifetime_nun: i32 = 9;

//         let coerced_static: &i32 = coerce_static(&lifetime_nun);

//         println!("coerced_static: {}", coerced_static);
//     }

//     println!("NUM: {} stays accessible!", NUM);
// }

// use std::fmt::Debug;

// fn print_it<T: Debug + 'static>(input: T) {
//     println!("'static value passed in is: {:?}", input );
// }

// fn print_it1(input: impl Debug + 'static) {
//     println!("'static value passed in is: {:?}", input);
// }

// fn print_it2<T: Debug + 'static>(input: &T) {
//     println!("'static value passed in is: {:?}", input);
// }

// fn main() {
//     static i: i32 = 5;
//     print_it(i);

//     print_it(&i);

//     print_it1(&i);

//     print_it2(&i);
// }

// use std::fmt::Debug;

// fn print_it<T: Debug + 'static>(input: T) {
//     println!("'static value passed in is: {:?}", input);
// }

// fn print_it1(input: impl Debug + 'static) {
//     println!("'static value passed in is: {:?}", input);
// }

// fn print_it2<T: Debug + 'static>(input: &T) {
//     println!("'static value passed in is: {:?}", input);
// }

// fn main() {

//     const i: i32 = 5;
//     print_it(i);

//     print_it(&i);

//     print_it1(&i);

//     print_it2(&i);
// }

// fn main() {
//     let x: i32 = 1;
//     let closure = |val| val + 1; 
//     assert_eq!(closure(2), 3);
// }

// fn main() {
//     fn function(i: i32) -> i32 { i + 1}

//     let closure_annotated = |i: i32| i + 1;
//     let closure_inferred = |j| j + 1;

//     let i: i32 = 1;

//     println!("function: {}", function(i));
//     println!("closure_annotated: {}",closure_annotated(i));
//     println!("closure_inferred: {}",closure_inferred(i));
// }

// fn main() {
//     let color: String = String::from("green");

//     let print =  || println!("`color`: {}", color);

//     print();
//     print();

//     let _reborrow: &String = &color;

//     println!("{}", color);
// }


// fn main() {
//     let mut count: i32 = 0;

//     let mut inc = move || {
//         count += 1;
//         println!("`count`: {}", count);
//     };

//     inc();

//     let _reborrow: &mut i32 = &mut count;

//     inc();

//     let _count_reborrowed: &mut i32 = &mut count;

//     assert_eq!(count, 0);
// }


// fn main() {
//     let movable: Box<i32> = Box::new(3);

//     let consume = || {
//         println!("`movable`: {:?}", movable);
//         take(movable);
//     };

//     consume();
//     consume();
// }

// fn take<T>(_v: T) {}


// use std::fmt::Display;

// fn print_a<T: Display + 'static>(t: &T) {
//     println!("{}", t);
// }

// fn print_b<T>(t:&T) where T: Display + 'static {
//     println!("{}", t);
// }

// fn print_c(t: &'static dyn Display) {
//     println!("{}", t);
// }

// fn print_d(t: &'static impl Display) {
//     println!("{}", t);
// }

// fn print_e(t: &(dyn Display + 'static)) {
//     println!("{}", t);
// }

// fn main() {
//     let mut string: String = "First".to_owned();

//     string.push_str(string.to_uppercase().as_str());
//     print_a(&string);
//     print_b(&string);
//     // print_c(&Box::new(string)); 
//     // print_d(&string); // Compilation error
//     // print_e(&string);
//     // print_f(&string);
//     // print_g(&string); // Compilation error
// }

// fn main() {
//     let example_closure= |x| x;

//     let s: String = example_closure(String::from("hello"));

//     let n: String = example_closure(5.to_string());
// }


// fn main() {
//     let v: Vec<i32> = vec![1, 2, 3];
//     fn_once(|z|z== v.len());
// }

// fn fn_once<F>(func: F) where F: Fn(usize) -> bool,
// {
//     println!("{}", func(3));
//     println!("{}", func(4));
// } 

// fn main() {
//     let mut s: String = String::new();

//     let update_string = |str| s.push_str(str);

//     exec(update_string);

//     println!("{:?}", s);
// }

// fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
//     f("hello")
// }


// fn main() {
//     let x: i32 = 1;
//     let closure = |x| x + 1;
//     assert_eq!(closure(2), 3)
// }

// fn main() {
//     let color: String = String::from("green");

//     let print = || println!("`color`: {}", color);

//     print();
//     print();

//     let _reborrow: &String = &color;
//     println!("{}", color);
// }


// fn main() {
//     let mut count: i32 = 0;

//     let mut inc = move || {
//         count += 1;
//         println!("`count`: {}", count);
//     };

//     inc();

//     let _reborrow: &i32 = &count;

//     inc();

//     let _count_reborrowed: &mut i32 = &mut count;

//     assert_eq!(count, 0);
// }

// fn main() {
//     let movable = Box::new(3);

//     let consume = || {
//         println!("`movable`: {:?}", movable);
//         take(&movable);
//     };

//     consume();
//     consume();
// }

// fn take<T>(_v: &T) {}


// fn main() {
//     let example_closure = |x| x;

//     let s: String = example_closure(String::from("hello"));

//     let n: String = example_closure(5.to_string());
// }


// fn fn_once<F>(func: F) where F: FnOnce(usize) -> bool,
// {
//     println!("{}", func(3)); // true
//     println!("{}", func(4)); // false
// }

// fn main() {
//     let x: Vec<i32> = vec![1, 2, 3];
//     fn_once(|z| { z == x.len()});
// }

// fn main() {
//     let mut s: String = String::new();

//     let update_string = |str| s.push_str(str);

//     exec(update_string);

//     println!("{:?}", s);
// }

// fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
//     f("hello")
// }

// fn main() {
//     use std::mem;

//     let greeting: &str = "hello";

//     let mut farewell: String = "goodbye".to_owned();

//     let diary = || {

//         println!("I said {}.", greeting);

//         farewell.push_str("!!!");

//         println!("Then I screamed {}.", farewell);
//         println!("Now I can sleep. zzz");

//         mem::drop(farewell);
//     };

//     apply(diary);

//     let double = |x| 2 * x;
    
//     println!("3 doubled: {}", apply_to_3(double));
// }

// fn apply_to_3<F>(f: F) -> i32 where F: Fn(i32) -> i32 {
//     f(3)
// }

// fn apply<F>(f: F) where F: FnOnce() {
//     f();
// }

// fn main() {
//     let s: String = String::new();

//     let update_string = move || println!("{}", s);

//     exec(update_string);
// }

// fn exec<F: FnOnce()>(f: F) {
//     f()
// }

// fn main() {
//     let mut s: String = String::new();

//     let update_string = |str| -> String {s.push_str(str); s};

//     exec(update_string);
// }

// fn exec<'a, F: FnOnce(&'a str) -> String >(mut f: F) {
//     f("hello");
// }

// fn call_me<F>(f: F) where F: Fn() {
//     f();
// }

// fn function() {
//     println!("I'm a function!");
// }

// fn main() {
//     let closure = || println!("I'm a closure");

//     call_me(closure);
//     call_me(function);
// }

// fn create_fn() -> Box<dyn FnOnce(i32) -> i32> {
//     let num: i32 = 5;
//     Box::new(move |x| x + num)
// }
 
// fn main() {
//     let fn_plain: Box<dyn FnOnce(i32) -> i32> = create_fn();
//     fn_plain(1);
// }

// fn create_fn() -> impl Fn(i32) -> i32 {
//     let num: i32 = 5;
//     move |x|x + num
// }
// fn main() {
//     let fn_plain = create_fn();
//     fn_plain(1);

// }


// fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
//     let num: i32 = 5;

//     if x > 1 {
//         Box::new(move |x: i32| x + num)
//     } else {
//         Box::new(move |x: i32| x + num)
//     }
// }

// fn main() {}

// struct Cacher<T, E>
// where 
//     T: Fn(E) -> E,
//     E: Copy
// {
//     query: T,
//     value: Option<E>,
// }

// impl<T, E> Cacher<T, E>
// where
//     T: Fn(E) -> E,
//     E: Copy
// {
//     fn new(query: T) -> Cacher<T, E> {
//         Cacher {
//             query,
//             value: None,
//         }
//     }

//     fn value(&mut self, arg: E) -> E {
//         match self.value {
//             Some(v) => v,
//             None => {
//                 let v = (self.query) (arg);
//                 self.value = Some(v);
//                 v
//             }
//         }
//     }
// }

// fn main() {

// }

// #[test]
// fn call_with_different_values() {
//     let mut c = Cacher::new(|a| a);
//     let v1 = c.value(1);
//     let v2 = c.value(2);

//     assert_eq!(v2, 1);
// }

// fn main() {
//     let v = vec![1, 2, 3];
//     for x in v {
//         println!("{}", x)
//     }
// }

// fn main() {
//     let v = vec![1, 2, 3];
//     for x in v.into_iter() {
//         println!("{}", x)
//     }
// }

// fn main() {
//     let arr = [0; 10];
//     for i in 0..arr.len() {
//         println!("{}", arr[i]);
//     }
// }

// fn main() {
//     let mut v: Vec<i32> = Vec::new();
//     for n in 0..100 {
//         v.push(n);
//     }

//     assert_eq!(v.len(), 100);
// }

// pub trait Iterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

// fn main() {}

// use std::vec::IntoIter;

// fn main() {
//     let mut v1: IntoIter<i32> = vec![1, 2].into_iter(); 

//     assert_eq!(v1.next(), Some(1));
//     assert_eq!(v1.next(), Some(2));
//     assert_eq!(v1.next(), None);

// }

// fn main() {
//     let mut names: Vec<&str> = vec!["Bob", "Frank", "Ferris"];

//     for name in names.iter_mut() {
//         *name = match name {
//         &mut "Ferris" => "there is a rustacean among us!",
//         _ => "hello",    
//         }
//     }

//     println!("names: {:?}", names);
// }


// fn main() {
//     let mut values: Vec<i32> = vec![1, 2, 3];
//     let mut values_iter = values.iter_mut();

//     if let Some(v) = values_iter.next() {
//         *v = 0;
//     }

//     assert_eq!(values, vec![0, 2, 3]);
// }

// struct Counter {
//     count: u32,
// }

// impl Counter {
//     fn new() -> Counter {
//         Counter { count: 0}
//     }
// }

// impl Iterator for Counter {
//     type Item = u32;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.count < 5 {
//             self.count += 1;
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

// fn main() {
//     let mut counter = Counter::new();
    
//     assert_eq!(counter.next(), Some(1));
//     assert_eq!(counter.next(), Some(2));
//     assert_eq!(counter.next(), Some(3));
//     assert_eq!(counter.next(), Some(4));
//     assert_eq!(counter.next(), Some(5));
//     assert_eq!(counter.next(), None);
// }

// #[derive(Debug)]
// struct Fibonacci {
//     curr: u32,
//     next: u32,
// }

// impl Iterator for Fibonacci {
//     type Item = u32;
//     // 0, 1, 1, 2, 3, 5, 8
//     fn next(&mut self) -> Option<Self::Item> {
//         let returned: u32 = self.next;

//         self.next += self.curr;
//         self.curr = returned;

//         Some(returned)
//     }
// }

// fn fibonacci() -> Fibonacci {
//     Fibonacci { curr: 0, next: 1}
// }

// fn main() {
//     let mut fib = fibonacci();
//     println!("{:?}",fib.next());
//     println!("{:?}",fib.next());
//     println!("{:?}",fib.next());
//     println!("{:?}",fib.next());
//     println!("{:?}",fib.next());
//     println!("{:?}",fib.next());
// }

// #[derive(Debug)]
// struct Func {
//     curr: i32,
//     next: i32,
// }

// impl Func {
//     // fn new(&self) -> Self {
//     //     Func {
//     //         curr: 0,
//     //         next: 1,
//     //     }
//     // }
//     // 0, 1, 1, 2, 3, 5, 8
//     fn func(&mut self) -> i32 {
//         let hold: i32 = self.curr; //return

//         self.curr = self.next;
//         self.next += hold;

//         hold
//     }
// }

// fn main() {
//     let mut f: Func = Func { curr: 0, next: 1};
//     println!("{:?}", f.func());
//     println!("{:?}", f.func());
//     println!("{:?}", f.func());
//     println!("{:?}", f.func());
//     println!("{:?}", f.func());
//     println!("{:?}", f.func());
//     println!("{:?}", f.func());
//     println!("{:?}", f.func());
//     println!("{:?}", f.func());
//     println!("{:?}", f.func());
// }

// fn reverse(input: &str) -> String {
//     let mut out: String = String::new();

//     for i in 0..input.len() {
//         if let Some(c) = input.chars().nth(input.len() - i - 1) {
//             out.push(c);
//         }
//     }
//     out
// }


// fn main() {
//     let out = reverse("hello");
//     println!("{}", out);
// }

// use std::fmt;

// struct Wrapper(Vec<String>);

// impl fmt::Display for Wrapper {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "[{}]", self.0.join(", "))
//     }
// }

// fn main() {
//     let w: Wrapper = Wrapper(vec![String::from("hello"), String::from("world")]);
//     println!("w = {}", w);
// }

// struct Meters(u32);

// fn main() {
//     let i: u32 = 2;
//     assert_eq!(i.pow(2), 4);

//     let n: Meters = Meters(i);
//     assert_eq!(n.0.pow(2), 4);
// }

// struct Years(i64);

// struct Days(i64);

// impl Years {
//     pub fn to_days(&self) -> Days {
//         Days(self.0 * 365)
//     }
// }

// impl Days {
//     pub fn to_years(&self) -> Years {
//         Years(self.0 / 365)
//     }
// }

// trait Content {
//     fn inner(&self) -> i64;
// }

// impl Content for Years {
//     fn inner(&self) -> i64 {
//         self.0
//     }
// }

// impl Content for Days {
//     fn inner(&self) -> i64 {
//         self.0
//     }
// }

// fn old_enough<T: Content>(age: &T) -> bool {
//     age.inner() >= 18
// }

// fn main() {
//     let age: Years = Years(5);
//     let age_days: Days = age.to_days();
//     println!("Old enough {}", old_enough(&age));
//     println!("Old enough {}", old_enough(&age_days));
// }

// use std::ops::Add;
// use std::fmt::{self, format};

// struct Meters(u32);
// impl fmt::Display for Meters {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "There are still {} meters left", self.0)
//     }
// }

// impl Add for Meters {
//     type Output = Self;

//     fn add(self, other: Meters) -> Meters {
//         Self(self.0 + other.0)
//     }
// }

// fn calculate_distance(first: Meters, second: Meters) -> Meters {
//     first.add(second) 
// }

// fn main() {
//     let d = calculate_distance(Meters(10), Meters(20));
//     assert_eq!(format!("{}", d), "There are still 30 meters left");
// }

// type Thunk = Box<dyn Fn() + Send + 'static>;

// let f: Thunk = Box::new(|| println!("hi"));

// fn takes_long_type(f: Thunk) {

// }

// enum VeryVerboseEnumOfThingsToDoWithNumbers {
//     Add,
//     Substract,
// }

// impl VeryVerboseEnumOfThingsToDoWithNumbers {
//     fn run(&self, x: i32, y: i32) -> i32 {
//         match self {
            
//         }
//     }
// }

// fn main() {}

// fn my_function<const N: usize>() -> [i32; N] {
//     [123; N]
// }
// fn main() {
//     let arr: [i32; 3] = my_function::<3>();
//     println!("{:?}", arr);
// }

// fn main() {
//     let s: &str = "Hello There!";
//     let arr: &[u8] = &[1, 2, 3];
// }

// use std::fmt::Display;
// fn foobar(thing: &dyn Display) {}

// fn main() {
// }

// fn main() {
//     let a: i32 = 5;
//     let b: &mut i32 = &mut a;

//     let mut c: i32 = 10;
//     let d: &i32 = &c;
//     *d = 20;
// }

// pub trait Messenger {
//     fn send(&self, msg: &str); // here a immutable reference to T, which is Messenger.messenger
// }

// pub struct LimitTracker<'a, T: Messenger> { // 3ndna reference donc khasna n annotew lifetime
//     messenger: &'a T,  
//     value: usize,
//     max: usize,
// }

// impl<'a, T> LimitTracker<'a, T> where T: Messenger, {
//     pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
//         LimitTracker {
//             messenger,
//             value: 0,
//             max,
//         }
//     }

//     pub fn set_value(&mut self, value: usize) {
//         self.value = value;

//         let percentage_of_max = self.value as f64 / self.max as f64;

//         if percentage_of_max >= 1.0 {
//             self.messenger.send("Error: You are over your quota!");
//         } else if percentage_of_max >= 0.9 {
//             self.messenger.send("Urgent warning: You've used up over 90% ");
//         } else if percentage_of_max >= 0.75 {
//             self.messenger.send("Warning: You've used up over 75%");
//         }
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::*;
//     struct MockMessenger { 
//         sent_messages: : RefCell<Vec<String>>, // drnah west ref cell
//     }

//     impl MockMessenger {
//         fn new() -> MockMessenger {
//             MockMessenger {
//                 sent_messages: RefCell::new(vec![]),
//             }
//         }
//     }

//     // in refCell we can take only one immutable reference
//     impl Messenger for MockMessenger {
//         send(&self, message: &str) { // we cannot take &mut self, because inb line 3022 we took immutable reference
//                     // so we cannot push here
//             self.sent_messages.borrow_mut().push(String::from(message)); // borrow_mut(): we took a mutable borrow 
//         } // so we need to use refCell to mutate a the data inside an immutable reference  
//     }

//     #[test]
//     fn it_sends_an_over_75_percent_warning_message() {
//         let mock_messenger: MockMessenger = MockMessenger::new();
//         let mut limit_tracker: LimitTracker<MockMessenger> = LimitTracker::new(&mock_messenger, 100)
        
//         limit_tracker.set_value(80);

//         assert_eq!(mock_messenger.sent_messages.borrow().len(), 1); // we just need a simple reference 'immutable' so we use borrow()
//     }
// }


// #[derive(Debug)]
// enum List {
//     // Cons kadar bach tcombini lists w recursive ghtl9a che7 f notes line 27. w Nill hia l end.
//     Cons(Rc<RefCell<i32>>, Rc<List>),
//     Nil, 
// }

// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// fn main() {
//     let value: Rc<RefCell<i32>> = Rc::new(RefCell::new(5));

//     let a: Rc<List> = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))); //drna 2 hwayj ldakhl d Rc::new()

//     let b: List = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
//     let c: List = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

//     println!("a after = {:?}", a);
//     println!("b after = {:?}", b);
//     println!("c after = {:?}", c);
// }

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use List::{Cons, Nil};

// fn main() {
//     let list: List = *Box::new(Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil)))))));
// }

// use std::ops::Deref;

// #[derive(Debug)]
// struct MyBox<T>(T);

// impl<T> MyBox<T> {
//     fn new(x: T) -> Self {
//         MyBox(x)
//     }
// }

// impl<T> Deref for MyBox<T> {
//     type Target = T;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// } 

// fn main() {
//     let x: i32 = 5;
//     let y: MyBox<i32> = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);

//     let m: Box<String> = Box::new(String::from("rust"));
//     hello(&m);
// }

// fn hello(m: &str) {
//     println!("Hello {}", m);
// }

// struct CustomSmartPointer { data: String,}

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data: {} !", self.data);
//     }
// }

// fn main() {
//     let c: CustomSmartPointer = CustomSmartPointer { data: String::from("my stuff"), };

//     let d: CustomSmartPointer = CustomSmartPointer { data: String::from("other stuff"), };

//     println!("CustomSmartPointers created.");

//     // c.drop(); // compile error : hna ghaydropa w mn baed mli ykhroj mn scope ghayeawd itdropa chi li ghay2di ldouble drop 
//     // right methode
//     drop(c);

//     println!("CustomSmartPointer dropped before the end of main.");
// }

// use std::rc::Rc;
// use std::cell::RefCell;

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil,
// }

// use List::{Cons, Nil};


// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             Cons(_, item) => Some(item),//: &RefCell<Rc<List>>
//             Nil => None,
//         }
//     }
// }

// fn main() {
//     let a: Rc<List> = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

//     println!("a initial rc count: {}", Rc::strong_count(&a));
//     println!("a next item = {:?}", a.tail());

//     let b: Rc<List> = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

//     println!("a rc count after b creation = {}", Rc::strong_count(&a));
//     println!("b initial rc count = {}", Rc::strong_count(&b));
//     println!("b next item = {:?}", b.tail());

//     if let Some(link) = a.tail() { // : &RefCell<Rc<List>>
//         *link.borrow_mut() = Rc::clone(&b);
//     }

//     println!("b rc count after changing a = {}", Rc::strong_count(&b));
//     println!("a rc count after changing a = {}", Rc::strong_count(&a));

// }

// use std::cell::RefCell;
// use std::rc::{Rc, Weak};

// #[derive(Debug)]

// struct Node {
//     value: i32,
//     parent: RefCell<Weak<Node>>,
//     children: RefCell<Vec<Rc<Node>>>,
// }

// fn main() {
//     let leaf: Rc<Node> = Rc::new(Node {
//         value: 3,
//         parent: RefCell::new(Weak::new()),
//         children: RefCell::new(vec![]),
//     });

//     // println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // upgrade attemp to convert weak smart pointer into rc smart pointer

//     println!("leaf strong = {}, weak = {}", 
//                 Rc::strong_count(&leaf),
//                 Rc::weak_count(&leaf),);
//     {
//         let branch: Rc<Node> = Rc::new(Node {
//             value: 5,
//             parent: RefCell::new(Weak::new()),
//             children: RefCell::new(vec![Rc::clone(&leaf)]),
//         });
//         *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // opposite of upgrade convert rc smart pointer into weak smart pointer
        
//         println!("branch strong = {}, weak = {}", 
//         Rc::strong_count(&branch),
//         Rc::weak_count(&branch),);        

//     }

//     println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // upgrade attemp to convert weak smart pointer into rc smart pointer

// }



// threads

use std::{thread, time::Duration};

fn main() {
    thread::spawn(|| { // creating a thread that will loop printing
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1)); // after each loop wait 1msc
        }
    });


    for i in 1..=5 { // the programme thread normal thread
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1)); // samething
    }
    // when this loop ends the programme will exit not carring about
    // the spawned threads
}

















































