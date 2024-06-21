/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
fn main(){

    let mut precios: Vec<f32> = vec![2.50, 3.20, 9.99];

    for precio in 0..precios.len() {
        precios[precio] = 2.50;
    }
}