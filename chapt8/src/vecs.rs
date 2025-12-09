pub fn run() {
    let mut vec = Vec::from_iter(1..5);

    // iterate over a vector in an immutable manner
    for v in &vec {
        let _v = v + 1;
    }

    print_vec_vals(&vec);

    // iterate over a vector in a mutable manner
    for v in &mut vec {
        *v += 1
    }

    print_vec_vals(&vec);

    increase_vec_vals(&mut vec, 1);

    print_vec_vals(&vec);
}

fn print_vec_vals(vec: &Vec<i32>) {
    for v in vec {
        println!("el = {}", v);
    }
    println!("---");
}

fn increase_vec_vals(vec: &mut Vec<i32>, by: i32) {
    for v in vec {
        *v += by
    }
}