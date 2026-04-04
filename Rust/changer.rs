use std::io;
use std::process;
use phf::phf_map;

static DLUGOSCI: phf::Map<&'static str, f64> = phf_map! {
    "km"    => 1000.0,
    "m"     => 1.0,
    "cm"    => 0.01,
    "mm"    => 0.001,
    "mila"  => 1609.344,
    "cal"   => 0.0254,
    "stopa" => 0.3048,
    "yard"  => 0.9144,
};

static WAGA: phf::Map<&'static str, f64> = phf_map! {
    "kg"      => 1.0,
    "g"       => 0.001,
    "mg"      => 0.000001,
    "tona"    => 1000.0,
    "funt"    => 0.45359237,
    "uncja"   => 0.028349523125,
};

static OBJETOSCI: phf::Map<&'static str, f64> = phf_map! {
    "litr"     => 0.001,
    "ml"       => 0.000001,
    "m3"       => 1.0,
    "galon_us" => 0.003785411784,
    "galon_uk" => 0.00454609,
};

fn main() {
    witaj()
}

fn witaj() {
    println!("Witaj w programie Zmiana jednostek");
    println!("|= Wybór działania =|");
    println!("1 - Długości");
    println!("2 - Wagi");
    println!("3 - Objętości");
    println!("4 - wylacz program");
    println!("5 - Wyjdź z programu");
    println!("====================");

    zapytanie()
}

fn zapytanie() {
    loop {
        println!("\nWybierz kategorię (1,2,3,5):");

        let mut wybor = String::new();
        io::stdin()
            .read_line(&mut wybor)
            .expect("Błąd odczytu linii");

        let wybor = wybor.trim();

        match wybor {
            "1" => konwertuj(&DLUGOSCI, "długość"),
            "2" => konwertuj(&WAGA,    "wagę"),
            "3" => konwertuj(&OBJETOSCI, "objętość"),
            "4" => ending(),
            "5" => {
                println!("Do widzenia! Dziękujemy za skorzystanie.");
                break;
            }
            _ => println!("Nieprawidłowy wybór. Spróbuj ponownie."),
        }
    }
}

fn konwertuj(jednostki: &phf::Map<&'static str, f64>, kategoria: &str) {
    println!("\nPodaj wartość do przeliczenia:");

    let mut wartosc_str = String::new();
    io::stdin()
        .read_line(&mut wartosc_str)
        .expect("Błąd odczytu wartości");

    let wartosc: f64 = match wartosc_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("To nie jest poprawna liczba!");
            return;
        }
    };

    println!("\nDostępne jednostki ({kategoria}):");
    for key in jednostki.keys() {
        println!(" - {key}");
    }

    println!("\nZ jakiej jednostki przeliczasz?");
    let mut z_jednostki = String::new();
    io::stdin()
        .read_line(&mut z_jednostki)
        .expect("Błąd odczytu");
    let z_jednostki = z_jednostki.trim();

    println!("Na jaką jednostkę przeliczasz?");
    let mut na_jednostke = String::new();
    io::stdin()
        .read_line(&mut na_jednostke)
        .expect("Błąd odczytu");
    let na_jednostke = na_jednostke.trim();

    // Sprawdzamy czy jednostki w ogóle istnieją w mapie
    let wsp_z = match jednostki.get(z_jednostki) {
        Some(&w) => w,
        None => {
            println!("Nie znam jednostki: {z_jednostki}");
            return;
        }
    };

    let wsp_na = match jednostki.get(na_jednostke) {
        Some(&w) => w,
        None => {
            println!("Nie znam jednostki: {na_jednostke}");
            return;
        }
    };

    let wynik = wartosc * wsp_z / wsp_na;

    println!("\n{wartosc} {z_jednostki} = {wynik} {na_jednostke}");
}

fn ending() {
    println!("Kończę program...");
    process::exit(0);
}
