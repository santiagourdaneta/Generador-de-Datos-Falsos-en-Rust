use rand::Rng;

// Aquí guardamos los nombres, apellidos y calles que usaremos.
// ¡Como ingredientes para nuestras recetas!
const FIRST_NAMES: &[&str] = &[
    "Juan", "María", "Pedro", "Ana", "Carlos", "Sofía", "Diego", "Laura", "Javier", "Elena",
    "Miguel", "Isabel", "Fernando", "Lucía", "Alejandro", "Carmen", "Daniel", "Paula",
];
const LAST_NAMES: &[&str] = &[
    "García", "Rodríguez", "López", "Martínez", "Pérez", "González", "Sánchez", "Ramírez",
    "Torres", "Flores", "Díaz", "Hernández", "Vázquez", "Morales", "Ortiz", "Ruiz", "Jiménez",
];
const STREET_NAMES: &[&str] = &[
    "Avenida Siempre Viva", "Calle Falsa", "Bulevar de la Esperanza", "Plaza del Sol",
    "Calle de los Pinos", "Camino Real", "Paseo de las Flores", "Calle de la Luna",
];
const CITIES: &[&str] = &[
    "Madrid", "Barcelona", "Valencia", "Sevilla", "Bilbao", "Málaga", "Zaragoza", "Alicante",
    "Murcia", "Palma", "Las Palmas", "Vigo", "Granada", "Córdoba", "Santander",
];
const DOMAINS: &[&str] = &["gmail.com", "hotmail.com", "yahoo.com", "protonmail.com"];

/// Genera un nombre completo falso.
pub fn generate_name() -> String {
    let mut rng = rand::thread_rng();
    let first = FIRST_NAMES[rng.gen_range(0..FIRST_NAMES.len())];
    let last = LAST_NAMES[rng.gen_range(0..LAST_NAMES.len())];
    format!("{} {}", first, last)
}

/// Genera una dirección falsa completa.
pub fn generate_address() -> String {
    let mut rng = rand::thread_rng();
    let street = STREET_NAMES[rng.gen_range(0..STREET_NAMES.len())];
    let number = rng.gen_range(1..=200);
    let city = CITIES[rng.gen_range(0..CITIES.len())];
    format!("{}, {} {}", street, number, city)
}

/// Genera un correo electrónico falso.
pub fn generate_email() -> String {
    let mut rng = rand::thread_rng();
    let first = FIRST_NAMES[rng.gen_range(0..FIRST_NAMES.len())].to_lowercase();
    let last = LAST_NAMES[rng.gen_range(0..LAST_NAMES.len())].to_lowercase();
    let domain = DOMAINS[rng.gen_range(0..DOMAINS.len())];
    format!("{}.{}@{}", first, last, domain)
}

