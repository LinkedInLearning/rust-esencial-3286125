/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
fn main(){

    let precios: Vec<f32> = vec![2.50, 3.20, 9.99];

    for precio in precios.iter(){
        println!("El precio es: {}", precio);
    }

    for precio in 0..precios.len(){
        println!("El precio es: {}", precios[precio]);
    }
}