use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    
    let random = rand::thread_rng().gen_range(1..101);
    println!("random: {}", random);

    loop {
        println!("Write a number");
        let mut sup = String::new();
    
        // TODO: Gérer le cas d'erreur
        io::stdin()
            .read_line(&mut sup)
            .expect("not good number");
        
        println!("votre num et {}", sup);
    
        let sup: u32 = match sup.trim().parse(){
            Ok(nombre) => nombre,
            Err(_) => {
                println!("Not a number");
                continue
            },
        };

        match sup.cmp(&random) {
            Ordering::Less => println!("C'est plus !"),
            Ordering::Greater => println!("C'est moins !"),
            Ordering::Equal => {
                println!("Vous avez gagné !");
                break;
            }
        }
    }
}

