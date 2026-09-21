#[derive(Debug,PartialEq)]
struct Complex {
    real: f64,
    im: f64,
}

fn sum(vs: &Vec<Complex>) -> Complex {
    let mut res = Complex {real:0.0, im: 0.0};
    for e in vs {
        res.real += e.real;
        res.im += e.im;
    }
    res
}

fn main() {
    let v = vec!(
        Complex{real:1.0, im:1.0}, Complex{real:2.0, im:2.0},
        Complex{real:3.0, im:3.0}, Complex{real:4.0, im:4.0},
        Complex{real:5.0, im:5.0});

    assert_eq!(Complex{real:15.0, im:15.0}, sum(&v));
}
