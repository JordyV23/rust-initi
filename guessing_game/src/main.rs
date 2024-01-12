use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    welcome();

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // crea un bucle infinito
    loop {
        println!("Por favor ingrese su adivinanza");

        // mut permite hacer a una variable mutable
        let mut guess = String::new();

        io::stdin()
            // Envia variable por referencia
            .read_line(&mut guess)
            .expect("No se pudo leer la linea");

        // shadowing: volver a crear la variable
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // El _ es un valor de captura
            Err(_) => continue,
        };

        println!("Adivinaste: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => message("Muy pequeño"),
            Ordering::Greater => message("Muy grande"),
            Ordering::Equal => {
                victory();
                break;
            }
        }
    }
}

fn welcome() {
    println!("╔═════════════════════════════════════╗");
    println!("║      ¡Bienvenido al Juego de        ║");
    println!("║            Adivinanzas!             ║");
    println!("╚═════════════════════════════════════╝");
    println!();
    println!("🌟 Desafía tu mente adivinando un numero 🌟");
    println!("*****************************************");
}

fn message(msg: &str) {
    println!("╔═════════════════════════════════════╗");
    println!("                 {msg}               ");
    println!("╚═════════════════════════════════════╝");
}

fn victory() {
    println!("¡Felicidades! ¡Has adivinado correctamente!");
    println!(
        r#"
   (\_/)
   (='.'=)
   (")___(")

¡Ganaste! ¡Eres un maestro de las adivinanzas!
"#
    );
}
