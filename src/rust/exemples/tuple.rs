fn main() {
    // tag::tuple[]
    let tuple = ("hello", 5, 'c'); //<1>

    assert_eq!(tuple.0, "hello"); //<2>
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');

    let (x, y, z) = tuple; //<3>
    assert_eq!(z, 'c');
    // end::tuple[]

    // tag::tuplestruct[]
    struct ColorRGB(u8, u8, u8);
    let color = ColorRGB(255, 0, 0);
    
    // On peut utiliser .0 .1 pour nommer les champs
    println!("red channel = {}", color.0);
    
    // On peut aussi déconstruire la structure avec du pattern matching
    let ColorRGB(r, g, b) = color;
    // end::tuplestruct[]
}
