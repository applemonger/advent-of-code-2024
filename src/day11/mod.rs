use aocd::*;
use cached::proc_macro::cached;

#[cached]
fn count_digits(num: u64) -> u32 {
    num.checked_ilog10().unwrap_or(0) + 1
}

#[cached]
fn blink_stone(stone: u64, blinks: u64) -> u64 {
    let mut count = 0;
    if blinks > 0 {
        if stone == 0 {
            count += blink_stone(1, blinks - 1);
        } else if count_digits(stone) % 2 == 0 {
            let n = count_digits(stone);
            let divisor = 10_u64.pow(n / 2);
            count += blink_stone(stone / divisor, blinks - 1);
            count += blink_stone(stone % divisor, blinks - 1);
        } else {
            count += blink_stone(stone * 2024, blinks - 1);
        }
    } else {
        count += 1;
    }
    count
}

#[aocd(2024, 11)]
pub fn solution1() {
    let stones: Vec<u64> = input!().split(' ').map(|n| n.parse().unwrap()).collect();
    submit!(1, stones.iter().map(|x| blink_stone(*x, 25)).sum::<u64>());
}

#[aocd(2024, 11)]
pub fn solution2() {
    let stones: Vec<u64> = input!().split(' ').map(|n| n.parse().unwrap()).collect();
    submit!(2, stones.iter().map(|x| blink_stone(*x, 75)).sum::<u64>());
}
