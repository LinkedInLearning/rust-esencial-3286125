/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
 
use std::io;

fn main(){

    let mut indicacion_usuario = String::new();

    loop {
        println!("------------------------------------");
        println!("Escribe 'salir' para terminar: ");
        indicacion_usuario.clear();

        io::stdin().read_line(&mut indicacion_usuario);

        if indicacion_usuario.trim() == "salir" {
            break;
        }
    }
}