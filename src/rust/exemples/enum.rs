// tag::enum[]
enum Variant { //<1>
    Rien,
    Nombre(i32),
    Texte(String),
}
// end::enum[]

// tag::match[]
fn affiche(v: Variant) {
    match v {
        Variant::Rien => println!("Le variant est vide"),
        Variant::Nombre(n) => println!("Le variant contient le nombre {}", n),
        Variant::Texte(s) => println!("Le variant contient le texte {}", s),
    }
}
// end::match[]

fn main() {
    // tag::enum[]
    let v1: Variant = Variant::Rien;
    let v2: Variant = Variant::Nombre(42);
    let v3: Variant = Variant::Texte(String::from("Hello"));
    // end::enum[]

    // tag::match[]
    affiche(v1);
    affiche(v2);
    affiche(v3);
    // end::match[]
}
