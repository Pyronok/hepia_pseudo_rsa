﻿///
///  Nom du projet 		: project_rsa
///  But du projet		: simuler le codage RSA
///  Date de création	: 29.11.2018
///  Version			: 0.1.0
///  Nom du fichier     : lib.rs
///  But du fichier     : librairie du projet, c'est à partir de ce fichier que le projet utilise les autres fichiers contenant le code
///
 pub mod euclide_etendu;
 pub mod algo_exp_rap;

#[cfg(test)] // cette partie sera exécuté que lors du test
mod tests {

  #[test] // la fonction suivant cette annotation sera exécutée comme test
  fn it_works() {
      assert_eq!(2 + 2, 4); 
}
