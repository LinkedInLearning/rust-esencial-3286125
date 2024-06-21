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

    let _hoy: Dia = Dia::Sabado;

    enum RespuestaHttp{
        Exito(String),
        Error(u16, String),
    }

    let _respuesta_exitosa = RespuestaHttp::Exito(String::from("<html>Contenido de la página</html>"));
    let _respuesta_error = RespuestaHttp::Error(404, String::from("No encontrado"));
}