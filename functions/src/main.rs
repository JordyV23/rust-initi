fn main() {
    println!("Hello, world!");

    another_function(23,'s');

    let x = five();

    println!("El valor de x es {x}");
}

fn another_function(x: i32, unit_label: char) {
    println!("Tardaste: {x}{unit_label}");
}

fn five() -> i32 {
    5
}
