/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
 
 fn main(){
    let a = 10;
    let b = 5;
    let tipo_de_operacion: char = '+';
    
    if b == 0 {
        println!("Error: No se puede dividir por cero.");
    } else {
        if tipo_de_operacion == '+' || tipo_de_operacion == '-' || tipo_de_operacion == '*' || tipo_de_operacion == '/' {
            let resultado = match tipo_de_operacion {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => a / b,
                _ => 0,
            };
            println!("Resultado: {}", resultado);
        } else {
            println!("Operación no válida");
        }
    }
}