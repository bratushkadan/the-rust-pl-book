pub mod foo {
    pub fn bar() -> String {
        String::new()
    }
}

fn foo() -> String {
    String::from("foo")
}
fn bar() -> String {
    String::from("bar")
}
fn baz() -> String {
    String::from("baz")
}
fn qux() -> String {
    String::from("qux")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Using Result<T, E> in Tests
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn tfoo() {
        assert_eq!(foo(), "foo");
    }

    #[test]
    fn tbar() {
        assert_ne!(bar(), "foo");
        assert_eq!(bar(), "bar");
    }

    #[test]
    fn tbaz() {
        assert_eq!(baz(), "baz");
    }

    #[test]
    fn tqux() {
        assert_eq!(
            qux(),
            "qux",
            "Resultant phrase did not contain \"qux\", got \"{}\" instead.",
            qux()
        );
    }

    #[test]
    #[should_panic]
    fn panicking() {
        (|| panic!("wtf"))();
    }

    #[test]
    // checks for a panic with exact substring in its message
    #[should_panic(expected = "roses are red")]
    fn panicking2() {
        (|| panic!("roses are red violets are blue"))();
    }
}
