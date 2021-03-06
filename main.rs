use std::io;

mod common;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;

fn main() -> io::Result<()> {
    let selection = std::env::args().nth(1).expect("no pattern given");

    match selection.as_ref() {
        "0101" => day_01::pt1::run(),
        "0102" => day_01::pt2::run(),
        "0201" => day_02::pt1::run(),
        "0202" => day_02::pt2::run(),
        "0301" => day_03::pt1::run(),
        "0302" => day_03::pt2::run(),
        "0401" => day_04::pt1::run(),
        "0402" => day_04::pt2::run(),
        "0501" => day_05::pt2::run(),
        _ => {
            println!("Not a valid selection : {}", selection);
            Ok(())
        }
    }
}
