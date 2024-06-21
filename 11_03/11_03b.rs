/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
 
use std::io;

fn main(){

    let mut numero_secreto: u8 = 47;
    println!("------------------------------");
    println!("¡Adivina el número secreto! :) \n");

    loop {
        println!("Introduce un número: ");

        let mut numero_jugador = String::new();
        io::stdin().read_line(&mut numero_jugador);

        let mut numero_jugador: u8 = numero_jugador.trim().parse().unwrap();

        if numero_jugador == numero_secreto {
            println!("¡Enhorabuena! ¡Lo adivinaste!");
            println!("------------------------------");
            break;
        } else {
            println!("Inténtalo de nuevo...");
            println!("------------------------------");
        }
    }
}