/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
 
fn main(){

    let mut temperatura = 100;
    let mut presion = 1;

    let temperatura_objetivo = 25;
    let presion_objetivo = 5;

    while temperatura > temperatura_objetivo && presion < presion_objetivo {
        temperatura = temperatura - 3;
        presion = presion + 1;
        println!("Temperatura: {}ºC, Presión: {} atm", temperatura, presion);
    }
}