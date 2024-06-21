/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
 
const PI: f64 = 3.141598;

fn main(){

    let radio: f64 = 3.5;
    let radio2: f64 = 4.8;

    let diametro = 2.0 * radio;
    let diametro2 = 2.0 * radio2;
    let area = PI * radio * radio;
    let circunferencia = 2.0 * PI * radio;
    
    println!("Diámetro: {:.2}", diametro);
    println!("Área: {:.2}", area);
    println!("Circunferencia: {:.2}", circunferencia);
}