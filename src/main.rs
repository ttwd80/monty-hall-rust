use rand::Rng;

const COUNT_PER_SET: i32 = 1000;

fn main() {
    let mut count = 0;
    for _i in 0..COUNT_PER_SET {
        count += simulate(true);
    }
    println!("If you stay, chances of winning are {:.3}%", (count as f64 / COUNT_PER_SET as f64 * 100.00));
}

fn simulate(stay: bool) -> i32 {
    let mut rng = rand::thread_rng();

    let car_location = rng.gen_range(0..3);
    println!("We have Door#1, Door#2, Door#3");
    println!("The car is behind Door#{}", car_location + 1);

    let picked_location = rng.gen_range(0..3);
    println!("We have chosen Door#{}", picked_location + 1);

    let pick: char;

    if car_location == picked_location {
        println!("(Good pick!)");
        pick = 'G';
    } else {
        println!("(Bad pick!)");
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

    println!("Reveal: Door#{} is bad!", reveal + 1);

    if stay && pick == 'G' {
        return 1;
    }
    if !stay && pick == 'B' {
        return 1;
    }
    return 0;
}
