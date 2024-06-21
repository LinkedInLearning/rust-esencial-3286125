/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

fn main(){
    let mut edades: Vec<u32> = Vec::new();
    edades.push(27);
    edades.push(30);
    edades.push(19);
    edades.push(25);
    edades.push(35);
    edades.insert(2, 30);
    let ultimo_elemento = edades.pop();
    let elemento_eliminado = edades.remove(2);
    let elemento = edades.get(1);
    edades[4] = 29;
}