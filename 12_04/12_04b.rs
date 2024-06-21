/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */

 fn main() {
    let empleados = vec![
        (1, "Roy Trenneman".to_string()),
        (2, "Maurice Moss".to_string()),
        (3, "Jen Barber".to_string()),
    ];

    let id = 4;
    let mut empleado_encontrado = None;

    for empleado in &empleados {
        if empleado.0 == id {
            empleado_encontrado = Some(&empleado.1);
            break;
        }
    }

    match empleado_encontrado {
        Some(nombre_completo) => println!("\nEmpleado encontrado! {}\n", nombre_completo),
        None => println!("\nEmpleado no encontrado...\n"),
    }
}