use std::io;
use rand::Rng;


fn main() {
    witam();

}

fn witam() {
    println!("ようこそ！");
    print!("U will now play A game:");
    println!("U will need to guess a number between 100 - 1");
    println!("U choose wisely u hawe only 5 tries");
    println!(" ");
    println!("write /yes/ if you want to start or /stop/ if you want to stop game: ");
    asking()
}
fn asking() {
    loop {
        let mut start_inp = String::new();
        io::stdin()
            .read_line(&mut start_inp)
            .unwrap();
        let start_inp = start_inp.trim();

        if start_inp == "yes" || start_inp == "true" {
            println!("lets start the game:");
            println!("write an number between 1 and 100:");
            gra();
            break;
        }
        else if start_inp == "stop" {
            println!("the game hawe ben stoped C ya later:");
            break;
        } else {
            println!("please write /yes/ or /stop/");
        }
    }
}

fn gra() {
    let mut pruby = 5;
    println!("U will need to guess a number between 100 - 1\n");
    let magiczna = rand::rng().random_range(1..100);

    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .unwrap();
        let user_liczba: i32  = input.trim().parse().expect("u forgot i need an number!");


        pruby -= 1;
        if user_liczba < magiczna {
            println!("number is to small");
            println!("Try's left: {}", pruby);
        }
        else if user_liczba > magiczna {
            println!("number is too big");
            println!("Try's left: {}", pruby);
        }
        else if user_liczba == magiczna {
            println!("good boy!");
            asking();
            break;
        }
        if pruby == 0 {
            println!("You hawe 0 Try's left");
            println!("You lost the winning number is: {}",  magiczna);
            asking();
            break;
        }
    }

}
