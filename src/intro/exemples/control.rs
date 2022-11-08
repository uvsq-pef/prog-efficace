fn main() {
    // tag::if[]
    let celsius = true; // false -> Fahrenheit

    let freeze_temp = if celsius { 0 } else { 32 }; //<1>
    assert_eq!(0, freeze_temp);

    let temperature = 12;
    assert!(temperature > freeze_temp);

    if temperature < freeze_temp { //<2>
        println!("Freezing");
    } else {
        println!("Not freezing");
    }
    // end::if[]

    // tag::match[]
    let message = match temperature {
        -20 ..= -1 => "Froid", //<1>
        0 => "Limite", //<2>
        1 ..= 10 => "Frais",
        11 ..= 25 => "Idéal",
        _ => "Insupportable", //<3>
    };
    assert_eq!("Idéal", message);
    // end::match[]

    // tag::loop[]
    let mut counter = 0;
    let result = loop { //<1>
        counter += 1;
        if counter == 10 {
            break counter * 2; //<2>
        }
    };
    assert_eq!(20, result);
    // end::loop[]

    // tag::while[]
    let mut number = 5;
    while number != 0 { //<1>
        println!("{}", number);
        number -= 1;
    }
    assert_eq!(0, number);
    // end::while[]

    // tag::for[]
    let elements = [10, 20, 30, 40, 50];

    for element in elements.iter() { //<1>
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() { //<2>
        println!("{}!", number);
    }
    // end::for[]
}
