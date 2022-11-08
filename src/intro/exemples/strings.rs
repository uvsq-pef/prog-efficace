fn main() {
    // tag::str[]
    let hello = "Hello, world!"; // <1>
    assert!(!hello.is_empty());
    assert_eq!(13, hello.len());

    assert_eq!("Hello", &hello[0..5]); // <2>
    assert_eq!(Some("world"), hello.get(7..12));

    println!("{}", hello); // <3>
    // end::str[]

    // tag::string[]
    let hello = "Hello".to_string(); // <1>
    assert!(!hello.is_empty());
    assert_eq!(5, hello.len());

    let mut world = String::from("world");
    assert_eq!(5, world.len());

    let msg = hello + &world;  // <2>
    assert_eq!("Helloworld", msg);

    world.insert_str(0, "Hello");
    assert_eq!("Helloworld", world);

    world.insert_str(5, ", ");
    assert_eq!("Hello, world", world);

    world.push_str(" !");
    assert_eq!("Hello, world !", world);
    // end::string[]
}
