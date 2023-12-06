use std::io;

fn main() {
    println!("Kalkulator Suhu dan Fibonacci");
    println!("Pilih opsi di bawah : ");
    println!("1. Angka Fibonacci ke-N");
    println!("2. Kalkulator Suhu");
    println!("Masukkan pilihan Anda (angka saja): ");

    let mut resp = String::new();
    let mut angka = String::new();
    let mut suhu = String::new();
    let mut suhu_resp = String::new();

    io::stdin().read_line(&mut resp).expect("Failed to read line!");

    let resp: i32 = resp.trim().parse().expect("Harus angka COK!");

    if resp == 1 {
        println!("Masukkan angka ke-N Fibonacci Anda: ");

        io::stdin().read_line(&mut angka).expect("Failed to read line!");

        let angka: i32 = angka.trim().parse().expect("Harus angka!");

        angka_fibonacci(angka);

        println!("Angka Fibonnaci ke-{angka} adalah : {:?}", angka_fibonacci(angka));
    }

    if resp == 2 {
        println!("Pilih unit temperatur untuk dikonversi : ");
        println!("1. Celcius ke Fahrenheit");
        println!("2. Fahrenheit ke Celcius");
        println!("Masukkan angka saja : ");

        io::stdin().read_line(&mut suhu_resp).expect("Failed to read line!");

        let suhu_resp: i32 = suhu_resp.trim().parse().expect("Harus angka!");

        if suhu_resp == 1 {
            println!("Masukkan suhu yang akan dikonversi: ");

            io::stdin().read_line(&mut suhu).expect("Failed to read line!");

            let suhu: i32 = suhu.trim().parse().expect("Harus angka yagesya!");

            kalkulator_celcius(suhu);

            println!("Suhu dari Celcius ke Fahrenheit adalah : {:?}", kalkulator_celcius(suhu));
        }

        if suhu_resp == 2 {
            println!("Masukkan suhu yang akan dikonversi: ");

            io::stdin().read_line(&mut suhu).expect("Failed to read line!");

            let suhu: i32 = suhu.trim().parse().expect("Harus angka yagesya!");

            kalkulator_fahren(suhu);

            println!("Suhu dari Fahrenheit ke Celcius adalah : {:?}", kalkulator_fahren(suhu));
        }
    };
}

fn angka_fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    return angka_fibonacci(n - 1) + angka_fibonacci(n - 2);
}

fn kalkulator_celcius(n: i32) -> i32 {
    let celcius = n * 9 / 5 + 32;

    return celcius;
}

fn kalkulator_fahren(n: i32) -> i32 {
    let fahre = (n - 32) * 5 / 9;

    return fahre;
}


