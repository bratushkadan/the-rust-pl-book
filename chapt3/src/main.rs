fn _tuple_indexing() {
    let tup: (i32, f64, u8) = (-257, 0.1384325, 25);

    let floating_num = tup.1;

    println!("{}", floating_num);
}

// Loops can be expressions
fn _loop_expr() {
    let mut counter = 0;
    let _result = loop {
        counter += 1;

        if counter == 10 {
            // breaks from the loop with the value counter * 2
            break counter * 2;
        };
    };
}

fn _if_as_an_expression() {
    let condition = true;
    let _number = if condition {
        5
    } else {
        6
    };
}

fn _expression() -> i8 {
    // This is an expression in the curly braces {}:
    let y = {
        let x: i8 = 3;
        // a line without a semicolon at the end is expression
        x + 1
    };

    y
}

// Arrays are very effective if one wants a fixed sized collection
// that is allocated on the stack
fn _use_array(arr: [u8; 8]) {
    for el in arr.iter() {
        println!("element: {}", el);
    }
}

fn _counting_with_for_loop() {
    for num in 1..=4 {
        println!("(o) num: {}", num);
    }
    for num in (1..=4).rev() {
        println!("(r) num: {}", num);
    }
    for num in 0..10 {
        println!("(n) num: {}", num);
    }
}


fn main() {
}
