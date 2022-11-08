// tag::another_function[]
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
// end::another_function[]

// tag::five[]
fn five() -> i32 { //<1>
    5
}
// end::five[]

fn main() {
    another_function(12);

    assert_eq!(5, five());
    println!("Value : {}", five());
}
