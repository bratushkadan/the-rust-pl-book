mod hashmaps;
mod strings;
mod tasks;
mod vecs;

fn main() {
    // vecs::run();
    // strings::run();
    // hashmaps::run();
    let vec = vec![1, 2, 4, 5, 5, 6];
    let props = tasks::task1(&vec);
    match props {
        Some(v) => println!("vec \"{:?}\" has the following properties: {:#?}", vec, v),
        None => (),
    }
}
