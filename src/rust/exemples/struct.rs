fn main() {
    // tag::struct[]
    struct Personne {
        nom: String,
        année_naissance: i32,
    }
    
    let mut ada = Personne {
        nom : String::from("Ada Lovelace"),
        année_naissance: 0,
    };
    
    println!("nom = {}", ada.nom);
    ada.année_naissance = 1815;
    // end::struct[]

    // tag::cons[]
    fn nouvelle_personne(nom: String, année: i32) -> Personne {
        Personne {
            année_naissance: année,
            nom, // sucre syntaxique
        }
    }
    // end::cons[]
}
