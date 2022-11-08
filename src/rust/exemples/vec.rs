fn main() {
    // tag::create[]
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    
    let w: Vec<i32> = vec![5, 6, 7]; //macro équivalent
    // end::create[]

    // tag::access[]
    let mut v = vec![1, 2, 3, 4, 5];

    let x: &i32 = &v[2]; // panique si l'index n'existe pas

    match v.get(2) {
        Some(y) => println!("v[3] == {}", y),
        None => println!("index non-existant")
    }

    v[2] = 42; // car le vecteur v est mutable
    // end::access[]

    // tag::init[]
    let n : usize = 100; //<1>
    let zero_v = vec![0; n];
    let one_v = vec![1; n];
    // end::init[]

    // tag::iter[]
    let mut v = vec![1, 2, 3, 4, 5];

    // itération non-mutable, i est de type &i32
    for i in &v {
        println!("{}", i);
    }
    
    // itération mutable, i est de type &mut i32
    // on peut incrémenter chaque élément du vecteur
    for i in &mut v {
        *i += 1;
    }
    // end::iter[]

    // tag::iterv[]
    let mut v = vec![1, 2, 3, 4, 5];

    // itération directement sur le vecteur les éléments du vecteur
    // sont « déplacés » dans i. v ne pourra plus être utilisé par la
    // suite.
    for i in v {
        println!("{}", i);
    }
    
    //v[0] = 15; //<1>
    // end::iterv[]
    
}
