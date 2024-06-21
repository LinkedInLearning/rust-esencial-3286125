/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

const IVA: f64 = 0.21;

fn main(){

    let precio_sin_impuestos: f64 = 10.00;
    let precio_con_impuestos = precio_sin_impuestos + (precio_sin_impuestos*IVA);
    println!("Precio sin IVA: {:.2}", precio_sin_impuestos);
    println!("Precio con IVA: {:.2}", precio_con_impuestos);
}