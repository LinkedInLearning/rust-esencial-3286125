/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
 
fn main(){

    let dividendo: u16 = 10;
    let divisor: u16 = 3;

    let cociente = calcular_cociente(dividendo, divisor);

    println!("El cociente es: {}", cociente);
}

fn calcular_cociente(dividendo: u16, divisor: u16) -> u16 {
    assert!(divisor != 0, "\n\n El divisor debe ser un número distinto de 0 \n\n");
    dividendo / divisor
}