fn main() {
    // tag::array[]
    let mut array: [i32; 3] = [0; 3];
    array[1] = 1;
    array[2] = 2;

    assert_eq!([1, 2], &array[1..]);

    for x in array {
        print!("{} ", x);
    }
    // end::array[]
}
