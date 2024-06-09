mod config;
mod position_ef1;
mod utils;

use crate::config::Config;
use crate::position_ef1::is_position_ef1;
use crate::utils::to_base;
use rand::Rng;
use std::env;
use std::process;


fn main() {

    // inputs: config.num_agents, config.num_items, config.max_utility
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(c) => {
            println!("Number of agents: {}", c.num_agents);
            println!("Number of items: {}", c.num_items);
            println!("Max value of utility: {}", c.max_utility);
            c
        },
        Err(err) => {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    // 10 tries
    let num_try: usize = 10;
    for _ in 0..num_try {

        let mut utility: Vec<Vec<i32>> = vec![Vec::new(); config.num_agents];
        for i in 0..config.num_agents {
            for _ in 0..config.num_items {
                // choose a value from gen_range() uniformly as random
                let rand_num:i32 = rand::thread_rng().gen_range(1..=config.max_utility);
                utility[i].push(rand_num);
            }
        }

        let mut nsw_allocations: Vec<usize> = Vec::new();
        let mut max_val: i32 = -1;

        // allocate
        for i in 0..(config.num_agents).pow(config.num_items as u32) as usize {
            let allocation_val = create_allocation_digit(i, config.num_agents, config.num_items, &utility).1;
            let product: i32 = allocation_val.iter().product();
            // println!("Product of allocation_val: {}", product);
            if product > max_val {
                max_val = product;
                nsw_allocations = vec![i];
            }
            else if product == max_val {
                nsw_allocations.push(i);
            }
        }

        println!("The number of nsw_allocations is {}",nsw_allocations.len());
        println!("nsw_allocations: {:?}", nsw_allocations);

        compare_pairs_nsw(nsw_allocations,config.num_agents, config.num_items, &utility);
    }
}



// Compare every pairs of two nsw allocations with respect to position envy_free up to one
fn compare_pairs_nsw(
        nsw_allocations: Vec<usize>, 
        num_agents: usize,
        num_items: usize,
        utility: &Vec<Vec<i32>>
    ) {

    if nsw_allocations.len() > 1 {
        for id_1 in &nsw_allocations {
            for id_2 in &nsw_allocations {
                if id_1 != id_2 {
                    let (allocation_1, allocation_val_1)= create_allocation_digit(*id_1, num_agents, num_items, &utility);
                    let (allocation_2, allocation_val_2) = create_allocation_digit(*id_2, num_agents, num_items, &utility);
                    
                    let product_1: i32 = allocation_val_1.iter().product();
                    let product_2: i32 = allocation_val_2.iter().product();
                    assert!(product_1 == product_2);

                    if !is_position_ef1(&utility, &allocation_1, &allocation_2) {
                        println!("utility = {:?}, allocation_1 = {:?}, allocation_2 = {:?}", &utility, &allocation_1, &allocation_2);
                        panic!("NOT Position EF1");
                    }
                }
            }
        }
        println!("All pairs are Position EF1");
    }
}



fn create_allocation_digit(
        id: usize, 
        num_agents: usize, 
        num_items: usize, 
        utility: &Vec<Vec<i32>>
    ) -> (Vec<Vec<usize>>, Vec<i32>) {

    let mut allocation: Vec<Vec<usize>> = vec![Vec::new(); num_agents];
    let mut allocation_val: Vec<i32> = vec![0; num_agents];
    // println!("id= {}",id);
    let mut allocation_digit = to_base(id as u64, num_agents as u32);

    // add 0 to the front of the allocation_digit to make its length equal to num_items
    while allocation_digit.len() < num_items {
        allocation_digit.insert(0, '0');
    }

    for j in 0..num_items {
        // println!("allocation_digit: {:?}", allocation_digit);
        let digit = allocation_digit[j].to_digit(36).unwrap() as usize;

        allocation_val[digit] += utility[digit][j];
        allocation[digit].push(j);
    }
    (allocation, allocation_val) 
}

