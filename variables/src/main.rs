fn main() {
    let mut x = 5;
    println!("EL valor de x es: {x}");
    x = 6;
    println!("EL valor de x es: {x}");

    // shadowing
    {
        let x = x * 2;
        println!("EL valor de x es {x}");
    }

    // Las constantes deben de tener el tipado
    // Como convencion se utilizan mayus y _ para constantes
    const THREE_HOURS_IN_SECONDS : u32 = 30*30*3;

    println!("Valor de la constante: {THREE_HOURS_IN_SECONDS}");

    println!("===============================");


    // Operaciones numericas

    //suma
    let suma = 2+10;
    println!("suma: {suma}");

    //resta
    let resta = 95.5 - 4.3;
    println!("resta: {resta}");

    //multiplicacion
    let producto = 4 * 30;
    println!("multiplicar: {producto}");

    //division
    let division = 56.7 / 32.2;
    println!("division: {division}");

    //mod 
    let modulo = 43%5;
    println!("modulo: {modulo}");


}
