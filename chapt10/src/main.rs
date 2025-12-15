use std::clone::Clone;
use std::cmp::PartialOrd;
use std::marker::Copy;

mod lifetimes;

fn main() {
    

    lifetimes::foo();
}

// Trait function parameter

pub trait Summable<T> {
    fn sum(&self) -> T;
}

pub trait Mul<T> {
    fn mul(&self) -> T;
}

pub trait Diff<T> {
    fn diff(&self) -> T;
}

pub fn sum_all<T>(s: &impl Summable<T>) -> T {
    s.sum()
}

// the same as above with the "trait bound"
pub fn sum_all_foo<P, T: Summable<P>>(s: &T) -> P {
    s.sum()
}

// trait bound is the most useful for enforcing the same trait in both parameters of a function
pub fn sum_two<P, T: Summable<P>>(one: &T, two: &T) -> P {
    _ = &one;
    two.sum()
}

// Multiple trait bounds
pub fn do_math<T>(num: &(impl Summable<T> + Mul<T>)) -> T {
    num.mul()
}
pub fn do_math_2<T, P: Summable<T> + Mul<T>>(num: &P) -> T {
    num.mul()
}

// Clearer Trait Bounds with where Clauses
fn some_function<T, U, P>(t: &T, u: &U) -> P
where
    T: Summable<P> + Mul<P>,
    U: Mul<P> + Diff<P>,
{
    _ = u;
    t.sum()
}

// same as above
fn some_function2<P, T: Summable<P> + Mul<P>, U: Mul<P> + Diff<P>>(t: &T, u: &U) -> P {
    _ = u;
    t.sum()
}

// Returns trait
fn returns_summarizable<T>() -> impl Summary {
    Tweet {
        username: String::from("dan"),
        content: String::from("foobar"),
    }
}

// Trais Copy and Clone

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &num in list.iter() {
        if num > largest {
            largest = num;
        }
    }
    largest
}

fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for num in list.iter() {
        let num = num.clone();
        if num > largest {
            largest = num;
        }
    }
    largest
}

// Conditionally Implement Methods
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    // Pair of any type can be created
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Methods in the following implementation can only be used on pairs of
// trait bounds Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x)
        } else {
            println!("The largest member is y = {}", self.y)
        }
    }
}

// Blanket Implementation
// impl<T> Trait for T
// https://stackoverflow.com/questions/73861891/what-are-blanket-implementations-in-rust


trait CopyClone: Copy + Clone {}

impl<T> CopyClone for T
where
    T: Copy + Clone + 'static
{
    
}

// another example of Blanket Implementation
trait Collection {
    fn size(&self) -> usize;
}
impl<T> Collection for [T] {
    fn size(&self) -> usize {
        self.len()
    }
}

// Traits and implementations
pub trait Summary {
    fn summarize(&self) -> String;
    fn default_implementation(&self) -> i32 {
        _ = self.summarize();
        0
    }
}

pub struct Tweet {
    username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", &self.username, &self.content)
    }
}

// Generics

pub struct Point<T> {
    x: T,
    y: T,
}

pub struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    // generics in a method
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x: x, y: y }
    }
    pub fn x(&self) -> &T {
        return &self.x;
    }
    pub fn y(&self) -> &T {
        return &self.y;
    }
}

// Implement a struct of a concrete type
impl Point<f64> {
    pub fn xv(&self) -> f64 {
        return self.x;
    }
    pub fn yv(&self) -> f64 {
        return self.y;
    }
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    // &item iterates over i32, whereas item iterates over &i32
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
