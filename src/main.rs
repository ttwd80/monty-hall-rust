use rand::Rng;

const COUNT_PER_SET: i32 = 1000;
const TOTAL_SET: i32 = 10;

fn main() {
    for _n in 0..TOTAL_SET {
        process_set(rand::random());
        println!("***");
    }
}
fn process_change() {
    let mut change_count = 0;
    for _i in 0..COUNT_PER_SET {
        change_count += simulate(false);
    }
    println!(
        "If you change, chances of winning are {:.3}%",
        (change_count as f64 / COUNT_PER_SET as f64 * 100.00)
    );
}
fn process_stay() {
    let mut stay_count = 0;
    for _i in 0..COUNT_PER_SET {
        stay_count += simulate(true);
    }
    println!(
        "If you stay, chances of winning are {:.3}%",
        (stay_count as f64 / COUNT_PER_SET as f64 * 100.00)
    );
}

fn process_set(stay_first: bool) {
    if stay_first {
        process_stay();
        process_change();
    } else {
        process_change();
        process_stay();
    }
}

fn simulate(stay: bool) -> i32 {
    let mut rng = rand::thread_rng();

    let car_location = rng.gen_range(0..3);
    //println!("We have Door#1, Door#2, Door#3");
    //println!("The car is behind Door#{}", car_location + 1);

    let picked_location = rng.gen_range(0..3);
    //println!("We have chosen Door#{}", picked_location + 1);

    let pick: char;

    if car_location == picked_location {
        //println!("(Good pick!)");
        pick = 'G';
    } else {
        //println!("(Bad pick!)");
        pick = 'B';
    }

    //0 -> 1,2: car_location + 1 (2), car_location + 2 (3)
    //1 -> 0,2: car_location + 1 (3), (car_location + 2) % 3 (1)
    //2 -> 0,1: (car_location + 1) % 3 (1), (car_location + 2) % 3 (1)
    let bad_doors: [i32; 2] = [(car_location + 1) % 3, (car_location + 2) % 3];

    let reveal_index = rng.gen_range(0..2);
    let mut reveal = bad_doors[reveal_index];
    if reveal == picked_location {
        reveal = bad_doors[(reveal_index + 1) % 2]
    }

    //println!("Reveal: Door#{} is bad!", reveal + 1);

    if stay && pick == 'G' {
        return 1;
    }
    if !stay && pick == 'B' {
        return 1;
    }
    return 0;
}
