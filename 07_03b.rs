/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
 
fn main(){
    
    let edad: i32 = 25;
    
    match edad {
        0..=12 => println! ("Niños"),
        13..=19 => println! ("Adolescentes"),
        20..=29 => println! ("Jóvenes"),
        _ => println!("Edad no válida"),
    }
}