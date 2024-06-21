/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){
    
    let a = 7;
    let b = 10;

    println!("a < b es {}", a < b);
    println!("a > b es {}", a > b);
    println!("a == b es {}", a == b);
    println!("a <= b es {}", a <= b);
    println!("a >= b es {}", a >= b);
    println!("a != b es {}", a != b);

    let es_externo = true;
    let acceso_instalaciones = !es_externo;
    let es_manager = false;
    let acceso_despacho = acceso_instalaciones || es_manager;
}