/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
 
fn main(){

    let salarios = vec![30000, 28000, 29500, 33000, 31000];

    let mut num_personas_ingresos_bajos: u16 = 0;
    let mut num_personas_ingresos_medios: u16 = 0;
    let mut num_personas_ingresos_altos: u16 = 0;

    for salario in salarios.iter() {
        match salario {
            0..=29500 => num_personas_ingresos_bajos += 1,
            30000..=30000 => num_personas_ingresos_medios += 1,
            _ => num_personas_ingresos_altos += 1,
        }
    }
}