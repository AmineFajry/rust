use rand::Rng;
//use std::collections::HashMap;
use std::io;

/*fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}*/

/*fn random_number(){
    let random_number = rand::thread_rng().gen_range(2..=32); // Genere un i32 allant de 1 à 12 inclus
    println!("{}",random_number);
}

fn multiply_two(number: i32) -> i32 { // Comme la fonction retourne une valeur, on doit déclarer son type apres les paramères comme ici -> i32
    return number * 2 ; 
}

fn string_length(nom: String)
{
    println!("{}",nom.len());
}

fn last_hashmap()
{
    let mut mois = HashMap::new();
    mois.insert(1,"janvier");
    mois.insert(2,"février");
    mois.insert(3,"mars");
    let vec1 = Vec::from_iter(mois.iter());
    println!("{:?}", vec1.last());
}*/

fn main() {

    /*
    Exercice 2
    let mut my_num = 6;
    println!("{}",my_num);

    my_num = 5;
    println!("{}", my_num);

    let my_num = 7; 
    println!("{}",my_num);
    */


    /*
    Exercice 3
    let mut logical1: bool = true;
    let logical2: bool = false;

    println!("{logical1}");
    println!("{logical2}");
    
    logical1 = false;
    println!("{logical1}");
    

    let a_float: f64 = 1.0;
    let an_integer = a_float as u8;
    println!("{an_integer}");
    
    let boolean = match an_integer{
           0 => false,
           1 => true,
           _ => panic!("Invalid bool in u8"),
    };
    println!("{} -> {}", an_integer, boolean);


    let mut mutable = 12;
    let mutable = true;
    println!("{mutable}");
    
    
	let array = [1, 2, 3.123 as u8];
    println!("{:?}", array);

    
	let my_var = 1;
	let my_var = "a" ;
    println!("{}", my_var ); 
    */   

    /*
    Exercice 5
    let random_number = rand::thread_rng().gen_range(1..=12); // Genere un i32 allant de 1 à 12 inclus
    
    //vector
    let new_vec = vec!["janvier", "février", "mars", "avril", "mai", "juin", "juillet", "août", "septembre", "octobre", "novembre", "décembre"];
    //hashmap
    let mut mois = HashMap::new();
    mois.insert("1","janvier");
    mois.insert("2","février");
    mois.insert("3","mars");
    mois.insert("4","avril");
    mois.insert("5","mai");
    mois.insert("6","juin");
    mois.insert("7","juillet");
    mois.insert("8","août");
    mois.insert("9","septembre");
    mois.insert("10","octobre");
    mois.insert("11","novembre");
    mois.insert("12","décembre");

    println!("{}",new_vec[random_number]);
    let s: String = random_number.to_string();
    println!("{}",mois.get(&s as &str).unwrap());
    */

    /*
    //Exercice  A
    let mut i = 4;
    let end = 1;
    let step = -1;
    
    while i >= end {
        println!("i: {}", i);
        i += step;
    }

    //Exercice  B
    let mut count = 0;
    let arr = [1, 2, 3, 4];
    for value in arr {
        count += value;
    }
    println!("{}", count);

    //Exercice  C

    println!("Ecrivez votre prénom et votre année de naissance : ");
    let mut input = String::new();


    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }

    let v: Vec<&str> = input.split(',').collect();
    let my_int:i32 = v[1].trim().parse().unwrap_or(0);
    let age = 2022 - my_int;
    println!("{} fêtera ses {} ans au cours de cette année", v[0], age);
    */

    /*
    //Exercice 6
    random_number();
    let num  = multiply_two(4);
    println!("{}", num);
    string_length(String::from("TATA"));

    last_hashmap();*/

    //Exercice 7
    /*let random_number = rand::thread_rng().gen_range(1..=100); // Genere un i32 allant de 1 à 12 inclus
    println!("Ecrivez un nombre entre 1 et 100 : ");
    let condition: bool = false;

    while condition != true{

        let mut input = String::new();
        io::stdin().read_line(&mut input);
        let my_int:i32 = input.trim().parse().unwrap_or(0);

        if my_int == random_number {
            println!("bravo !");
            let condition = true;
            break;
        }
        else if my_int > random_number{
            println!("votre nombre est grand {}",random_number);
        }
        else if my_int < random_number{
            println!("votre nombre est petit {}",random_number);
        }
        let condition = false;
    }*/

    //Exercice 8
    

}
