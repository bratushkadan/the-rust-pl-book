pub struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // The lifetime parameter declaration after impl
    // and its use after the type name are required,
    // but because of the first elision rule, we’re not required
    // to annotate the lifetime of the reference to self.
    fn level(&self) -> i32 {
        3
    }

    // There are two input lifetimes, so Rust applies
    // the first lifetime elision rule and gives both &self
    // and announcement their own lifetimes. Then,
    // because one of the parameters is &self,
    // the return type gets the lifetime of &self,
    // and all lifetimes have been accounted for.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        _ = announcement;
        self.part
    }
}

// Generic Type Parameters, Trait Bounds, and Lifetimes

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        return x;
    }
    return y;
}

pub fn foo() {
    let string1 = String::from("abcd");

    let result;
    {
        let string2 = "ecdf";
        result = longest(string1.as_str(), string2);
    }
    println!("Самая длинная строка равна {}", result);

    let novel = String::from("Hello. World.");
    let first_sentence = novel.split(".").next().expect("Couldn't find '.'");

    _ = ImportantExcerpt {
        part: first_sentence,
    };

    // The Static Lifetime specifier 'static
    // denotes that the affected reference can
    // live for the entire duration of the program
    let s: &'static str = "I have a static lifetime.";
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if y.len() > x.len() {
        return y;
    }
    x
}
