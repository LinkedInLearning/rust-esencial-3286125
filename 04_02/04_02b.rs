/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){
    let edad: u8 = 30;
    let numero_de_acciones: u8 = 150;

    let ahorros_en_el_banco: f64 = 10000.46;
    
    let inicial_nombre: char = 'E';
    let inicial_primer_apellido: char = 'L';
    let inicial_segundo_apellido: char = 'R';

    println!("El inversor {}{}{} tiene {} años,\nha adquirido {} acciones durante los últimos meses,\ny posee una cantidad de dinero ahorrada equivalente a {}€.", inicial_nombre, inicial_primer_apellido, inicial_segundo_apellido, edad, numero_de_acciones, ahorros_en_el_banco);
}