use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Advinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);


    loop{
        println!("Por favor, insira um número.");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
    
        let guess: u32 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("Você chutou: {guess}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você Acertou!!!");
                break;
            }
        }
    
    }

}

