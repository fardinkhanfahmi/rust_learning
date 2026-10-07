use std::io::{self, Read};

pub fn run() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = it.next().unwrap().parse().unwrap();
        let cows: i32 = it.next().unwrap().parse().unwrap();

        let mut positions: Vec<i32> = Vec::with_capacity(n);

        for _ in 0..n {
            let x: i32 = it.next().unwrap().parse().unwrap();
            positions.push(x);
        }

        positions.sort();

        let mut lo: i32 = 0;
        let mut hi: i32 = 1_000_000_000;

        while hi - lo > 1 {
            let mid = (lo + hi) / 2;

            if can_place_cows(mid, cows, &positions) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        if can_place_cows(hi, cows, &positions) {
            println!("{}", hi);
        } else {
            println!("{}", lo);
        }
    }
}

fn can_place_cows(
    min_dist: i32,
    cows: i32,
    positions: &[i32],
) -> bool {
    let mut last_pos = -1;
    let mut cows_ct = cows;

    for &position in positions {
        if last_pos == -1 || position - last_pos >= min_dist {
            cows_ct -= 1;
            last_pos = position;
        }

        if cows_ct == 0 {
            break;
        }
    }

    cows_ct == 0
}
