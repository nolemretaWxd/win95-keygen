use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1{
        println!("Usage: {} <number of keys> [--oem (generates OEM keys)]", args[0]);
    } else if args.len() > 1 {
        let number = &args[1];
        generate_keys(number.parse().unwrap(), args.len() >= 3 && args[2] == "--oem");
    }
}

fn generate_keys(number: i32, oem: bool) {
    for _i in 0..number {
        if oem {
            let day = format!("{:03}", rand::thread_rng().gen_range(1..=366));
            let year_choose = rand::thread_rng().gen_range(0..=8);
            let year = match year_choose {
                0 => "95",
                1 => "96",
                2 => "97",
                3 => "98",
                4 => "99",
                5 => "00",
                6 => "01",
                7 => "02",
                8 => "03",
                _ => "00",
            };
            let mut randomrng = rand::thread_rng().gen_range(0..=999999);
            while randomrng % 7 == 0 {
                randomrng = rand::thread_rng().gen_range(0..=999999);
            }
            let randomrng = format!("{:06}", randomrng);
            let randomrng2 = format!("{:05}", rand::thread_rng().gen_range(1..=99999));

            println!("{}{}-OEM-0{}-{}", day, year, randomrng, randomrng2);
        } else {
            let mut randomrng = rand::thread_rng().gen_range(0..=999);
            while randomrng % 7 == 0 && randomrng != 333 && randomrng != 444 && randomrng != 555 && randomrng != 666 && randomrng != 777 && randomrng != 888 && randomrng != 999 {
                randomrng = rand::thread_rng().gen_range(0..=999);
            }
            let randomrng = format!("{:03}", randomrng);
            let mut randomrng2 = rand::thread_rng().gen_range(0..=9999999);
            while randomrng2 % 7 == 0 {
                randomrng2 = rand::thread_rng().gen_range(0..=9999999);
            }
            let randomrng2 = format!("{:07}", randomrng2);
            println!("{}-{}", randomrng, randomrng2);
        }
    }
}