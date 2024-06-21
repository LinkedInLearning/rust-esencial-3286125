/*
 * Curso: Rust Esencial
 * Plataforma: LinkedIn Learning
 * Instructor: @EliezerLopez
 * */
fn main(){

    enum Dia{
        Lunes,
        Martes,
        Miercoles,
        Jueves,
        Viernes,
        Sabado,
        Domingo,
    }

    impl Dia {
        fn es_laboral(&self) -> bool {
            match self {
                Dia::Sabado | Dia::Domingo => false,
                _ => true,
            }
        }
    }
}