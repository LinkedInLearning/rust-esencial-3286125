/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

use std::io;

fn main() {
    println!("-------------------------------");
    println!("Por favor, escribe tu nombre de usuario: \n");
    
    let mut mensaje = solicitar_mensaje();
    
    while let None = mensaje {
        println!("Nombre de usuario no detectado.");
        println!("Por favor, escribe tu nombre de usuario: \n");
        mensaje = solicitar_mensaje();
    }
    
    println!("El nombre de usuario introducido es: {}", mensaje.unwrap());
}

fn solicitar_mensaje() -> Option<String> {
    
    let mut mensaje = String::new();
    io::stdin().read_line(&mut mensaje);
    let mensaje_sin_salto_de_linea = mensaje.trim().to_string();
    
    if mensaje_sin_salto_de_linea.is_empty(){
        None
    } else {
        Some(mensaje)
    }
}