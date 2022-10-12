use rand::Rng;
use std::time::Instant;

fn update_all(input: &mut [f64], base: &f64) {
    input.iter_mut()
         .for_each(|num| *num = base * 10.0_f64.powf(*num));
}

fn gen_numbers() -> Vec<f64> {
    (0..10_000_000)
        //.into_par_iter()
        .map(|_| {
            rand::thread_rng().gen()
        }).collect()
}

fn main() {
    let mut from = Instant::now();
    let mut a_few_numbers = gen_numbers();
    println!("Number generation took {} us", (Instant::now() - from).as_micros());

    from = Instant::now();
    update_all(&mut a_few_numbers, &6.0);
    println!("Update of all numbers took {} us", (Instant::now() - from).as_micros());
}
