use clap::Parser;
use rand::Rng;
use std::time::Instant;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Number of integers in the array
    #[arg(long, default_value_t = 10_000_000)]
    int_size: usize,

    // Number of strings in the array
    #[arg(long, default_value_t = 1_000_000)]
    string_size: usize,
}

fn main() {
    let args = Args::parse();

    // Integers
    let int_data = generate_sorted_int_vec(args.int_size);
    let target_int = rand::thread_rng().gen_range(0..args.int_size as i32);

    println!(
        "🔢 Searching for {target_int} in int array of {} items...",
        args.int_size
    );
    benchmark_search("int", &int_data, &target_int);

    // Strings
    let string_data = generate_sorted_string_vec(args.string_size);
    let string_index = rand::thread_rng().gen_range(0..args.string_size);
    let target_string = string_data[string_index].clone();

    println!(
        "\n🔤 Searching for \"{target_string}\" in string array of {} items...",
        args.string_size
    );
    benchmark_search("string", &string_data, &target_string);
}

// Benchmarking
fn benchmark_search<T: Ord + std::fmt::Debug>(label: &str, data: &[T], target: &T) {
    let now = Instant::now();
    let result_safe = binary_search_safe(data, target);
    let duration_safe = now.elapsed();

    let now = Instant::now();
    let result_unsafe = binary_search_unsafe(data, target);
    let duration_unsafe = now.elapsed();

    println!(
        "✅ Safe {label}:   {:?} (took {:?})",
        result_safe, duration_safe
    );
    println!(
        "⚡ Unsafe {label}: {:?} (took {:?})",
        result_unsafe, duration_unsafe
    );
}

// Safe binary search from standard library
fn binary_search_safe<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = slice.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match slice[mid].cmp(target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    None
}

// Unsafe binary search, inspired by Denis Bazhenov
fn binary_search_unsafe<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    let mut low = 0;
    let mut high = slice.len();
    let ptr = slice.as_ptr();

    while low < high {
        let mid = low + (high - low) / 2;
        let mid_val = unsafe { &*ptr.add(mid) };

        match mid_val.cmp(target) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }

    None
}

// Generate sorted vector of integers
fn generate_sorted_int_vec(size: usize) -> Vec<i32> {
    (0..size as i32).collect()
}

// Generate sorted vector of strings
fn generate_sorted_string_vec(size: usize) -> Vec<String> {
    (0..size).map(|i| format!("item{:07}", i)).collect()
}
