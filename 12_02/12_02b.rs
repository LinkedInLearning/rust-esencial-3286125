/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){

    let dividendo: u16 = 216;
    let divisor: u16 = 2;

    let cociente = calcular_cociente(dividendo, divisor);
    
    if let Some(108) = cociente {
        println!("¡Huevo de pascua desbloqueado! 4 8 15 16 23 42...");
    }
    
    match cociente {
        Some(valor) => println!("El cociente es: {}", valor),
        None => println!("No se puede dividir entre cero."),
    }
}

fn calcular_cociente(dividendo: u16, divisor: u16) -> Option<u16> {
    if divisor == 0{
        None
    } else {
        Some(dividendo / divisor)
    }
}