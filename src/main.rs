///
///  Nom du projet 		: 
///  But du projet		: 
///  Date de création	: 23.11.2018
///  Version			: 0.1.0
///  Nom du fichier     : main.rs
///  But du fichier     : 
///
extern crate project_rsa;
use std::io::{self, Write};
use project_rsa::*; 

///
/// Fonction read_line
/// ...
/// Permet de récupérer la valeur saisie par l'utilisateur et la convertir en une chaine de caractères
/// ...
/// return [String] renvoie la saisie de l'utilisateur convertie en chaine de caractères
///
fn read_line() -> String {
	let mut guess = String::new();

	io::stdin().read_line(&mut guess)
	    	.expect("Failed to read line.");

	guess
}

///
/// Fonction read_int
/// ...
/// Permet de recuperer la valeur saisie par l'utilisateur et la convertir en un
/// nombre entier en faisant appel a la fonction str_to_int
/// return [Option<u32>] renvoie une option de nombre entier qui sera soit rempli d'une valeur entière, soit vide en cas d'erreur
///
fn read_int() -> Option<u32>{
	str_to_int(&read_line())
}


///
/// Fonction str_to_int
/// ...
/// Permet de convertir une variable en un nombre entier. 
/// Elle a ete modifiee pour ne pas planter dans le cas d'une fausse information
/// #exemple
/// ```
/// let mut nb_entier = "1";
/// str_to_int(nb_entier);
/// ```
/// return [Option<u32>] renvoie une option de nombre entier qui sera soit rempli d'une valeur entière, soit vide en cas d'erreur
///
fn str_to_int(line: &str) -> Option<u32> {
	match line.trim().parse() {
		Ok(nb) => Some(nb),
		Err(_) => None,
	}
}




fn main() {
    
}
