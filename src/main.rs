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

fn inbuilt_count_ones(num: NumType) -> u8 {
    num.count_ones() as u8
}

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
fn count_ones_inplace_4(num: NumType) -> u8 {
    let num = num as u64;
    //note, numtype can only be up to u32
    //set ones in num = set ones in left half of num + set ones in right half of num
    //we calculate those at the same time using instruction over the whole number rather then just checking one bit at a time
    let masks = [0x55555555, 0x33333333, 0x0f0f0f0f, 0x00ff00ff, 0x0000ffff];
    let mut ret = num - ((num >> 1) & masks[0]);
    ret = ((ret >> (1 << 1)) & masks[1]) + (ret & masks[1]);
    ret = ((ret >> (1 << 2)) + ret) & masks[2];
    ret = ((ret >> (1 << 3)) + ret) & masks[3];
    ret = ((ret >> (1 << 4)) + ret) & masks[4];
    ret as u8
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
    let mut ret = 0;
    for i in (0..DIGIT_COUNT).step_by(8) {
        ret += count_ones_memo_small(((num & (255 << i)) >> i) as u8)
    }
    ret
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
    let time_count_inplace_naive = measure_execution_time(nums, count_ones_inplace_naive);
    let time_count_inplace = measure_execution_time(nums, count_ones_inplace_0);
    let time_count_inplace_1 = measure_execution_time(nums, count_ones_inplace_1);
    let time_count_inplace_2 = measure_execution_time(nums, count_ones_inplace_2);
    let time_count_inplace_3 = measure_execution_time(nums, count_ones_inplace_3);
    let time_count_inplace_4 = measure_execution_time(nums, count_ones_inplace_4);
    let time_count_memo = measure_execution_time(nums, count_ones_memo_0);
    let time_count_memo_1 = measure_execution_time(nums, count_ones_memo_1);
    let time_inbuilt = measure_execution_time(nums, inbuilt_count_ones);

    println!("Count Methods Execution Times:");
    println!("  Inplace Naive:          {:?}", time_count_inplace_naive);
    println!("  Inplace 0:              {:?}", time_count_inplace);
    println!("  Inplace 1:              {:?}", time_count_inplace_1);
    println!("  Inplace 2:              {:?}", time_count_inplace_2);
    println!("  Inplace 3:              {:?}", time_count_inplace_3);
    println!("  Inplace 4:              {:?}", time_count_inplace_4);
    println!("  Memoization:            {:?}", time_count_memo);
    println!("  Memoization 1:          {:?}", time_count_memo_1);
    println!("  Inbuilt Function:       {:?}", time_inbuilt);
}

lazy_static! {
    //init lookup arrays, the arrays[i] = amount_of_set_bits(i)

    //ARR can be large (around 4 billion for u32 for example) so we save it to the heap
    //
    static ref ARR: Vec<u8> = {
        println!("started init");
        let start = Instant::now();
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
        println!("finished init, {:?}", start.elapsed());
        arr
    };
}

//this is the same array as the const one, we just messure the time to create it in here.
fn mesure_time_to_create_small_arr() -> [u8; 1 << 8] {
    let arr: [u8; 1 << 8] = {
        println!("started init small");
        let start = Instant::now();
        let mut arr = [0; 1 << (8)];
        let mut next_pow = 2;
        arr[0] = 0;
        arr[1] = 1;
        for _ in 0..8 - 1 {
            for i in next_pow..next_pow << 1 {
                arr[i] = 1 + arr[i - next_pow];
            }
            next_pow <<= 1;
        }
        println!("finished init small, {:?}", start.elapsed());
        arr
    };
    arr
}

fn main() {
    mesure_time_to_create_small_arr();

    let tmp = get_rand_arr(100);
    let ans = tmp
        .iter()
        .map(|i| count_ones_inplace_0(*i) as usize)
        .sum::<usize>();
    assert!(
        ans == tmp
            .iter()
            .map(|i| count_ones_inplace_naive(*i) as usize)
            .sum()
    );
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_naive(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_1(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_2(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_3(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_inplace_4(*i) as usize).sum());

    assert!(ans == tmp.iter().map(|i| count_ones_memo_0(*i) as usize).sum());
    assert!(ans == tmp.iter().map(|i| count_ones_memo_1(*i) as usize).sum());


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
