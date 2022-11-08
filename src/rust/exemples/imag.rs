#[derive(Debug,PartialEq)]
struct Imaginary {
    real: f64,
    im: f64,
}

fn sum(vs: &Vec<Imaginary>) -> Imaginary {
    let mut res = Imaginary {real:0.0, im: 0.0};
    for e in vs {
        res.real += e.real;
        res.im += e.im;
    }
    res
}

fn main() {
    let v = vec!(
        Imaginary{real:1.0, im:1.0}, Imaginary{real:2.0, im:2.0},
        Imaginary{real:3.0, im:3.0}, Imaginary{real:4.0, im:4.0},
        Imaginary{real:5.0, im:5.0});

    assert_eq!(Imaginary{real:15.0, im:15.0}, sum(&v));
}
