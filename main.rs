#![feature(linked_list_remove)] 
mod cache;
use cache::Cache;
use std::fs::File;
use std::io::Read;
use std::time::Instant;
use std::env;
use std::process;

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() != 4 && args.len() != 5 {
        print_usage();
    }
    match args[1].as_str() {
        "1" => {
            if args.len() == 4 {
                let start = Instant::now();
                single_cache(args[2].as_str().parse::<usize>().unwrap(),args[3].as_str());
                let elapsed = start.elapsed();
                println!("Program execution took {}.{:0<3} seconds", elapsed.as_secs(), elapsed.subsec_millis());
            } else {
                print_usage();
            }
        },
        "2" => {
            if args.len() == 5 {
                let start = Instant::now();
                double_cache(args[2].as_str().parse::<usize>().unwrap(), args[3].as_str().parse::<usize>().unwrap(), args[4].as_str());
                let elapsed = start.elapsed();
                println!("Program execution took {}.{:0<3} seconds", elapsed.as_secs(), elapsed.subsec_millis());
            } else {
                print_usage();
            }
        },
        _ => {print_usage();}
    }
}

fn single_cache(cache_size: usize, input_file: &str ) {
    let mut cache = Cache::<&str>::new(cache_size);
    let mut file_text = String::new();
    let mut file = File::open(input_file).expect("Unable to open");
    file.read_to_string(&mut file_text);
    let mut token_iter = file_text.split_whitespace();
    let mut cache_hits: i32 = 0;
    let mut cache_references: i32 = 0;
    while let Some(token) = token_iter.next() {
        cache_references += 1;
        if let Some(_) = cache.get_object(token) {
            cache_hits += 1;
            cache.move_to_top(token);
        } else {
            cache.add_object(token);
        }
    }
    let hit_ratio: f64 = cache_hits as f64/ cache_references as f64;
    println!("The number of global references: {}", cache_references);
	println!("The number of global cache hits: {}", cache_hits);
	println!("The global hit ratio: {}", hit_ratio);
	println!("----------------------------------------------------------------\n");
}

fn double_cache(first_cache_size: usize, second_cache_size: usize, input_file: &str ) {
    let mut cache1 = Cache::<&str>::new(first_cache_size);
    let mut cache2 = Cache::<&str>::new(second_cache_size);
    let mut file_text = String::new();
    let mut file = File::open(input_file).expect("Unable to open");
    file.read_to_string(&mut file_text);
    let mut token_iter = file_text.split_whitespace();
    let mut cache1_hits: i32 = 0;
    let mut cache1_references: i32 = 0;
    let mut cache2_hits: i32 = 0;
    let mut cache2_references: i32 = 0;
    while let Some(token) = token_iter.next() {
        cache1_references += 1;
        if let Some(_) = cache1.get_object(token) {
            cache1_hits += 1;
            cache1.move_to_top(token);
            cache2.move_to_top(token);
        } else {
            cache2_references += 1;
            if let Some(_) = cache2.get_object(token) {
                cache2_hits += 1;
                cache2.move_to_top(token);
                cache1.add_object(token);
            } else {
                cache1.move_to_top(token);
                cache2.move_to_top(token);
            }
        }
    }
    let hit1_ratio: f64 = cache1_hits as f64/ cache1_references as f64;
    let hit2_ratio: f64 = cache2_hits as f64/ cache2_references as f64;
    let global_hits = cache1_hits + cache2_hits;
    let global_ratio: f64 = global_hits as f64/ cache1_references as f64;
    println!("The number of global references: {}", cache1_references);
	println!("The number of global cache hits: {}", global_hits);
	println!("The global hit ratio: {}", global_ratio);
	println!("----------------------------------------------------------------\n");
    println!("The number of 1st level references: {}", cache1_references);
	println!("The number of 1st level cache hits: {}", cache1_hits);
	println!("The 1st level cache hit ratio {}", hit1_ratio);
	println!("----------------------------------------------------------------\n");
    println!("The number of 2nd level references: {}", cache2_references);
	println!("The number of 2nd level cache hits: {}", cache2_hits);
	println!("The 2nd level cache hit ratio {}", hit2_ratio);
	println!("----------------------------------------------------------------\n");
}

fn print_usage() {
    println!("Incorrect usage!");
    println!("Usage: ./main 1 <cache size> <input textfile name>");
    println!("Usage: ./main 2 <1st level cache size> <2nd level cache size> <input textfile name>");
    process::exit(1);
}