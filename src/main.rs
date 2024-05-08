use lazy_static::lazy_static;
use rand::Rng;
use std::{hint::black_box, sync::Once, time::Instant};

//if you try to run with with a u64, you will crash, better avoid...
type NumType = u16;
const DIGIT_COUNT: usize = 16;
const VEC_SIZE: usize = 1000000;

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

//https://graphics.stanford.edu/~seander/bithacks.html#CountBitsSetKernighan
fn count_ones_inplace_3(num: NumType) -> u8 {
    //note, numtype can only be up to u32
    //literal dark voodoo
    let num = num as u64;
    let mut ret = (((num & 0xfff) * 0x1001001001001_u64) & 0x84210842108421_u64) % 0x1f;
    ret += ((((num & 0xfff000) >> 12) * 0x1001001001001_u64) & 0x84210842108421_u64) % 0x1f;
    ret += (((num >> 24) * 0x1001001001001_u64) & 0x84210842108421_u64) % 0x1f;
    ret as u8
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

//https://graphics.stanford.edu/~seander/bithacks.html#CountBitsSetKernighan
fn count_ones_memo_2(num: NumType) -> u8 {
    let num = num as u64;
    //note, numtype can only be up to u32
    //literal dark voodoo
    let masks = [0x55555555, 0x33333333, 0x0f0f0f0f, 0x00ff00ff, 0x0000ffff];
    let mut ret = num - ((num >> 1) & masks[0]);
    ret = ((ret >> (1 << 1)) & masks[1]) + (ret & masks[1]);
    ret = ((ret >> (1 << 2)) + ret) & masks[2]; 
    ret = ((ret >> (1 << 3)) + ret) & masks[3]; 
    ret = ((ret >> (1 << 4)) + ret) & masks[4];
    ret as u8
}

fn measure_execution_time<F, NumType>(nums: &[NumType], count_fn: F) -> std::time::Duration
where
    F: Fn(NumType) -> u8,
    NumType: Copy,
{
    let start = std::time::Instant::now();
    for &i in nums {
        black_box(count_fn(i));
    }
    start.elapsed()
}


fn run_vec(nums: &[NumType]) {
    let time_count_inplace = measure_execution_time(nums, count_ones_inplace_0);
    let time_count_inplace_1 = measure_execution_time(nums, count_ones_inplace_1);
    let time_count_inplace_2 = measure_execution_time(nums, count_ones_inplace_2);
    let time_count_inplace_3 = measure_execution_time(nums, count_ones_inplace_3);
    let time_count_memo = measure_execution_time(nums, count_ones_memo);
    let time_count_memo_1 = measure_execution_time(nums, count_ones_memo_1);
    let time_count_memo_2 = measure_execution_time(nums, count_ones_memo_2);
    
    println!("count_inplace: {:?}, count_inplace_1: {:?}, count_inplace_2: {:?}, count_inplace_3: {:?}, count_memo: {:?}, count_ones_memo_1: {:?}, count_ones_memo_2: {:?}",
             time_count_inplace, time_count_inplace_1, time_count_inplace_2, time_count_inplace_3, time_count_memo, time_count_memo_1, time_count_memo_2);
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
    assert!(ans == tmp.iter().map(|i| count_ones_memo_2(*i) as usize).sum());

    let arr: Vec<NumType> = get_rand_arr(VEC_SIZE);
    println!("finished array gen");
    println!("random array:");
    run_vec(&arr);
    let arr: Vec<NumType> = (0..VEC_SIZE).map(|i| i as NumType).collect();
    println!("finished array gen");
    println!("consecutive array:");
    run_vec(&arr);
    let arr: Vec<NumType> = (0..VEC_SIZE).map(|i| NumType::MAX - (i % 255) as NumType).collect();
    println!("finished array gen");
    println!("large numbers:");
    run_vec(&arr);
    let arr: Vec<NumType> = (0..VEC_SIZE).map(|i| (i % 255) as NumType).collect();
    println!("finished array gen");
    println!("small numbers:");
    run_vec(&arr);
}
