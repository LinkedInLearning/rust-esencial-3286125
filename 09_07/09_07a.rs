/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){

    let sueldo_bruto_anual: f64 = 90_400.43;
    
    println!("El sueldo bruto mensuale es: {:.2}", 
                calcula_sueldo_mensual(sueldo_bruto_anual));
}