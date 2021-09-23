#![allow(dead_code)]


use std::convert::TryFrom;
use std::convert::TryInto;
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::Add;
use std::num::ParseIntError;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::{Sender, Receiver};
use std::path::Path;
use std::fs::File;
use std::io::Read;

// fn main() {
fn main_arr() {
    let array:[i32;5] = [1, 2, 3, 4, 5];
    println!("as is {:?}", array);
    let bs:[i32;6] = [111; 6];
    println!("bs is {:?}", bs);

    println!("bs mem size is {}", std::mem::size_of_val(&bs));
    ana_slice(&bs);
    ana_slice(&array[0 .. 1]);
    ana_slice(&array[1 .. 2]);
}

fn ana_slice(sl:&[i32]) {
    println!("first is {} and size is {}", sl[0], sl.len());
}

#[derive(Debug,Copy, Clone)]
struct Nil;
struct Pair(&'static str, i32);
struct StrPair(String, i32);
#[derive(Debug, Clone, Copy)]
struct Point {x: i32, y: i32}
#[derive(Debug)]
struct Rectangle {p1: Point, p2: Point}

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

// fn main() {
fn main_struct() {
    let name = "Peter";
    let ag:u8 = 27;
    let peter = Person{name, age:ag};
    println!("person is {:?}", peter);

    let point:Point = Point{x:11, y:22};
    let new_point = Point{x:33, ..point};
    println!("point is {:?}, new_point is {:?}", point, new_point);

    let Point{x:px, y:py} = new_point;
    println!("px is {:?}, py is {:?}", px, py);
    let _nil = Nil;

    let pair = Pair("xxxxaa", 2);
    println!("pair contains {} and {}", pair.0, pair.1);
    let Pair(px, py) = pair;
    println!("px is {:?}, py is {:?}", px, py);

    let str_pair = StrPair(String::from("stringxxx"), 2);
    let StrPair(px, py) = str_pair;
    println!("px is {:?}, py is {:?}", px, py);
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x:i64, y:i64}
}

fn inspect(event:WebEvent) {
    match event {
        WebEvent::PageLoad => {println!("PageLoad event")},
        WebEvent::PageUnload => println!("PageUnload event"),
        WebEvent::KeyPress(c) => println!("KeyPress {}", c),
        WebEvent::Paste(ps) => println!("Paste {}", ps),
        WebEvent::Click { x,y } => println!("Click {} {}", x, y),
    }
}

enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl Operations {
    fn run(&self, x:i32, y:i32) -> i32 {
        match &self {
            Self::Add => {x + y}
            Self::Subtract => {x - y}
        }
    }
}

// fn main(){
fn main_enum(){
    let page_load = WebEvent::PageLoad;
    inspect(page_load);
    let event = WebEvent::PageUnload;
    inspect(event);
    let event = WebEvent::KeyPress('w');
    inspect(event);
    let event = WebEvent::Paste("hdlqdq".to_owned());
    inspect(event);
    let event = WebEvent::Click {x:33, y:22};
    inspect(event);

    let add_opr = Operations::Add;
    println!("add_opr is {}", add_opr.run(3, 5));

    let sub_opr = Operations::Subtract;
    println!("sub_opr is {}", sub_opr.run(3, 5));
}

enum List {
    Next(i32, Box<List>),
    Nil
}

impl List {
    fn new() -> List { List::Nil }

    fn prepend(self, ele:i32) -> List {
        List::Next(ele, Box::new(self))
    }

    fn len(&self) -> i32 {
        match *self {
            List::Next(_, ref n) => {1 + n.len()}
            List::Nil => {1}
        }
    }

    fn stringfy(&self) -> String {
        match *self {
            List::Next(v, ref n) => { format!("{}, {}", v, n.stringfy())}
            List::Nil => {String::from("Nil")}
        }
    }
}

// fn main() {
fn main_enum_list() {
    let mut list:List = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("list len {:?} content {:?}", list.len(), list.stringfy());
}

// fn main() {
fn main_type_ana() {
    let ele = 2u8;
    let mut vec = Vec::new();
    println!("vec is {:?}", vec);
    vec.push(ele);
    println!("vec is {:?}", vec);
}

#[derive(Debug)]
struct MyNumber {
    value:i32
}

impl From<i32> for MyNumber {
    fn from(i:i32) -> Self {
        MyNumber {value:i}
    }
}

// fn main() {
fn main_from_to() {
    let n = MyNumber::from(11);
    println!("n is {:?}", n);
    let i = 55;
    let nn:MyNumber = i.into();
    println!("n is {:?}", nn);
}



#[derive(Debug, PartialEq)]
struct EvenNumber (i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

// fn main() {
fn main_try_from() {
    assert_eq!(EvenNumber::try_from(20), Ok(EvenNumber(20)));
    assert_eq!(EvenNumber::try_from(21), Err(()));

    let en:Result<EvenNumber, ()> = 22i32.try_into();
    assert_eq!(en, Ok(EvenNumber(22)));
    let en:Result<EvenNumber, ()> = 21i32.try_into();
    assert_eq!(en, Err(()));
}

// fn main() {
fn main_from_str() {
    let n1:i32 = "22".parse().unwrap();
    let n2 = "33".parse::<i32>().unwrap();
    println!("sum is {}", (n1 + n2));
}

// fn main() {
fn main_expression() {
    let mut x = 20;
    let y = {
      let a = x + x;
        let b = 2 + x;
        x + a + b
    };
    let z = {
        x = 30;
    };
    println!("x = {}, y = {}, z={:?}", x, y, z);
}

// fn main() {
fn main_if_else() {
    let n = 20;
    let m = if n > 20 { n - 20 }
        else if n > 10 {n - 10}
        else {n};
    println!("m is {}", m);
}

// fn main() {
fn main_loop() {
    let mut c = 0;
    loop {
        println!("looping c is {}", c);
        c += 1;
        if c == 3 {
            println!("c is {}", c);
            continue;
        }
        if c == 8 {
            println!("c is {}", c);
            break;
        }
    }
}

// fn main() {
fn main_nested_loop() {
    let mut n = 0;
    'outer: loop {
        n += 1;
        println!("n is {}", n);
        let mut m = 0;
        'inner: loop {
            m += 1;
            println!("m is {}", m);
            if m >= 3 {
                break 'inner;
            }
        }
        if n >= 2 {
            break 'outer;
        }
    }
}

// fn main() {
fn main_loop_value() {
    let mut a = 1;
    let b = loop {
        if a > 15 {
            break a
        }
        a *= 2;
    };
    println!("b is {}", b);
}

// fn main() {
fn main_while() {
    let mut c = 0;
    while c < 100 {
        if c % 5 == 0 {
            println!("c is 5 multiplied {}", c);
        } else if c % 9 == 0 {
            println!("c is 9 multiplied {}", c);
        }
        c += 1;
    }
}

// fn main() {
fn main_for() {
    for c in 0..=(100 - 1) {
        if c % 5 == 0 {
            println!("c is 5 multiplied {}", c);
        } else if c % 9 == 0 {
            println!("c is 9 multiplied {}", c);
        }
    }
}

// fn main() {
fn main_for_iter() {
    let names = vec!["Alisa", "Bella", "Celina", "Diana"];
    for name in names.iter() {
        match name {
            &"Bella" => {println!("Hello, I am {}", name)}
            _ => {println!("My name is {}", name)}
        }
    }
    println!("names is {:?}", names);
}

// fn main() {
fn main_for_into_iter() {
    let names = vec!["Alisa", "Bella", "Celina", "Diana"];
    for name in names.into_iter() {
        match name {
            "Bella" => {println!("Hello, I am {}", name)}
            _ => {println!("My name is {}", name)}
        }
    }
    // println!("names is {:?}", names);
}

// fn main() {
fn main_iter_mut() {
    let mut names = vec!["Alisa", "Bella", "Celina", "Diana"];
    println!("names is {:?}", names);
    for name in names.iter_mut() {
        *name = match name {
            &mut "Bella" => {"GoodBella"}
            _ => { "ok"}
        }
    }
    println!("names is {:?}", names);
}

// fn main() {
fn main_match() {
    let a = 5;
    match a {
        _ if a > 8 => println!("hello {}", a),
        1 | 2 | 3 => println!("ok"),
        5..=8 => println!("good"),
        _ => println!("ookk")
    }
    let b = false;
    let c = match b {
        false => {2},
        true => {1}
    };
    println!("c is {}", c);
}

// fn main() {
fn main_match_pair() {
    let pair = (1, 6);
    match pair {
        (1, 2) => println!("first match"),
        (1, y) if y > 3 => println!("second match"),
        (_, 2) => println!("third match"),
        _ => println!("ok")
    }
}

// fn main() {
fn main_match_ref() {
    let a = &3;
    match a {
        b => println!("a is {}", b)
    }

    let b = &4;
    match b {
       &c => println!("c is {}", c)
    }

    match *a {
        val=> println!("val is {}", val)
    }

    let not_ref = 5;
    match not_ref {
        ref val => println!("val is {}", val)
    }

    let mut not_ref_mut = 6;
    match not_ref_mut {
        ref mut val=> { *val = 88; println!("val is {}", val)}
    }
    println!("not_ref_mut is {}", not_ref_mut);
}

#[derive(Debug)]
struct MyTuple(i32, i32);

struct MatchStruct {
    x: MyTuple,
    y: i32
}

// fn main() {
fn main_struct_destructure() {
    let ms : MatchStruct = MatchStruct{x: MyTuple(2, 8), y: 22};
    match ms {
        MatchStruct{x: MyTuple(m, n), y : 22} => println!("y is 22, x is {:?} {:?}", m, n),
        MatchStruct { .. } => {}
    }
    let MatchStruct{x: xx, ..} = ms;
    println!("xx is {:?}", xx);
}

fn age() -> u32 {
    16
    // 99
}

// fn main() {
fn main_match_binding() {
    match age() {
        12 => println!("age is 12"),
        a @ 1..=99 => println!("age is {}", a),
        a if a > 99 => println!("big age is {}", a),
        _ => {}
    }
}

fn some_number() -> Option<(i32,i32)> {
    Some((8,22))
}

// fn main() {
fn main_match_binding_enum() {
    match some_number() {
        Some((x@ 1..=10, _) ) => {println!("x is {}", x)}
        Some((_, y)) if y > 20 => {println!("xxx is {}", y)}
        Some(_) => {}
        None => {}
    }
}

// fn main() {
fn main_let_if() {
    let number = Some(22);
    let letter:Option<i32> = None;
    let emoticon:Option<i32> = None;

    if let Some(i) = number {
        println!("i is {}", i);
    }

    if let Some(i) = letter {
        println!("letter is {}", i);
    } else {
        println!("letter is none");
    }

    let i_like_letter = false;

    if let Some(i) = emoticon {
        println!("emoticon is {}", i);
    } else if i_like_letter {
        println!("i_like_letter");
    } else {
        println!("emoticon is none");
    }
}

enum FooLet {
    Bar, Baz, Qux(u32)
}

// fn main() {
fn main_if_let_enum() {
    let foo_bar = FooLet::Bar;
    let foo_qux = FooLet::Qux(22);

    if let FooLet::Qux(i) = foo_qux {
        println!("qux is {}", i);
    } else {
        println!("not a qux");
    }

    if let FooLet::Bar = foo_bar {
        println!("is Bar");
    } else {
        println!("not a Bar");
    }
}

// fn main() {
fn main_while_let() {
    let mut opt = Some(0);
    while let Some(val) = opt {
        println!("opt is {:?}", opt);
        if val > 5 {
            // opt = None;
            break;
        } else {
            opt = Some(val + 1)
        }
    }
    println!("opt is {:?}", opt);
}

impl Point {
    fn origion() -> Point {
        Point{x:0, y:0}
    }
    fn new(a:i32, b:i32) -> Point {
        Point{x:a, y:b}
    }
}

impl Rectangle {
    fn area(&self) -> i32 {
        let Point{x:x1, y:y1} = &self.p1;
        let Point{x:x2, y:y2} = &self.p2;
        // ((&self.p1.x - &self.p2.x) * (&self.p1.y - &self.p2.y)).abs()
        return ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> i32 {
        let Point{x:x1, y:y1} = &self.p1;
        let Point{x:x2, y:y2} = &self.p2;
        return 2 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.p1.x += dx;
        self.p1.y += dy;
        // self.p1 = Point{x: &self.p1.x + dx, y: &self.p1.y + dy};
        self.p2 = Point{x: &self.p2.x + dx, y: &self.p2.y + dy};
    }

}

#[derive(Debug)]
struct PairBox(Box<i32>, Box<i32>);

impl PairBox {
    fn destroy(self) {
        let PairBox(a, b) = self;
        println!("PairBox destroyed {} {}", a, b);
    }
}

// fn main() {
fn main_method() {
    let mut rect:Rectangle = Rectangle{p1:Point::origion(), p2:Point::new(5,5)};
    println!("rect perimeter is {}", rect.perimeter());
    rect.translate(-1, -1);
    println!("rect is {:?} perimeter is {}", rect, rect.perimeter());

    let pb = PairBox(Box::new(32), Box::new(22));
    println!("pb is {:?}", pb);
    pb.destroy();
    // println!("pb after destroy is {:?}", pb);
}



// fn main() {
fn main_closure() {
    fn calc(i:i32) -> i32 {i + 1}
    let calc_closure = |i:i32| -> i32 {i + 1};
    let calc_closure_infer = |i| i + 1;

    let i = 1;
    println!("calc is {}", calc(i));
    println!("calc_closure is {}", calc_closure(i));
    println!("calc_closure_infer is {}", calc_closure_infer(i));

    let two = ||false;
    println!("two is {}", two());
}

// fn main() {
fn main_closure_move() {
    let color = "Red";
    let print = || println!("`color` is {}", color);

    print();
    print();

    let mut i = 0;
    let mut count = || i += 1;
    count();
    count();
    // println!("i is {}", i);
    count();
    println!("i is {}", i);

    let movable = Box::new(3);
    let consume = || std::mem::drop(movable);
    consume();
    // consume();

    let haystack = vec![1,2,3,4];
    let contains = move |item| {let b = haystack.contains(item); println!("contains {} {}", item, b); b};
    contains(&1);
    contains(&5);
    // println!("haystack len is {}", haystack.len());
}

// fn apply(f:F) where F:FnOnce(){
//     f();
// }

fn apply<F>(f: F) where
    F: FnOnce() {
    f();
}

fn apply_to_3<F>(f:F) -> i32 where F:Fn(i32) -> i32 {
    f(3)
}

// fn main() {
fn main_closure_type() {
    let greeting = "hello";
    let mut farewell = greeting.to_owned();

    let diary = || {
      println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzz");
        std::mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled is {}", apply_to_3(double));

}

fn call_me<F:Fn()>(f:F) {
    f()
}

fn my_function() {
    println!("I'm a function");
}

// fn main() {
fn main_closure_arg() {
    let clo = || println!("I'm a closure");
    call_me(my_function);
    call_me(clo);
}

fn is_odd(n:u32) -> bool {
    n % 2 == 1
}

// fn main() {
fn main_function_high() {
    let upper = 1000;
    let mut c = 0;
    for n in 0.. {
        let ns = n * n;
        if ns > upper {
            break
        } else if is_odd(ns) {
            c += ns;
        }
    }
    println!("imperative res is {}", c);

    let fc = (0..).map(|n| n* n).take_while(|&n|n<upper)
        .filter(|&n|is_odd(n)).fold(0, |sum, i| sum + i);
    println!("functional res is {}", fc);
}

mod my {
    pub struct OpenBox<T> {
        pub contents: T
    }

    pub struct ClosedBox<T> {
        contents: T
    }

    impl <T> ClosedBox<T> {
        pub fn new(c:T) -> ClosedBox<T> {
            ClosedBox {
                contents: c
            }
        }
    }
}

// fn main() {
fn main_mod_scope() {
    let ob = my::OpenBox{contents: 22};
    println!("contents in ob is {}", ob.contents);
    // let cb =  my::ClosedBox::new(21);
    // println!("contents in cb is {}", cb.contents);
}

#[cfg(target_os = "Linux")]
fn are_you_on_linux() {
    println!("yes you are on Linux");
}

#[cfg(not(target_os = "Linux"))]
fn are_you_on_linux() {
    println!("no you are not on Linux");
}

// fn main() {
fn main_prop_cfg() {
    are_you_on_linux();
    if cfg!(target_os = "linux") {
        println!("yes linux");
    } else {
        println!("no linux");
    }
}

struct SingleGen<T>(T);

// fn main() {
fn main_generic() {
    struct A;
    struct Single(A);
    let _s = Single(A);
    let _sg:SingleGen<i32> = SingleGen(22);
    let _sga = SingleGen("aa");
    let _sgb = SingleGen(false);
}

fn gen_func<T>(_sg:SingleGen<T>) {
}

impl <T> SingleGen<T> {
    fn val(&self) -> &T {
        &self.0
    }
}

// fn main() {
fn main_generic_func() {
    gen_func(SingleGen(32));
    gen_func(SingleGen(true));

    println!("generic impl test {}", SingleGen(22).val());
    println!("generic impl test {}", SingleGen(true).val());
    println!("generic impl test {}", SingleGen("hhhqqq").val());
}

struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl <T,U> DoubleDrop<T> for U {
    fn double_drop(self, _: T) {
    }
}

// fn main() {
fn main_generic_trait() {
    let empty = Empty;
    let null = Null;
    empty.double_drop(null);
    // empty;
    // null;
}

fn compare_prints<T:std::fmt::Display + std::fmt::Debug>(t:T) {
    println!("debug {:?}", t);
    println!("display {}", t);
}

fn compare_types<T:Debug, U:Debug>(t:T, u:U) {
    println!("t: {:?}", t);
    println!("u: {:?}", u);
}

// fn main() {
fn main_multi_constraint() {
    let string = "words";
    let ass = [1,2,3];
    let bs = vec![1,2,3];
    compare_prints(string);
    println!("string is {:?}", string);
    // compare_prints(ass);
    compare_types(&ass, &bs);
}

trait PrintInOption{
    fn print_in_option(self);
}

impl <T> PrintInOption for T where Option<T>:Debug {
    fn print_in_option(self) {
        println!("print_in_option {:?}", Some(self));
    }
}

// fn main() {
fn main_where_constraint() {
    let ass = vec![1,2,3];
    ass.print_in_option();
}

#[derive(Debug)]
struct Container(i32, i32);

trait Contains {
    type A;
    type B;
    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
}



impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool {
        &self.0 == a && &self.1 == b
    }
}

fn difference(c: &Container) -> i32 {
    c.0 - c.1
}

// fn main() {
fn main_trait_type_bounds() {
    let c = Container(22, 55);
    println!("contains {}", c.contains(&22, &55));
    println!("difference {}", difference(&c));
    println!("c is {:?}", c);
}

#[derive(Clone, Copy, Debug)]
struct Inch;
#[derive(Clone, Copy, Debug)]
struct Mm;

#[derive(Clone, Copy, Debug)]
struct Length<T>(f64, PhantomData<T>);

impl <T> Add for Length<T> {
    type Output = Length<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

// fn main() {
fn main_generic_void_constraint() {
    let li:Length<Inch> = Length(22f64, PhantomData);
    let lm:Length<Mm> = Length(100f64, PhantomData);

    let dli = li + li;
    println!("dli {:?}", dli);

    let dlm = lm + lm;
    println!("dlm {:?}", dlm);

    // let dim = li + lm;
}

fn create_box() {
    let _box1 = Box::new(32);
}

// fn main() {
fn main_raii() {
    let _box2 = Box::new(22);
    println!("box2 is {}", _box2);
    {
        let _box3 = Box::new(99);
    }
    for _ in 0..1000 {
        create_box();
    }
}

struct ToDrop;
impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("drop ToDrop");
    }
}

// fn main() {
fn main_drop() {
    let _a = ToDrop;
    println!("ToDrop created");
}

fn destroy_box(b: Box<i32>) {
    println!("destroying box {:?}", b);
}

// fn main() {
fn main_ownership() {
    let x = 0;
    let y = 1;
    println!("x is {}, y is {}", x, y);

    let a = Box::new(22);
    println!("a is {}", a);
    let b = a;
    // println!("a is {}", a);
    println!("b is {}", b);
    destroy_box(b);
    // println!("b is {}", b);
}

// fn main() {
fn main_mut_variable() {
    let mbox = Box::new(22);
    // *mbox = 55;
    let mut mbox = mbox;
    *mbox = 55;
    println!("mbox is {}", mbox);
}

// fn main() {
fn main_partical_movement() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u8
    }
    let person = Person{name: "Alisa".to_string(), age: 22};
    println!("person is {:?}", person);
    let Person{name, ref age} = person;
    // let name = person.name;
    // let age = &person.age;
    println!("name is {}", name);
    println!("age is {}", age);
    // println!("person is {:?}", person);
    // println!("age of person is {}", person.name);
    println!("age of person is {}", person.age);
}

fn eat_box_i32(a: Box<i32>) {
    println!("destroying {}", a);
}

fn borrow_i32(a: &i32) {
    println!("borrowing {}", a);
}

// fn main() {
fn main_ownership_borrow() {
    let box32 = Box::new(22);
    let stack32 = 21;
    borrow_i32(&box32);
    borrow_i32(&stack32);
    {
        let _ref_box32 = &box32;
        // eat_box_i32(box32);
        borrow_i32(_ref_box32);
    }
    eat_box_i32(box32);
}

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32
}

fn borrow_book_immutable(b: &Book) {
    println!("borrow_book_immutable {:?}", b);
}

fn borrow_book_mutable(b: &mut Book) {
    b.year = 2009;
    println!("borrow_book_mutable {:?}", b);
}

// fn main() {
fn main_mutable_ownership() {
    let book = Book{
        author: "Alisa", title: "The Success", year: 2000
    };
    let mut m_book = book;
    borrow_book_immutable(&book);
    borrow_book_immutable(&m_book);
    borrow_book_mutable(&mut m_book);
    // borrow_book_mutable(&mut book);
}

struct Point3 {
    x:i32, y:i32, z:i32
}

// fn main() {
fn main_mutable_borrow() {
    let mut point = Point3{x:1, y:2, z:3};
    let borrowed_point = &point;
    let another_borrow = &point;
    println!("Point has coordinates ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);

    // let mutable_borrow = &mut point;
    println!("Point has coordinates ({}, {}, {})", borrowed_point.x, another_borrow.y, point.z);

    let mutable_borrow = &mut point;
    mutable_borrow.x = 0;
    mutable_borrow.y = 0;
    mutable_borrow.z = 0;

    // let y = &point.y;
    // println!("Point z is {}", point.z);
    println!("Point has coordinates ({}, {}, {})", mutable_borrow.x, mutable_borrow.y, mutable_borrow.z);

    let new_borrowed_point = &point;
    println!("Point has coordinates ({}, {}, {})", new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z);
}

// fn main() {
fn main_ref_destructure() {
    let c = 'Q';
    let ref cf = c;
    let cfa = &c;
    println!("cf equals cfa is {}", *cf == *cfa);

    let point = Point{x:0, y:12};
    let copy_of_x = {
        let Point{ ref x, .. } = point;
        *x
    };
    println!("copy_of_x is {}", copy_of_x);

    let mut mut_point = point;
    {
        let Point{x:_, ref mut y} = mut_point;
        *y = 22;
    }
    println!("point is {:?}", point);
    println!("mut_point is {:?}", mut_point);

    let mut mut_tuple = (Box::new(42), 15);
    {
        let (_, ref mut y) = mut_tuple;
        *y = 16;
    }
    println!("mut_tuple is {:?}", mut_tuple);
}

fn print_refs<'a, 'b>(a: &'a i32, b: &'b i32) {
    println!("a is {}, b is {}", a, b);
}

fn failed_borrow<'a>() {
    let _x = 12;
    // let y: &'a i32 = &_x;
}

// fn main() {
fn main_explicit_lifetime() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    failed_borrow();
}

fn print_one<'a, T>(a:&'a T) where T: Debug {
    println!("print_one {:?}", a);
}

fn add_one<'a>(a:&'a  mut i32) {
    *a += 1
}

fn print_multi<'a, 'b>(a:&'a i32, b:&'b i32) {
    println!("a is {}, b is {}", a, b);
}

fn pass_x<'a, 'b>(a:&'a i32, _:&'b i32) -> &'a i32 {
    a
}

// fn main() {
fn main_explicit_lifetime_func() {
    let a = 32;
    print_one(&a);
    let c = 99;
    print_multi(&a, &c);

    let mut b = a;
    add_one(&mut b);
    print_one(&b);

    let z = pass_x(&a, &c);
    print_one(z);
}

struct Owner(i32);
impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1
    }
    fn print<'a>(&'a self) {
        println!("content is {}", self.0);
    }
}

// fn main() {
fn main_explicit_lifetime_method() {
    let mut o = Owner(22);
    o.add_one();
    o.print();
}

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NameBorrowed<'a> {
    x: &'a i32, y:&'a i32
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32)
}

// fn main() {
fn main_explicit_lifetime_struct() {
    let (x,y) = (5,8);
    let single = Borrowed(&x);
    let double = NameBorrowed{x:&x, y:&y};
    let reference = Either::Ref(&y);
    let num = Either::Num(x);
    println!("single is {:?}", single);
    println!("double is {:?}", double);
    println!("reference is {:?}", reference);
    println!("num is {:?}", num);
}

impl <'a> Default for Borrowed<'a> {
    fn default() -> Self {
        Self(&10)
    }
}

// fn main() {
fn main_explicit_lifetime_trait() {
    let b = Borrowed::default();
    println!("b is {:?}", b);
}

#[derive(Debug)]
struct Ref<'a, T: 'a>(&'a T);
fn print<T>(t: T) where T: Debug {
    println!("print t is {:?}", t);
}
fn print_ref<'a, T>(t: &'a T) where T:'a + Debug {
    print(t);
}

// fn main() {
fn main_multi_lifetime_constraint() {
    let x = 7;
    let ref_x = Ref(&x);
    print_ref(&ref_x);
    print(&ref_x);
}

fn multiply<'a>(a: &'a i32, b: &'a i32) -> i32 {
    a * b
}

fn choose_first<'a: 'b, 'b>(a: &'a i32, _: &'b i32) -> &'b i32 {
    a
}

// fn main() {
fn main_explicit_lifetime_force_conversion() {
    let first = 2;
    {
        let second = 3;
        println!("multiply is {}", multiply(&first, &second));
        println!("the first is {}", choose_first(&first, &second));
    }
}

fn coerce_static<'a>(_: &'a i32, n: &'static i32) -> &'a i32 {
    n
}

// fn main() {
fn main_static_lifetime() {
    static NUM: i32 = 22;
    {
        let static_string = "hhhdqwq";
        println!("static_string is {}", static_string);
    }
    {
        let lifetime_num = 2;
        let coerced_static = coerce_static(&lifetime_num, &NUM);
        println!("coerced_static is {}", coerced_static);
    }
    println!("NUM is {}", NUM);
}

struct Sheep {naked: bool, name:&'static str}
trait Animal {
    fn new(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;
    fn talk(&self) {
        println!("{} talks {}", &self.name(), &self.noise());
    }
}
impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep{naked: false, name}
    }

    fn name(&self) -> &'static str {
        &self.name
    }

    fn noise(&self) -> &'static str {
        "mieee"
    }

    fn talk(&self) {
        if self.is_naked() {
            println!("I am free");
        } else {
            println!("I want to be free");
        }
    }
}

impl Sheep {
    fn shear(&mut self) {
        if self.is_naked() {
            println!("no need to shear for {}", self.name());
        } else {
            println!("shear the sheep {}", self.name());
            self.naked = true;
        }
    }
    fn is_naked(&self) -> bool {
        self.naked
    }
}

// fn main() {
fn main_trait() {
    let mut sheep = Sheep::new("Dolly");
    sheep.talk();
    sheep.shear();
    sheep.talk();
    sheep.shear();
    sheep.talk();
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Centimeters(f64);
#[derive(Debug)]
struct Inches(i32);
impl Inches {
    fn to_centimeters(&self) -> Centimeters {
        let &Inches(num) = self;
        Centimeters(num as f64 * 2.54)
    }
}
struct Seconds(i32);

// fn main() {
fn main_trait_derivation() {
    let _one_second = Seconds(1);
    let foot = Inches(12);
    println!("one foot equals {:?}", foot);
    let meter = Centimeters(100.0);
    let cmp = {
        if foot.to_centimeters() < meter {
            "smaller"
        } else {
            "bigger"
        }
    };
    println!("{:?} is {} than {:?}", foot.to_centimeters(), cmp, meter);
}

struct Cow{name: &'static str}
impl Animal for Cow {
    fn new(name: &'static str) -> Self {
        Self{name}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        "mouuuuu"
    }
}

struct MySheep {}
struct MyCow {}

trait MyAnimal {
    // 实例方法签名
    fn noise(&self) -> &'static str;
}

// 实现 `Sheep` 的 `Animal` trait。
impl MyAnimal for MySheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// 实现 `Cow` 的 `Animal` trait。
impl MyAnimal for MyCow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// 返回一些实现 Animal 的结构体，但是在编译时我们不知道哪个结构体。
fn my_random_animal(random_number: f64) -> Box<dyn MyAnimal> {
    if random_number < 50.0 {
        Box::new(MySheep {})
    } else {
        Box::new(MyCow {})
    }
}

// fn main() {
fn main_trait_dyn() {
    let mut num = 29f64;
    let mut ani = my_random_animal(num);
    println!("{}", ani.noise());
    num = 99f64;
    ani = my_random_animal(num);
    println!("{}", ani.noise());
}

struct Fibonacci {
    cur: i32, next: i32
}

impl Iterator for Fibonacci {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.cur + self.next;
        self.cur = self.next;
        self.next = new_next;

        Some(self.cur)
    }
}

fn fibonacci() -> Fibonacci {
    Fibonacci{cur: 1, next: 1}
}

// fn main() {
fn main_iterator_trait() {
    let mut sequence = 1..3;
    println!("sequence next is {:?}", sequence.next());
    println!("sequence next is {:?}", sequence.next());
    println!("sequence next is {:?}", sequence.next());
    for i in 1..3 {
        println!("seq next is {}", i);
    }
    for i in fibonacci().take(4) {
        println!("fibonacci {}", i);
    }
    for i in fibonacci().take(4).skip(3) {
        println!("fibonacci skip {}", i);
    }
    let arr = [1,3,3,7];
    for i in arr.iter() {
        println!("arr i is {}", i);
    }
}

fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> std::iter::Cycle<std::iter::Chain<std::vec::IntoIter<i32>, std::vec::IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn combine_vecs_impl(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// fn main() {
fn main_impl_trait_as_return() {
    let v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    // let mut v3 = combine_vecs(v1, v2);
    let mut v3 = combine_vecs_impl(v1, v2);
    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());
    println!("all done");

    let plus_one = make_adder_func(1);
    println!("plus_one is {}", plus_one(5));

    let vec = vec![-2, -1, 0, 1, 2, 3, 4];
    for i in double_positives(&vec) {
        println!("double_positives {}", i);
    }

    let x = &&5;
    let b1 = x > &&0;
    let y = &&x;
    let b2 = **x > 0;
    let b3 = **y > &&0;
    println!("x is {}, b1 is {}, b2 is {}, b3 is {}", x, b1, b2, b3);
}

fn make_adder_func(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x| x + y;
    closure
}

fn double_positives<'a>(a: &'a Vec<i32>) -> impl Iterator<Item=i32> + 'a {
    a.iter().filter(|x| **x > 0).map(|x| x *2)
}

#[derive(Debug, Clone)]
struct BoxPair(Box<i32>, Box<i32>);

// fn main() {
fn main_trait_clone() {
    let nil = Nil;
    let copy_nil = nil;
    println!("nil is {:?}", nil);
    println!("copy_nil is {:?}", copy_nil);

    let box_pair = BoxPair(Box::new(5), Box::new(65));
    println!("box_pair is {:?}", box_pair);
    let moved_pair = box_pair;
    // println!("box_pair is {:?}", box_pair);
    println!("moved_pair is {:?}", moved_pair);
    let copy_box_pair = moved_pair.clone();
    drop(moved_pair);
    println!("copy_box_pair is {:?}", copy_box_pair);
}

trait APerson {
    fn name(&self) -> String;
}
trait Student:APerson {
    fn university(&self) -> String;
}
trait Programmer {
    fn fav_language(&self) -> String;
}
trait ComputeScienceStudent: Student + Programmer {
    fn git_username(&self) -> String;
}

fn greeting_computer_science_student(student: &dyn ComputeScienceStudent) {
    println!("welcome {} from university {}, your favorite language is {}, and your git username is {}",
    student.name(), student.university(), student.fav_language(), student.git_username());
}

struct OxfordCSStudent;

impl APerson for OxfordCSStudent {
    fn name(&self) -> String {
        "Howard".to_string()
    }
}

impl Student for OxfordCSStudent {
    fn university(&self) -> String {
        "Oxford".to_string()
    }
}

impl Programmer for OxfordCSStudent {
    fn fav_language(&self) -> String {
        "Rust".to_string()
    }
}

impl ComputeScienceStudent for OxfordCSStudent {
    fn git_username(&self) -> String {
        "oxford_cs_user_001".to_string()
    }
}

// fn main() {
fn main_super_trait() {
    let student = OxfordCSStudent;
    greeting_computer_science_student(&student);
}

trait UserNameWidget {
    fn get(&self) -> String;
}

trait AgeWidget {
    fn get(&self) -> u8;
}

struct UserForm {
    name: String,
    age: u8
}

impl UserNameWidget for UserForm {
    fn get(&self) -> String {
        self.name.clone()
    }
}

impl AgeWidget for UserForm {
    fn get(&self) -> u8 {
        self.age
    }
}

// fn main() {
fn main_trait_override_method_same_name() {
    let p = UserForm{name: "Alisa".to_string(), age: 8};
    let name = <UserForm as UserNameWidget>::get(&p);
    let age = <UserForm as UserNameWidget>::get(&p);
    println!("name is {}, age is {}", name, age);
}

macro_rules! say_hello {
    () => {
        println!("hello");
    }
}

// fn main() {
fn main_macro() {
    say_hello!();
}

macro_rules! create_function {
    ($func_name: ident) => (
        fn $func_name() {
            println!("You called {} function", stringify!($func_name));
        }
    )
}

create_function!(foo);
create_function!(bar);

macro_rules! print_results {
    ($expression: expr) => (
        println!("{} is {}", stringify!($expression), $expression);
    )
}

// fn main() {
fn main_macro_indicator() {
    foo();
    bar();

    print_results!(1 + 2);
    print_results!({ 1 + 2});
    print_results!({ let a = 2; a + 9});
    print_results!({
        match 1 + 9 {
            5 => "smaller",
            10 => "right",
            101 => "bigger",
            _ => "default"
        }
    });
}

macro_rules! test {
    ($left:expr; and $right:expr) => (
        println!("left is {:?} AND right is {:?}, res is {:?}", stringify!($left), stringify!($right), $left && $right);
    );
    ($left:expr; or $right:expr) => (
        println!("left is {:?} OR right is {:?}, res is {:?}", stringify!($left), stringify!($right), $left || $right);
    );
}

// fn main() {
fn main_macro_override() {
    test!( 1 + 2 == 3; and 2 + 3 == 4);
    test!( 1 + 2 == 3; or 2 + 3 == 4);
}

macro_rules! find_main {
    ($x:expr) => (
        $x
    );
    ($x:expr, $($y:expr),+) => (
        std::cmp::min($x, find_main!($($y),+))
    );
}

// fn main() {
fn main_macro_recursion() {
    println!("{}", find_main!(3));
    println!("{}", find_main!(3, 0 + 1));
    println!("{}", find_main!(3, 2));
    println!("{}", find_main!(3, 2, 0 - 1));
}

macro_rules! calculate {
    (eval $e:expr) => {{
        {
            let val: usize = $e;
            println!("{} = {}", stringify!($e), val);
        }
    }};
    (eval $e:expr, $(eval $y: expr),+) => {{
       calculate!(eval $e);
        calculate!($(eval $y),+);
    }};
}

// fn main() {
fn main_macro_dsl() {
    calculate! {
      eval 1 + 2
    };
    calculate! {
      eval 1 + 2 * (8 / 4)
    };
    calculate! {
      eval 1 + 2 * (8 / 4),
        eval 2 * 66,
        eval 3 / 1 + 7
    };
}

fn give_princess(s: &str) {
    if s == "snake" {panic!("AAAAAAAA!!")}
    println!("I love {}s", s);
}

// fn main() {
fn main_panic() {
    give_princess("apple");
    give_princess("banana");
    give_princess("snake");
}

fn give_commoner(gift: Option<&str>) {
    match gift {
        Some("snake") => println!("I don't fear snakes"),
        Some(xx) => println!("Well, I get {}", xx),
        None => println!("OK")
    }
}

fn give_princess_option(gift: Option<&str>) {
    let inside = gift.unwrap();
    if inside == "snake" { panic!("AAAAAAAAAAA!!")}
    println!("The princess likes {}s", inside);
}

// fn main() {
fn main_option_unwrap() {
    let apple = Some("apple");
    let snake = Some("snake");
    let none = Option::None;
    give_commoner(apple);
    give_commoner(snake);
    give_commoner(none);
    give_princess_option(apple);
    give_princess_option(none);
    give_princess_option(snake);
}

fn next_birthday(age: Option<u8>) -> Option<String> {
    let ma = age?;
    Some(format!("Next year, I will be {}", ma))
}

struct JobPerson {
    job: Option<Job>
}

#[derive(Copy, Clone)]
struct Job {
    phone_number: Option<PhoneNumber>
}

#[derive(Copy, Clone)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32
}

impl JobPerson {
    fn get_area_code(&self) -> Option<u8> {
        self.job?.phone_number?.area_code
    }
}

// fn main() {
fn main_option_questionmark() {
    let p = JobPerson{job: Some(Job{
        phone_number: Some(PhoneNumber {area_code: Some(21u8), number: 22223333u32})
    })
    };
    println!("area code is {:?}", p.get_area_code().unwrap());
}

#[derive(Debug)] enum Food {Apple, Carrot, Potato}
#[derive(Debug)] struct Peeled(Food);
#[derive(Debug)] struct Chopped(Food);
#[derive(Debug)] struct Cooked(Food);

fn peel(food: Option<Food>) -> Option<Peeled> {
    match food {
        Some(t) => Some(Peeled(t)),
        None => None
    }
}

fn chop(food: Option<Peeled>) -> Option<Chopped> {
    match food {
        Some(Peeled(t)) => Some(Chopped(t)),
        None => None
    }
}

fn cook(food: Option<Chopped>) -> Option<Cooked> {
    food.map(|Chopped(t)|Cooked(t))
}

fn process(food: Option<Food>) -> Option<Cooked> {
    food.map(|f| Peeled(f))
        .map(|Peeled(f)|Chopped(f))
        .map(|Chopped(f)|Cooked(f))
}

fn eat(food: Option<Cooked>) {
    match food {
        Some(Cooked(f)) => println!("I like cooked {:?}", f),
        None => println!("give me some food")
    }
}

// fn main() {
fn main_option_map() {
    let cooked_apple = cook(chop(peel(Some(Food::Apple))));
    let cooked_carrot = cook(chop(peel(Some(Food::Carrot))));
    let cooked_potato = process(Some(Food::Potato));
    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
    eat(process(None));
}

fn multiply_str(a: &str, b: &str) -> i32 {
    let ia = a.parse::<i32>().unwrap();
    let ib = b.parse::<i32>().unwrap();
    ia * ib
}

// fn main() {
fn main_result() {
    let m1 = multiply_str("21", "2");
    println!("m1 is {}", m1);
    let m1 = multiply_str("tt", "2");
    println!("m1 is {}", m1);
}

fn multiply_str_map(a: &str, b: &str) -> Result<i32, ParseIntError> {
    a.parse::<i32>().and_then(|a_parsed|{
        b.parse::<i32>().map(|b_parsed| a_parsed * b_parsed)
    })
}

fn print_result_i32(res: Result<i32, ParseIntError>) {
    match res {
        Ok(a) => {println!("print_result_i32 res is {}", a)}
        Err(_) => {println!("print_result_i32 err")}
    }
}

// fn main() {
fn main_result_map() {
    let m1 = multiply_str_map("22", "2");
    print_result_i32(m1);
    let m1 = multiply_str_map("t", "2");
    print_result_i32(m1);
}

type AliasedResult<T> = Result<T, ParseIntError>;

fn multiply_str_map_alias(a: &str, b: &str) -> AliasedResult<i32> {
    a.parse::<i32>().and_then(|a_parsed|{
        b.parse::<i32>().map(|b_parsed| a_parsed * b_parsed)
    })
}

fn print_result_i32_alias(res: AliasedResult<i32>) {
    match res {
        Ok(a) => {println!("print_result_i32_alias res is {}", a)}
        Err(_) => {println!("print_result_i32_alias err")}
    }
}

// fn main() {
fn main_result_map_alias() {
    print_result_i32_alias(multiply_str_map_alias("22", "2"));
    print_result_i32_alias(multiply_str_map_alias("tt", "2"));
}

fn multiply_str_return_advanced(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let a_parsed = match a.parse::<i32>() {
        Ok(p) => {p}
        Err(e) => {return Err(e)}
    };
    let b_parsed = match b.parse::<i32>() {
        Ok(p) => {p}
        Err(e) => {return Err(e)}
    };
    Ok(a_parsed * b_parsed)
}

// fn main() {
fn main_result_advanced_return() {
    print_result_i32(multiply_str_return_advanced("22", "2"));
    print_result_i32(multiply_str_return_advanced("tt", "2"));
}

fn multiply_result_question_mark(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let a_parsed = a.parse::<i32>()?;
    let b_parsed = b.parse::<i32>()?;
    Ok(a_parsed * b_parsed)
}

// fn main() {
fn main_result_question_mark() {
    print_result_i32(multiply_result_question_mark("22","2"));
    print_result_i32(multiply_result_question_mark("tt","2"));
}

fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap();
    2 * first.parse::<i32>().unwrap()
}

// fn main() {
fn main_result_err_multi() {
    let vec = vec!["22", "33", "44"];
    double_first(vec);

    let strings = vec!["tt", "11", "12"];
    double_first(strings);

    let empty = vec![];
    double_first(empty);
}

fn double_first_contains(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first|{
        first.parse::<i32>().map(|n| 2 * n)
    })
}

// fn main() {
fn main_result_multi_contains() {
    let vec = vec!["22", "33", "44"];
    println!("double_first_contains vec is {:?}", double_first_contains(vec));

    let strings = vec!["tt", "11", "12"];
    println!("double_first_contains vec is {:?}", double_first_contains(strings));

    let empty = vec![];
    println!("double_first_contains vec is {:?}", double_first_contains(empty));
}

fn double_first_result_multi_switch(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first|{
       first.parse::<i32>().map(|n| 2 * n)
    });
    opt.map_or(Ok(None), |r| r.map(Some))
}

// fn main() {
fn main_result_multi_switch() {
    let vec = vec!["22", "33", "44"];
    println!("double_first_result_multi_switch vec is {:?}", double_first_result_multi_switch(vec));

    let strings = vec!["tt", "11", "12"];
    println!("double_first_result_multi_switch vec is {:?}", double_first_result_multi_switch(strings));

    let empty = vec![];
    println!("double_first_result_multi_switch vec is {:?}", double_first_result_multi_switch(empty));
}

#[derive(Debug, Clone)]
struct DoubleError;

type DoubleErrResult<T> = std::result::Result<T, DoubleError>;

impl std::fmt::Display for DoubleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl std::error::Error for DoubleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

fn double_first_customized_error(vec: Vec<&str>) -> DoubleErrResult<i32> {
    vec.first().ok_or(DoubleError)
        .and_then(|s|{
            s.parse::<i32>().map_err(|_| DoubleError).map(|a| 2 * a)
        })
}

fn print_double_error_result(result: DoubleErrResult<i32>) {
    match result {
        Ok(a) => {println!("print_double_error_result doubled is {}", a)}
        Err(e) => {println!("print_double_error_result err {}", e)}
    }
}

// fn main() {
fn main_result_customized_error() {
    let vec = vec!["22", "33", "44"];
    print_double_error_result(double_first_customized_error(vec));

    let strings = vec!["tt", "11", "12"];
    print_double_error_result(double_first_customized_error(strings));

    let empty = vec![];
    print_double_error_result(double_first_customized_error(empty));
}

#[derive(Debug,Clone)]
struct EmptyVec;

type EmptyVecResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

impl std::fmt::Display for EmptyVec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl std::error::Error for EmptyVec {
    fn description(&self) -> &str {
        "invalid first item to double"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        None
    }
}

fn double_first_box(vec: Vec<&str>) -> EmptyVecResult<i32> {
    vec.first().ok_or_else(|| EmptyVec.into())
        .and_then(|s|{
            s.parse::<i32>().map_err(|e| e.into()).map(|s| 2 * s)
        })
}

fn print_result_empty_vec(result: EmptyVecResult<i32>) {
    match result {
        Ok(n) => {println!("print_result_empty_vec first doubled is {}", n)}
        Err(e) => {println!("print_result_empty_vec error {}", e)}
    }
}

// fn main() {
fn main_result_box_error() {
    let vec = vec!["22", "33", "44"];
    print_result_empty_vec(double_first_box(vec));

    let strings = vec!["tt", "11", "12"];
    print_result_empty_vec(double_first_box(strings));

    let empty = vec![];
    print_result_empty_vec(double_first_box(empty));
}

// fn main() {
fn main_iterate_result() {
    let strings = vec!["tt", "22", "33"];
    let numbers: Vec<_> = strings.into_iter().map(|s| s.parse::<i32>()).filter_map(Result::ok).collect();
    println!("results: {:?}", numbers);
}

// fn main() {
fn main_iterate_result_partition() {
    let strings = vec!["tt", "22", "33"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings.into_iter().map(|s|s.parse::<i32>()).partition(Result::is_ok);
    println!("numbers: {:?}", numbers);
    println!("errors: {:?}", errors);
    let numbers: Vec<_> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    println!("numbers: {:?}", numbers);
    println!("errors: {:?}", errors);
}

#[derive(Debug,Copy, Clone)]
struct PointF64 {
    x: f64,
    y: f64
}

struct RectangleF64 {
    p1: PointF64, p2: PointF64
}

fn origin() -> PointF64 {
    PointF64 {x: 0f64, y : 0f64}
}

fn boxed_origin() -> Box<PointF64> {
    Box::new(PointF64{x: 0.0, y: 0.0})
}

// fn main() {
fn main_box() {
    let point = origin();
    let rectangle = RectangleF64 {p1: origin(), p2: PointF64{x: 3.0, y: 4.0}};
    let boxed_rectangle = Box::new(RectangleF64{p1: origin(), p2: origin()});
    let boxed_point = Box::new(origin());
    let box_in_a_box = Box::new(boxed_origin());

    println!("point_f64 occupies {} bytes in the stack", std::mem::size_of_val(&point));
    println!("rectangle_f64 occupies {} bytes in the stack", std::mem::size_of_val(&rectangle));
    println!("boxed_point occupies {} bytes in the stack", std::mem::size_of_val(&boxed_point));
    println!("boxed_rectangle occupies {} bytes in the stack", std::mem::size_of_val(&boxed_rectangle));
    println!("box_in_a_box occupies {} bytes in the stack", std::mem::size_of_val(&box_in_a_box));

    let unboxed_point = *boxed_point;
    println!("unboxed_point occupies {} bytes in the stack", std::mem::size_of_val(&unboxed_point));
}

// fn main() {
fn main_vec() {
    let collected_vec: Vec<i32> = (0..10).collect();
    println!("collected_vec is {:?}", collected_vec);

    let mut xs = vec![1,2,3];
    println!("vector is {:?}", xs);

    println!("push to vector");
    xs.push(22);
    println!("vector is {:?}", xs);

    // collected_vec.push(0);
    println!("vector size is {}", xs.len());
    println!("second element is {}", xs[1]);
    println!("pop last element {:?}", xs.pop());
    // println!("fourth element is {}", xs[3]);
    println!("content of vector: ");
    for x in xs.iter() {
        println!(" > {}", x);
    }
    for (i, x) in xs.iter().enumerate() {
        println!("in position {} we have {}", i, x);
    }
    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("updated vector is {:?}", xs);
}

// fn main() {
fn main_string() {
    let pangram = "the quick brown fox jumps over the lazy dog";
    println!("pangram: {}", pangram);
    println!("pangram reverse:");
    for i in pangram.split_whitespace().rev() {
        println!(" > {}", i);
    }
    let mut chars: Vec<char> = pangram.chars().collect();
    chars.sort();
    chars.dedup();
    let mut string = String::new();
    for c in chars {
        string.push(c);
        string.push_str(",");
    }
    let chars_to_trim: &[char] = &[' ', ','];
    let trimmed_str = string.trim_matches(chars_to_trim);
    println!("used characters: {}", trimmed_str);

    let alice = String::from("I like dogs");
    let bob = alice.replace("dogs", "cats");
    println!("Alice says {}", alice);
    println!("Bob says {}", bob);
}

#[derive(PartialEq,Eq,Hash)]
struct Account<'a> {
    username: &'a str,
    password: &'a str
}

struct AccountInfo<'a> {
    name: &'a str,
    email: &'a str
}

type Accounts<'a> = HashMap<Account<'a>, AccountInfo<'a>>;

fn try_login<'a>(accounts: &Accounts<'a>, username: &'a str, password: &'a str) {
    println!("username: {}", username);
    println!("password: {}", password);
    println!("try login");
    let login = Account{username: username, password: password};
    match accounts.get(&login) {
        None => { println!("login failed") }
        Some(a) => { println!("success login"); println!("name: {}, email: {}", a.name, a.email)}
    }
}

// fn main() {
fn main_hashmap() {
    let mut accounts: Accounts = HashMap::new();
    let account = Account{username: "alisa", password: "pwd123"};
    let account_info = AccountInfo{name: "Alisa Wu", email: "alisa@kda.io"};
    accounts.insert(account, account_info);
    try_login(&accounts, "alisa", "pwdpwd");
    try_login(&accounts, "alisa", "pwd123");
}

// fn main() {
fn main_hashset() {
    let mut a:HashSet<i32> = vec!(1,2,3).into_iter().collect();
    let mut b:HashSet<i32> = vec!(2,3,4).into_iter().collect();
    assert!(a.insert(4));
    assert!(a.contains(&4));
    // assert!(b.insert(4), "value is already in set");
    b.insert(5);
    println!("A: {:?}", a);
    println!("B: {:?}", b);
    println!("union: {:?}", a.union(&b).collect::<Vec<&i32>>());
    println!("difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());
    println!("difference b: {:?}", b.difference(&a).collect::<Vec<&i32>>());
    println!("intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());
    println!("symmetric_difference: {:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
}

// fn main() {
fn main_spawn() {
    let mut children = vec![];
    for i in 0..10 {
        children.push(std::thread::spawn(move || {
            println!("child thread {}", i);
        }));
    }
    for child in children {
        let _ = child.join();
    }
}

// fn main() {
fn main_map_reduce() {
    let data = "86967897737416471853297327050364959
11861322575564723963297542624962850
70856234701860851907960690014725639
38397966707106094172783238747669219
52380795257888236525459303330302837
58495327135744041048897885734297812
69920216438980873548808413720956532
16278424637452589860345374828574668";
    let mut children = vec![];
    let chunked_data = data.split_whitespace();
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is {}", i, data_segment);
        children.push(std::thread::spawn(move || -> u32 {
            let result = data_segment.chars().map(|a| a.to_digit(10).expect("should be a digit"))
                .sum();
            println!("segment result for {} is {}", i, result);
            result
        }))
    }
    let mut intermediate_sums = vec![];
    for child in children {
        intermediate_sums.push(child.join().unwrap());
    }
    let final_result = intermediate_sums.iter().sum::<u32>();
    println!("final_result is {}", final_result);
}

// fn main() {
fn main_thread_channel() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = std::sync::mpsc::channel();
    for id in 0..3 {
        let thread_tx = tx.clone();
        std::thread::spawn(move || {
           thread_tx.send(id).unwrap();
            println!("thread {} finished", id);
        });
    }
    let mut ids = Vec::with_capacity(3);
    for _ in 0..3 {
        ids.push(rx.recv());
    }

    println!("{:?}", ids);
}

// fn main() {
fn main_path() {
    let a = Path::new(".");
    // let display = a.display();
    let new_path = a.join("a").join("b");
    match new_path.to_str() {
        None => {println!("new path is not valid chars")}
        Some(a) => {println!("new path is {}", a)}
    }
}


// fn main() {
fn main_open_file() {
    let path = Path::new("d:/cap.txt");
    let display = path.display();
    let mut file = match File::open(path) {
        Ok(a) => {a}
        Err(e) => {panic!("could not open file {:?} reason {:?}", display, e.to_string())}
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Ok(_) => {println!("{:?} contains {}", display, s)}
        Err(e) => {panic!("could not read file {:?} reason {:?}", display, e.to_string())}
    }
}

// fn main() {
fn main_args() {
    let args: Vec<String> = std::env::args().collect();
    println!("my path is {}", args[0]);
    println!("args len {} and contains {:?}", args.len() - 1, &args[1..]);
}

pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
    let mut i = 0;
    let mut j = 0;
    let mut r = -100000 - 100;
    let mut si = 0;
    let mut sj = 0;
    let mut sum = 0;
    for va in matrix.iter() {
        for v in va.iter() {
            si = i;
            sj = j;
            println!("now iter si={} sj={} ", si, sj);
            while si < matrix.len() {
                sum = 0;
                while sj < va.len() {
                    let m = matrix[si][sj];
                    println!("suming si={} sj={} m={}", si, sj, m);
                    sum += m;
                    if sum == k {
                        println!("sum is k");
                        return sum
                    }

                    if sum < k && sum > r {
                        r = sum;
                        println!("update r {}", r);
                    }

                    sj += 1;
                }
                si += 1;
            }


            j += 1;
        }
        i += 1;
    }
    r
}

fn main() {
    // let v1 = vec![1, 0, 1];
    // let v2 = vec![0, -2, 3];
    // let v = vec![v1, v2];
    // println!("largest rectangle sum is {}", max_sum_submatrix(v, 2));

    // let v1 = vec![2, 2, -1];
    // let v = vec![v1];
    // println!("largest rectangle sum is {}", max_sum_submatrix(v, 2));

    let v1 = vec![5,  -4, -3, 4];
    let v2 = vec![-3, -4, 4,  5];
    let v3 = vec![5,  1,  5, -4];
    let v = vec![v1, v2, v3];
    println!("largest rectangle sum is {}", max_sum_submatrix(v, 10));
}

