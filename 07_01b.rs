/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){
    let sanciones_pendientes: bool;
    let tuvo_sanciones: bool;
    
    if sanciones_pendientes {
        println!("Oh... El usuario tiene sanciones pendientes");
    } else if tuvo_sanciones {
        println!("Lo sentimos, no puedes reservar nuevos libros.");
    } else {
        println!("Usuario libre de sanciones :).");
    }
}