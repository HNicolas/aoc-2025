mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() {
    let timer = std::time::Instant::now();
    day01::run();
    day02::run();
    day03::run();
    day04::run();
    day05::run();
    println!("{}ms ellapsed", timer.elapsed().as_millis());
}
