/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){
    println!("Nombre de usuario: @EliezerLopez");
    let edad: u8 = 30;

    {
        let inicial_nombre: char = 'E';
        println!("La edad es {}", edad);
    }
}