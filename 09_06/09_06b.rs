/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
fn main(){

    let dividendo: u8 = 10;
    let divisor: u8 = 0;

    let cociente: u8 = calcular_cociente(dividendo, divisor);

    println!("El cociente es: {}", cociente);
}

fn calcular_cociente(dividendo: u8, divisor: u8) -> u8 {
    assert!(divisor != 0,
        "\n\nEl divisor debe ser un número distinto de 0.\n\n");
    dividendo / divisor
}