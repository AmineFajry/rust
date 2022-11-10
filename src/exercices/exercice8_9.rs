//use rand::Rng;
//use std::collections::HashMap;
use std::io;

/*
struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(longueur:f32, largeur:f32)->f32
{
    return longueur * largeur;
}

fn soustraction(x:f32,y:f32)->f32
{
    return x - y;
}

fn square(longueur : Point, largeur :Point)->Rectangle
{
    let _rectangle = Rectangle{
        top_left : longueur,
        bottom_right : largeur
    };
    return _rectangle;
}
*/

struct Personnage {
    nom : String,
    prenom: String,
    age : String,
    classe : String,
    niveau : String,
    pouvoir : String,
    point_de_vie : String
}

fn attribution_pouvoir(classe : String)->String
{
    if classe == "guerrier".to_string()
    {
        return "arme => bouclier, pistolet".to_string();
    }
    else if classe == "sorcier".to_string()
    {
        return "magie".to_string();
    }
    else if classe == "druide".to_string()
    {
        return "intelligence artificielle, pistolet".to_string();
    }
    return "Resaisissez votre classe".to_string(); 
}

fn main() {
    /*
    Exercice 8
    // Instantiate a `Point`
    let longueur: Point = Point { x: 10.3, y: 0.4 };
    let largeur: Point = Point { x: 5.3, y: 0.4 };
    let _rectangle = Rectangle {
        top_left: longueur,
        bottom_right: largeur,
    };

    let size_longueur = soustraction(_rectangle.top_left.x,_rectangle.top_left.y);
    let size_largeur = soustraction(_rectangle.bottom_right.x,_rectangle.bottom_right.y);

    let rect = rect_area(size_longueur,size_largeur);
    println!("{}",rect);

    let _square = square(longueur,largeur);
    println!("{}",_square.top_left.y);
    */

    println!("Votre nom : ");
    let mut nom = String::new();
    io::stdin().read_line(&mut nom);

    println!("Votre prenom : ");
    let mut prenom = String::new();
    io::stdin().read_line(&mut prenom);

    println!("Votre age : ");
    let mut age = String::new();
    io::stdin().read_line(&mut age);

    println!("Votre classe (sorcier, druide, guerrier) : ");
    let mut classe = String::new();
    io::stdin().read_line(&mut classe);

    println!("----------Caracteristique----------");
    println!("nom : {}",nom);
    println!("prenom : {}",prenom);
    println!("age : {}",age);
    println!("classe : {}",classe);
    println!("niveau : 1 débutant");

    if classe == "guerrier"
    {
        let pouvoir =  "arme => bouclier, pistolet";
        println!("pouvoir : {}",pouvoir);
    }
    else if classe == "sorcier"
    {
        let pouvoir =  "magie";
        println!("pouvoir : {}",pouvoir);
    }
    else if classe == "druide"
    {
        //let pouvoir = "IA, pistolet";
        println!("IA, pistolet");
    }
    else if classe != "sorcier" || classe != "guerrier" || classe != "druide"
    {
        println!("error, ressaissez votre classe!" );
    }

    println!("Point de vie : 5/5");
    println!("-----------------------------------");
    
}

