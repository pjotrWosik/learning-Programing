use std::io;

fn main() {
    let mut historia: Vec<String> = Vec::new();
    
    loop {
        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut wybur = String::new();

        println!("wpisz pierwszy numer: ");
        io::stdin()
            .read_line(&mut input1)
            .expect("niema liczby");
        let liczba1: f32 = input1.trim().parse().expect("niedostalem numera");

        println!("wpisz drugi numer: ");
        io::stdin()
            .read_line(&mut input2)
            .expect("niema liczby");
        let liczba2: f32 = input2.trim().parse().expect("niedostalem numera");

        println!("wybierz Dzialanie z wybranych:");
        println!("1 - dodawanie");
        println!("2 - odejmowanie");
        println!("3 - mnorzenie");
        println!("4 - dzielenie");
        println!("5 - potegowanie");
        println!("6 - pierwiastek tylko jenda liczba sie liczy");
        println!("7 - historia");
        println!("8 - wyjscie");
        io::stdin()
            .read_line(&mut wybur)
            .expect("niema wybora");

        if wybur.trim() == "1" {
            let liczba3 = liczba1 + liczba2;
            println!("wynik dodawania to {} + {} = {}", liczba1, liczba2, liczba3);
            historia.push(format!("{} + {} = {}", liczba1, liczba2, liczba3));
        }
        else if wybur.trim() == "2" {
            let liczba3 = liczba1 - liczba2;
            println!("wynik odejmowania to {} - {} = {}", liczba1, liczba2, liczba3);
            historia.push(format!("{} - {} = {}", liczba1, liczba2, liczba3));
        }
        else if wybur.trim() == "3" {
            let liczba3 = liczba1 * liczba2;
            println!("wynik mnorzenia to {} * {} = {}", liczba1, liczba2, liczba3);
            historia.push(format!("{} * {} = {}", liczba1, liczba2, liczba3));
        }
        else if wybur.trim() == "4" {
            if liczba2 == 0.0 {
                println!("Niemorzna dzielic przez zero");
                historia.push(format!("Blad obliczania"));
            }
            else {
                let liczba3 = liczba1 / liczba2;
                println!("wynik dzielenia to {} / {} = {}", liczba1, liczba2, liczba3);
                historia.push(format!("{} / {} = {}", liczba1, liczba2, liczba3));
            }
        }
        else if wybur.trim() == "5" {
            let liczba3 = liczba1.powf(liczba2);
            println!("wynik potegi to {}", liczba3);
            historia.push(format!("{}^2 = {}", liczba1, liczba3));
        }
        else if wybur.trim() == "6" {
            let liczba3 = liczba1.sqrt();
            println!("wynik pierwiastka to {}", liczba3);
            historia.push(format!("√{} = {}", liczba1, liczba3));
        }
        else if wybur.trim() == "7" {
            println!("historia: ");
            if historia.is_empty() {
                println!("historia jest pusta skurwysynie")
            }
            else {
                for h in &historia {
                    println!("{}", h);
                }
            }
        }
        else if wybur.trim() == "8" {
            println!("Program zostal zakonczony");
            break;
        }
        let mut kontynuj = String::new();
        println!("Wpisz 1 jerzeli chcesz kontynuowac, lub cos innego jerzeli chcesz zakonczyczyc:");
        io::stdin()
            .read_line(&mut kontynuj)
            .expect("blad z kontynuj");

        if kontynuj.trim() != "1" {
            println!("koniec Programu");
            break;
        }
    }
}
