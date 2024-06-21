/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
 
enum Semaforo {
    Rojo,
    Amarillo,
    Verde,
}

fn accion_a_realizar(semaforo: Semaforo) -> String {

    match semaforo {
        Semaforo::Rojo => "Detente".to_string(),
        Semaforo::Amarillo => "Precaución".to_string(),
        Semaforo::Verde => "Avance".to_string(),
    }
}
fn main(){
    let semaforo_ecuestre = Semaforo::Verde;
    println!("Acción a realizar: {}", accion_a_realizar(semaforo_ecuestre));
}