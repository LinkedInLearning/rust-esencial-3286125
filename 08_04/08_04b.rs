/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

 fn main(){
    let mut matriz: Vec<Vec<i32>> = Vec::new();
    matriz.push(vec![1,2,3]);
    matriz.push(vec![7,1,4]);
    matriz.push(vec![6,3,8]);
    matriz[2][1] = 9;
}