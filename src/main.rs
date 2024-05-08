use lazy_static::lazy_static;
use rand::Rng;
use std::{hint::black_box, sync::Once, time::Instant};

//if you try to run with with a u64, you will crash, better avoid...
type NumType = u16;
const DIGIT_COUNT: usize = 16;
const VEC_SIZE: usize = 100000000;

pub fn get_rand_arr(length: usize) -> Vec<NumType> {
    //returns an array of length N filled with random numbers
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
        vec.push(rng.gen::<NumType>() as NumType);
    }
    vec
}

fn count_ones_inplace_0(num: NumType) -> u8 {
    //my first thought, run over the number adding each 1
    let mut ret: u8 = 0;
    for i in 0..DIGIT_COUNT {
        ret += ((num & (1 << i)) >> i) as u8
    }
    ret
}

fn count_ones_inplace_1(num: NumType) -> u8 {
    //my first thought, with added check for early return
    let mut ret: u8 = 0;
    for i in 0..DIGIT_COUNT {
        ret += ((num & (1 << i)) >> i) as u8;
        if (1 << i) > num {
            return ret;
        }
    }
    ret
}

fn count_ones_inplace_2(num: NumType) -> u8 {
    //small trick, num &= num - 1 will always remove the first 1
    let mut num = num;
    let mut ret: u8 = 0;
    while num != 0 {
        ret += 1;
        num &= num - 1;
    }
    ret
}

fn count_ones_memo(num: NumType) -> u8 {
    //array the size of the whole number type
    ARR[num as usize]
}

fn count_ones_memo_small(num: u8) -> u8 {
    //assumes an 8 sized number
    ARR[num as usize]
}

fn count_ones_memo_1(num: NumType) -> u8 {
    //break the number to sections of size 8, then sum them to avoid having the huge array
    let mut ret = 0;
    for i in (0..DIGIT_COUNT).step_by(8) {
        ret += count_ones_memo_small(((num & (255 << i)) >> i) as u8)
    }
    ret
}

fn run_vec(nums: &[NumType]) {
    let start = std::time::Instant::now();
    for i in nums {
        black_box(count_ones_inplace_0(*i));
    }
    let time_count_inplace = start.elapsed();
    let start = std::time::Instant::now();
    for i in nums {
        black_box(count_ones_inplace_1(*i));
    }
    let time_count_inplace_1 = start.elapsed();
    let start = std::time::Instant::now();
    for i in nums {
        black_box(count_ones_inplace_2(*i));
    }
    let time_count_inplace_2 = start.elapsed();
    let start = std::time::Instant::now();
    for i in nums {
        black_box(count_ones_memo(*i));
    }
    let time_count_memo = start.elapsed();
    let start = std::time::Instant::now();
    for i in nums {
        black_box(count_ones_memo_1(*i));
    }
    let time_count_memo_1 = start.elapsed();
    println!("count_inplace: {time_count_inplace:?}, count_inplace_1: {time_count_inplace_1:?}, count_inplace_2: {time_count_inplace_2:?}, count_memo: {time_count_memo:?}, count_ones_memo_1: {time_count_memo_1:?}")
}

lazy_static! {
    static ref ARR: Vec<u8> = {
        println!("started init");
        let start = Instant::now();
        let mut arr = Vec::with_capacity(1 << DIGIT_COUNT);
        let once = Once::new();
        once.call_once(|| {
            for i in 0..(1_u128 << (DIGIT_COUNT)) {
                arr.push(count_ones_inplace_0(i as NumType));
            }
        });
        println!("finished init, {:?}", start.elapsed());
        arr
    };
    static ref ARR_SMALL: [u8; 256] = {
        println!("started init");
        let start = Instant::now();
        let mut arr = [0; 256];
        let once = Once::new();
        once.call_once(|| {
            for i in 0..(1_u128 << (8)) {
                arr[i as usize] = count_ones_inplace_0(i as NumType);
            }
        });
        println!("finished init, {:?}", start.elapsed());
        arr
    };
}

fn main() {
    let tmp = get_rand_arr(100);
    let ans = tmp
        .iter()
        .map(|i| count_ones_inplace_0(*i) as usize)
        .sum::<usize>();
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_1(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_2(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_memo(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_memo_1(*i) as usize).sum());

    let arr: Vec<NumType> = get_rand_arr(VEC_SIZE);
    println!("finished array gen");
    println!("random array:");
    run_vec(&arr);
    let arr: Vec<NumType> = (0..VEC_SIZE).map(|i| i as NumType).collect();
    println!("finished array gen");
    println!("consecutive array, small:");
    run_vec(&arr);
    let arr: Vec<NumType> = ((NumType::MAX - VEC_SIZE as NumType)..NumType::MAX).map(|i| i as NumType).collect();
    println!("finished array gen");
    println!("consecutive array, large:");
    run_vec(&arr);
}
