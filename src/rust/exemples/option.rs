use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

// tag::option[]
fn racine(n : f64) -> Option<f64> {
    if n > 0.0 {
        Some(n.sqrt())
    } else {
        None
    }
}
// end::option[]

// tag::ownresult[]
fn lire_fichier_1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(fichier) => fichier,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
// end::ownresult[]

// tag::opint[]
fn lire_fichier_2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?; //<1>

    Ok(s)
}
// end::opint[]

fn main() {
    // tag::option[]
    racine(25.0);  // -> Some(5.0)
    racine(-25.0); // -> None
    
    if let Some(r) = racine(25.0) { //<1>
        println!("La racine est {}", r);
    } else {
        println!("Pas de racine réelle");
    }
    
    // panique si None
    let v : f64 = racine(25.0).unwrap(); 
    let v : f64 = racine(25.0).expect("erreur: racine d'un nombre négatif"); 
    // end::option[]

    // tag::conv[]
    let x: Result<u32, &str> = Ok(10);
    assert_eq!(Some(10), x.ok());
    let x: Result<u32, &str> = Err("Erreur");
    assert_eq!(None, x.ok());

    assert_eq!(Ok(10), Some(10).ok_or("Erreur"));
    let x : Option<u32> = None;
    assert_eq!(Err("Erreur"), x.ok_or("Erreur"));
    // end::conv[]

    // tag::result[]
    let r : Result<File, std::io::Error> = File::open("hello.txt");

    let f : File = match r {
        Ok(fichier) => fichier,
        Err(erreur) => panic!("Erreur d'ouverture: {:?}", erreur),
    };
    // end::result[]

    // tag::error[]
    let f = File::open("hello.txt");

    let f = match f {
        Ok(fichier) => fichier,
        Err(erreur) => match erreur.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Erreur de création du fichier : {:?}", e),
            },
            autre_erreur => {
                panic!("Erreur d'ouverture du fichier : {:?}", autre_erreur)
            }
        },
    };
    // end::error[]

    // tag::expect[]
    let f = File::open("hello.txt").expect("Échec à l'ouverture de hello.txt");
    // end::expect[]
}
