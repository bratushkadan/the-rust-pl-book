pub fn run() {
    let s1 = String::from("hello");
    let s2 = String::from("world");
    let s = s1 + " " + &s2;

    println!("s2 = {}, s = {}", s2, s);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");

    let s = format!("{}-{}-{}", tic, tac, toe);

    println!("{}", s);

    // // thread 'main' (187173310) panicked at src/strings.rs:17:19:
    // // byte index 3 is not a char boundary; it is inside 'р' (bytes 2..4) of `привет`
    // // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    // let hello = "привет";
    // let s = &hello[0..3];
    // _ = s;

    // returns unicode code points
    // NOTE: be sure to remember that valid Unicode scalar values may be made up of more than one byte.
    for c in "привет".chars() {
      _ = c
    }

    for b in "привет".bytes() {
      _ = b
    }
}