
use indicatif::{ProgressBar, ProgressStyle};
use std::num::Wrapping;

fn penguinrandom(seed: u64) -> u64 {
    let mut t:u64 = 0;
    let s:u64 = seed / 100;
    if (seed ^ 23) % 2 == 0 {
        t ^= s >> (13 ^ t);
        t ^= 448;
        t ^= s << 1;
    } else {
        t ^= s << (11 ^ t);
        t ^= 127;
        t ^= s >> 3;
    }
    (t.wrapping_mul(s).wrapping_div(11)) % (seed.wrapping_mul(19)) + (seed % 3) + (seed % 7) + (seed % 5) +
        (seed % 13) + (seed % 17)
}

fn u32_prng(seed: u32) -> u32 {
    let mut result = 0;
    for _ in 0..32 {
        let new_seed = penguinrandom(seed as u64);
        result = (result << 1) + new_seed % 2;
    }
    result as u32
}

fn u32_dprng(seed: u32) -> u32 {
    let mut result = 0;
    for _ in 0..8 {
        let new_seed = penguinrandom(seed as u64);
        result = result * 10 + new_seed % 10;
    }
    result as u32
}
fn u64_dprng(seed: u64) -> u64 {
    let mut result = 0;
    let mut new_seed = penguinrandom(seed);
    for _ in 0..18 {
        new_seed = penguinrandom(new_seed);
        result = result * 10 + new_seed % 10;
    }
    result
}
fn sort<A, T>(mut array: A) -> A
where
    A: AsMut<[T]>,
    T: Ord,
{
    let slice = array.as_mut();
    slice.sort();

    array
}
fn main() {
    let seed = 1724397362; // Replace with your desired seed value
    let mut v = [0; 10];
    let mut b = [0; 2];
    let mut d = [0; 6];

    println!("A new random function?");
    println!("Better categorize it.");
    println!("1. Fairness (Distribution evenness)");
    println!("Running on 2**31-1 numbers.");

    let mut new_seed:u64 = penguinrandom(seed) ;
    //let B: u32 = 2147483647;
    const B:u32 = 1000001;
    let mut e = vec![];
    for i in 0..B {
        new_seed = penguinrandom(new_seed);
        v[(new_seed % 10) as usize] += 1;
        b[(new_seed % 2) as usize] += 1;
        d[(new_seed % 6) as usize] += 1;
        e.push(new_seed%1000);
    }

    println!("1.a Decimal Generation");
    println!("Expected outcomes: {}", B / 10);
    let mut chisq:f64 = 0_f64;
    let mut S:f64 = 0_f64;
    for (k, count) in v.iter().enumerate() {
        println!("{}: {}", k, count);
        chisq += (*count as f64/B as f64-0.1)*(*count as f64/B as f64-0.1)/(0.1);
        S+= (*count as f64/B as f64)*(*count as f64/B as f64).log10() as f64;
    }
    println!("Chi-Squared Value: {:?}", chisq);
    println!("Entropy (Base 10): {:?}", -S);
    chisq = 0.;
    S = 0.;
    println!("1.b Binary Generation");
    println!("Expected outcomes: {}", B / 2);
    for (k, count) in b.iter().enumerate() {
        println!("{}: {}", k, count);
        chisq += (*count as f64 /B as f64-0.5)*(*count as f64/B as f64-0.5)/(0.5);
        S+= (*count as f64 /B as f64)*(*count as f64 /B as f64).log2() as f64;
    }
    println!("Chi-Squared Value: {:?}", chisq);
    println!("Entropy (Base 2): {:?}", -S);
    chisq = 0.;
    S = 0.;
    println!("1.c Dice Generation");
    println!("Expected outcomes: {}", B / 6);
    for (k, count) in d.iter().enumerate() {
        println!("{}: {}", k, count);
        chisq += (*count as f64/B as f64-(1.0/6.0))*(*count as f64/B as f64-(1.0/6.0))/((1.0/6.0));
        S+= (*count as f64/B as f64 )*(*count as f64/B as f64).log2() as f64;
    }
    println!("Chi-Squared Value: {:?}", chisq);
    println!("Entropy (Base 2): {:?}", -S);
    println!("2. Monte-Carlo Pi test");
    let mut inside = 0;
    let interval = 5000;
    let mut seed1 = penguinrandom(seed);
    let mut new_seed = penguinrandom(seed1);
    println!("Generating {} pairs", interval * interval);
    for _ in 0..interval * interval {
        seed1 = penguinrandom(new_seed);
        new_seed = penguinrandom(seed1);
        let x = (seed1 % (interval + 1)) as f64 / interval as f64;
        let y = (new_seed % (interval + 1)) as f64 / interval as f64;
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }

    println!("Dots counted inside the quater-circle: {}", inside);
    let pi_estimate:f64 = (inside as f64)/ (interval/2).pow(2) as f64;
    println!("Estimated value of pi: {}",pi_estimate);
    println!("3. Runs Test, using generated values from Test 1 mod 100.");
    let e_sort = sort(e.clone());
    let median = (e_sort[(B/2) as usize] as f64 + e_sort[((B+1)/2) as usize] as f64)/2.;

    let mut runs:u64 = 0;
    let mut i:usize = 0;
    let mut Nplus:u64 = 0;
    let mut Nminus:u64 = 0;
    println!("Calculated median: {}", median);

    while i<(e.len()-1){
        
            if (e[i] as f64) > median { while ((e[i] as f64) > median) && i<(e.len()-1){
                i+=1;
                Nplus += 1;
            }
            runs+=1;
        }

            if (e[i] as f64) < median {while ((e[i] as f64) < median) && i<(e.len()-1){
                i+=1;
                Nminus += 1;
            }
            runs+=1;
        }
            
            if (e[i] as f64) == median {while ((e[i] as f64) == median) && i<(e.len()-1){
                i+=1;
            }
            //runs+=1;
        }
    }
    let mu = 2.*Nplus as f64*Nminus as f64 / e.len() as f64 + 1.0;
    let var = (mu-1.)*(mu-2.)/(e.len() as f64-1.);
    let e_runs = 2*Nplus*Nminus/e.len() as u64 + 1;
    println!("Estimated mu: {}", mu);
    println!("Estimated var: {}", var);
    println!("Number of +ve values: {}", Nplus);
    println!("Number of -ve values: {}", Nminus);
    println!("Number of expected runs: {}",e_runs);
    println!("Number of runs: {}", runs);
    println!("4. Craps Games");
    println!("Simulating 200,000 games");
    const games:u32 = 200000;
    let mut dice1 = penguinrandom(seed)%6;
    let mut dice2 = penguinrandom(dice1)%6;
    let throw_count = 0;
    for _ in 0..games{
        let dicetot = dice1 + dice2 + 2;
        let mut dice1 = penguinrandom(seed)%6;
        let mut dice2 = penguinrandom(dice1)%6;

    }
}
