// Aquí le decimos a nuestra "caja" que existe el "libro de recetas" llamado 'faker'.
mod faker;

fn main() {
    println!("--- Generador de Datos Falsos ---");
    println!("Nombre: {}", faker::generate_name());
    println!("Dirección: {}", faker::generate_address());
    println!("Correo: {}", faker::generate_email());
}