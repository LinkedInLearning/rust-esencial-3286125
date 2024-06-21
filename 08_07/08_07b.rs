/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

use std::io;

fn main(){
     println!("Introduce tu mensaje: ");
 
     let mut mensaje = String::new();
     io::stdin().read_line(&mut mensaje);
     mensaje = mensaje.trim().to_string();
     println!("Mensaje leido: {}", mensaje);
     println!("----------------------------------------");
     println! ("Introduce un número: ");
     let mut numero: String = String::new();
     io::stdin().read_line(&mut numero);
     let mut numero: f64 = numero.trim().parse().unwrap();
}