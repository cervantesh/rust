extern crate  rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Adivina el número!");

    let secret_number = rand::thread_rng().gen_range(1,101);

    //println!("El número secreto es: {}.",secret_number);

    loop {
    	println!("Favor ingrese su adivinanza.");
	    let mut guess = String::new();
		io::stdin().read_line(&mut guess).expect("Fallo al leer la linea");
	    let guess: u32 = match guess.trim().parse(){
	    	Ok(num) => num,
	    	Err(_) => {println!("Error Ingrese un número"); continue;}
	    };

	    println!("Usted adivino: {}", guess);

	    match guess.cmp(&secret_number) {
	    	Ordering::Less    => println!("Muy Pequeño!"),
	        Ordering::Greater => println!("Muy Grande!"),
	        Ordering::Equal   =>{println!("Es Igual! Ganaste "); break;
	    	}
	    }
    }

    
}