/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

const PI: f64 = 3.141598;

fn main(){

    let radio: f64 = 3.5;

    let diametro = calcular_diametro(radio); // 2.0 * radio;
    let area = calcular_area(radio); // PI * radio * radio;
    let circunferencia = calcular_circunferencia(radio); // 2.0 * PI * radio;
    
    println!("Diámetro: {:.2}", diametro);
    println!("Área: {:.2}", area);
    println!("Circunferencia: {:.2}", circunferencia);
}

fn calcular_diametro(r: f64) -> f64 {
    2.0 * r
}

fn calcular_area(radio: f64) -> f64 {
    PI * radio * radio
}

fn calcular_circunferencia(radio: f64) -> f64 {
    2.0 * PI * radio
}