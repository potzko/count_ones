use core::num;
use lazy_static::lazy_static;
use rand::Rng;
use std::{hint::black_box, time::Instant};

//if you try to run with with a u64, you will crash, better avoid...
type NumType = u32;
const VEC_SIZE: usize = 100000000;
const DIGIT_COUNT: usize = NumType::MAX.count_ones() as usize;

fn get_rand_arr(length: usize) -> Vec<NumType> {
    //returns an array of length N filled with random numbers
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(length);
    for _ in 0..length {
        vec.push(rng.gen::<NumType>());
    }
    vec
}

//the small arr is very small so we can let the compiler know to use it as a const
const ARR_SMALL: [u8; 1 << 8] = {
    let mut arr = [0; 1 << (8)];
    let mut next_pow = 2;
    arr[0] = 0;
    arr[1] = 1;
    let mut step = 7;
    while step > 0 {
        step -= 1;

        let mut i = next_pow;
        while i < (next_pow << 1) {
            arr[i] = 1 + arr[i - next_pow];
            i += 1;
        }
        next_pow <<= 1;
    }
    arr
};

#[no_mangle]
fn inbuilt_count_ones(num: NumType) -> u8 {
    num.count_ones() as u8
}

#[no_mangle]
fn count_ones_inplace_naive(num: NumType) -> u8 {
    //naive implemantation
    let mut num = num;
    let mut ret: u8 = 0;
    while num != 0 {
        ret += if num % 2 == 1 { 1 } else { 0 };
        num /= 2;
    }
    ret
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
    //should be slower because of the branch
    let mut ret: u8 = 0;
    for i in 0..DIGIT_COUNT {
        ret += ((num & (1 << i)) >> i) as u8;
        if (1 << i) > num {
            return ret;
        }
    }
    ret
}

#[no_mangle]
fn count_ones_inplace_2(num: NumType) -> u8 {
    //small trick, num &= num - 1 will always remove the first one in the number
    //we do this as many times as there are ones
    let mut num = num;
    let mut ret: u8 = 0;
    while num != 0 {
        ret += 1;
        num &= num - 1;
    }
    ret
}

//implemented based on the version on https://graphics.stanford.edu/~seander/bithacks.html#CountBitsSetKernighan
fn count_ones_inplace_3(num: NumType) -> u8 {
    //note, numtype can only be up to u32
    //literal dark magic voodoo code
    let num = num as u64;
    let mut ret = (((num & 0xfff) * 0x1001001001001_u64) & 0x84210842108421_u64) % 0x1f;
    ret += ((((num & 0xfff000) >> 12) * 0x1001001001001_u64) & 0x84210842108421_u64) % 0x1f;
    ret += (((num >> 24) * 0x1001001001001_u64) & 0x84210842108421_u64) % 0x1f;
    ret as u8
}

//implemented based on the version on https://graphics.stanford.edu/~seander/bithacks.html#CountBitsSetKernighan
#[no_mangle]
fn count_ones_inplace_4(num: NumType) -> u8 {
    let num = num as u64;
    //note, numtype can only be up to u32
    //set ones in num = set ones in left half of num + set ones in right half of num
    //we calculate those at the same time using instruction over the whole number rather then just checking one bit at a time
    let masks = [0x55555555, 0x33333333, 0x0f0f0f0f, 0x00ff00ff, 0x0000ffff];
    let mut ret = num - ((num >> 1) & masks[0]);
    ret = ((ret >> (2)) & masks[1]) + (ret & masks[1]);
    ret = ((ret >> (4)) + ret) & masks[2];
    ret = ((ret >> (8)) + ret) & masks[3];
    ret = ((ret >> (16)) + ret) & masks[4];
    ret as u8
}

#[no_mangle]
fn count_ones_inplace_5(num: NumType) -> u8 {
    let mut num = num;
    //note, numtype can only be up to u32
    //set ones in num = set ones in left half of num + set ones in right half of num
    //we calculate those at the same time using instruction over the whole number rather then just checking one bit at a time
    let masks = [0x55555555, 0x33333333, 0x0f0f0f0f, 0x00ff00ff, 0x0000ffff];
    let inverse_masks = [0xaaaaaaaa, 0xcccccccc, 0xf0f0f0f0, 0xff00ff00, 0xffff0000];
    num = (num & masks[0]) + ((num & inverse_masks[0]) >> 1);
    num = (num & masks[1]) + ((num & inverse_masks[1]) >> 2);
    num = (num & masks[2]) + ((num & inverse_masks[2]) >> 4);
    num = (num & masks[3]) + ((num & inverse_masks[3]) >> 8);
    num = (num & masks[4]) + ((num & inverse_masks[4]) >> 16);
    num as u8
}

fn count_ones_memo_0(num: NumType) -> u8 {
    //array the size of the whole number type
    ARR[num as usize]
}

fn count_ones_memo_small(num: u8) -> u8 {
    //assumes an 8 sized number
    ARR_SMALL[num as usize]
}

fn count_ones_memo_1(num: NumType) -> u8 {
    //break the number to sections of size 8, then sum them to avoid having the huge array
    num.to_ne_bytes()
        .iter()
        .map(|i| count_ones_memo_small(*i))
        .sum()
}

fn count_ones_memo_2(num: NumType) -> u8 {
    //note, numtype can only be up to u64
    //count_ones_memo_1 with some nice unrolling, might be faster because im showing transative addition where as .sum() can't allways use it?
    let num = (num as u64).to_ne_bytes();
    let a = count_ones_memo_small(num[0]) + count_ones_memo_small(num[1]);
    let b = count_ones_memo_small(num[2]) + count_ones_memo_small(num[3]);
    let c = count_ones_memo_small(num[4]) + count_ones_memo_small(num[5]);
    let d = count_ones_memo_small(num[6]) + count_ones_memo_small(num[7]);
    a + b + c + d
}

fn measure_execution_time<F>(nums: &[NumType], count_fn: F) -> std::time::Duration
where
    F: Fn(NumType) -> u8,
{
    let start = std::time::Instant::now();
    for &i in nums {
        black_box(count_fn(i));
    }
    start.elapsed()
}

fn check_correct<F>(nums: &[NumType], count_fn: F) -> bool
where
    F: Fn(NumType) -> u8,
{
    nums.iter()
        .map(|i| inbuilt_count_ones(*i))
        .collect::<Vec<u8>>()
        == nums.iter().map(|i| count_fn(*i)).collect::<Vec<u8>>()
}
fn run_vec(nums: &[NumType]) {
    let time_count_inplace_naive = measure_execution_time(nums, count_ones_inplace_naive);
    let time_count_inplace = measure_execution_time(nums, count_ones_inplace_0);
    let time_count_inplace_1 = measure_execution_time(nums, count_ones_inplace_1);
    let time_count_inplace_2 = measure_execution_time(nums, count_ones_inplace_2);
    let time_count_inplace_3 = measure_execution_time(nums, count_ones_inplace_3);
    let time_count_inplace_4 = measure_execution_time(nums, count_ones_inplace_4);
    let time_count_inplace_5 = measure_execution_time(nums, count_ones_inplace_5);
    let time_count_memo = measure_execution_time(nums, count_ones_memo_0);
    let time_count_memo_1 = measure_execution_time(nums, count_ones_memo_1);
    let time_count_memo_2 = measure_execution_time(nums, count_ones_memo_2);
    let time_inbuilt = measure_execution_time(nums, inbuilt_count_ones);

    println!("Count Methods Execution Times:");
    println!(
        "  Inplace Naive:          {:?}, {}",
        time_count_inplace_naive,
        check_correct(nums, count_ones_inplace_naive)
    );
    println!(
        "  Inplace 0:              {:?}, {}",
        time_count_inplace,
        check_correct(nums, count_ones_inplace_0)
    );
    println!(
        "  Inplace 1:              {:?}, {}",
        time_count_inplace_1,
        check_correct(nums, count_ones_inplace_1)
    );
    println!(
        "  Inplace 2:              {:?}, {}",
        time_count_inplace_2,
        check_correct(nums, count_ones_inplace_2)
    );
    println!(
        "  Inplace 3:              {:?}, {}",
        time_count_inplace_3,
        check_correct(nums, count_ones_inplace_3)
    );
    println!(
        "  Inplace 4:              {:?}, {}",
        time_count_inplace_4,
        check_correct(nums, count_ones_inplace_4)
    );
    println!(
        "  Inplace 5:              {:?}, {}",
        time_count_inplace_5,
        check_correct(nums, count_ones_inplace_5)
    );
    println!(
        "  Memoization:            {:?}, {}",
        time_count_memo,
        check_correct(nums, count_ones_memo_0)
    );
    println!(
        "  Memoization 1:          {:?}, {}",
        time_count_memo_1,
        check_correct(nums, count_ones_memo_1)
    );
    println!(
        "  Memoization 2:          {:?}, {}",
        time_count_memo_2,
        check_correct(nums, count_ones_memo_2)
    );
    println!(
        "  Inbuilt Function:       {:?}, {}",
        time_inbuilt,
        check_correct(nums, inbuilt_count_ones)
    );
}

lazy_static! {
    //init lookup arrays, the arrays[i] = amount_of_set_bits(i)
    //ARR can be large (around 4 billion for u32 for example) so we save it to the heap
    static ref ARR: Vec<u8> = {
        let mut arr = vec![0; 1 << (DIGIT_COUNT)];
        let mut next_pow = 2;
        arr[0] = 0;
        arr[1] = 1;
        for _ in 0..DIGIT_COUNT - 1 {
            for i in next_pow..next_pow << 1 {
                arr[i] = 1 + arr[i - next_pow];
            }
            next_pow <<= 1;
        }
        arr
    };
}

//this is the same array as the const one, we just messure the time to create it in here.
fn mesure_time_to_create_memoization_array_mine() -> Vec<u8> {
    let mut arr = vec![0; 1 << (DIGIT_COUNT)];
    println!("started init");
    let start = Instant::now();
    let mut next_pow = 2;
    arr[0] = 0;
    arr[1] = 1;
    for _ in 0..DIGIT_COUNT - 1 {
        for i in next_pow..next_pow << 1 {
            arr[i] = 1 + arr[i - next_pow];
        }
        next_pow <<= 1;
    }
    println!("finished init, {:?}", start.elapsed());
    arr
}

fn mesure_time_to_create_memoization_array_inbuilt() -> Vec<u8> {
    let mut arr: Vec<u8> = vec![0; 1 << (DIGIT_COUNT)];
    println!("started init");
    let start = Instant::now();
    for i in 0..arr.len() {
        arr[i] = (i as NumType).count_ones() as u8;
    }
    println!("finished init, {:?}", start.elapsed());
    arr
}

fn main() {
    println!("started checking mine");
    black_box(mesure_time_to_create_memoization_array_mine());
    println!("started checking std");
    black_box(mesure_time_to_create_memoization_array_inbuilt());

    let tmp = get_rand_arr(100);
    let ans = tmp
        .iter()
        .map(|i| inbuilt_count_ones(*i) as usize)
        .sum::<usize>();

    assert!(
        ans == tmp
            .iter()
            .map(|i| count_ones_inplace_naive(*i) as usize)
            .sum()
    );
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_0(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_1(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_2(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_3(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_4(*i) as usize).sum());

    assert!(ans == tmp.iter().map(|i| count_ones_memo_0(*i) as usize).sum());
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
    let arr: Vec<NumType> = (0..VEC_SIZE)
        .map(|i| NumType::MAX - (i % 255) as NumType)
        .collect();
    println!("finished array gen");
    println!("large numbers:");
    run_vec(&arr);
    let arr: Vec<NumType> = (0..VEC_SIZE).map(|i| (i % 255) as NumType).collect();
    println!("finished array gen");
    println!("small numbers:");
    run_vec(&arr);
}
