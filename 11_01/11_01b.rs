/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){

    let mut num_repeticiones: u8 = 3;

    while num_repeticiones != 0 {
        println!("--------------------------------------------------");
        println!("Aprendiendo a implementar bucles while en Rust.");
        println!("Valor de num_repeticiones: {}.", num_repeticiones);

        num_repeticiones = num_repeticiones - 1;
    }
}