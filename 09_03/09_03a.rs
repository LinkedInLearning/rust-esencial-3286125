/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
 
const PI: f64 = 3.141598;

fn main(){

    let radio: f64 = 3.5;

    let diametro = calcular_diametro(); // 2.0 * radio;
    let area = calcular_area(); // PI * radio * radio;
    let circunferencia = calcular_circunferencia(); // 2.0 * PI * radio;
    
    println!("Diámetro: {:.2}", diametro);
    println!("Área: {:.2}", area);
    println!("Circunferencia: {:.2}", circunferencia);
}