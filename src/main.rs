mod config;
mod position_ef1;
mod utils;

use crate::config::Config;
use crate::position_ef1::is_position_ef1;
//use crate::utils::to_base;
use std::cmp::Ordering;
use std::env;
use std::process;
use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};

fn main() -> Result<()> {
    // inputs: config.num_agents, config.num_items, config.max_utility
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(c) => {
            println!("Number of agents: {}", c.num_agents);
            println!("Number of items: {}", c.num_items);
            println!("Max value of utility: {}", c.max_utility);
            c
        }
        Err(err) => {
            println!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    };

    let max_utility_usize = config.max_utility as usize;
    let total_cases = max_utility_usize.pow((config.num_agents * config.num_items) as u32) as usize;
    println!("total_cases: {}", total_cases);

    println!("total allocation cases: {}", (config.num_agents).pow(config.num_items as u32) as usize);


    let update_frequency = (total_cases / 1000).max(1);
    let pb = ProgressBar::new(total_cases as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{bar:40.cyan/blue}] {msg}")
        .progress_chars("█▓▒░"));
    pb.set_message("進行中...");  

    for case in 0..total_cases {
        let mut utility = vec![vec![0; config.num_items]; config.num_agents];
        let mut temp_case = case;
        
        for agent in 0..config.num_agents {
            for item in 0..config.num_items {
                let val = (temp_case % max_utility_usize + 1) as i32;
                utility[agent][item] = val as i32;
                temp_case /= max_utility_usize;
            }
        }

        let mut nsw_allocations = Vec::new();
        let mut max_val = -1;

        // allocate
        for i in 0..(config.num_agents).pow(config.num_items as u32) as usize {
            let (_, allocation_val) =
                create_allocation_digit(i, config.num_agents, config.num_items, &utility);
            let product: i32 = allocation_val.iter().product();

            match product.cmp(&max_val) {
                Ordering::Greater => {
                    max_val = product;
                    nsw_allocations.clear();
                    nsw_allocations.push(i);
                }
                Ordering::Equal => {
                    nsw_allocations.push(i);
                }
                _ => (),
            }
        }

        if nsw_allocations.len() > 1 {
            compare_pairs_nsw(
                nsw_allocations,
                config.num_agents,
                config.num_items,
                &utility,
            );
        }
        // Progress barの更新を間引く
        if case % update_frequency == 0 {
            pb.inc(1);
        }
    }
    pb.finish();
    Ok(())
}

// Compare every pairs of two nsw allocations with respect to position envy_free up to one
fn compare_pairs_nsw(
    nsw_allocations: Vec<usize>,
    num_agents: usize,
    num_items: usize,
    utility: &[Vec<i32>],
) {
    // 各配分の事前計算
    let allocations: Vec<_> = nsw_allocations
        .iter()
        .map(|&id| create_allocation_digit(id, num_agents, num_items, utility))
        .collect();
    // 必要な比較のみを実行
    for i in 0..allocations.len() {
        for j in (i + 1)..allocations.len() {
            let (alloc1, val1) = &allocations[i];
            let (alloc2, val2) = &allocations[j];
            
            // 積が等しいことを確認
            assert!(val1.iter().product::<i32>() == val2.iter().product::<i32>());

            if !is_position_ef1(utility, alloc1, alloc2) {
                panic!(
                    "NOT Position EF1 \n utility = {:?}, allocation_1 = {:?}, allocation_2 = {:?}",
                    utility, alloc1, alloc2
                );
            }
        }
    }


    // if nsw_allocations.len() > 1 {
    //     for id_1 in &nsw_allocations {
    //         for id_2 in &nsw_allocations {
    //             if id_1 != id_2 {
    //                 let (allocation_1, allocation_val_1) =
    //                     create_allocation_digit(*id_1, num_agents, num_items, utility);
    //                 let (allocation_2, allocation_val_2) =
    //                     create_allocation_digit(*id_2, num_agents, num_items, utility);

    //                 let product_1: i32 = allocation_val_1.iter().product();
    //                 let product_2: i32 = allocation_val_2.iter().product();
    //                 assert!(product_1 == product_2);

    //                 if !is_position_ef1(utility, &allocation_1, &allocation_2) {
    //                     panic!(
    //                         "NOT Position EF1 \n utility = {:?}, allocation_1 = {:?}, allocation_2 = {:?}",
    //                         &utility, &allocation_1, &allocation_2
    //                     );
    //                 }
    //             }
    //         }
    //     }
    //     // println!("All pairs are Position EF1");
    // }
}

// fn create_allocation_digit(
//     id: usize,
//     num_agents: usize,
//     num_items: usize,
//     utility: &[Vec<i32>],
// ) -> (Vec<Vec<usize>>, Vec<i32>) {
//     let mut allocation = vec![Vec::new(); num_agents];
//     let mut allocation_val = vec![0; num_agents];
//     // println!("id= {}",id);
//     let mut allocation_digit = to_base(id as u64, num_agents as u32);

//     // add 0 to the front of the allocation_digit to make its length equal to num_items
//     while allocation_digit.len() < num_items {
//         allocation_digit.insert(0, '0');
//     }

//     for (j, digit) in allocation_digit.iter().enumerate().take(num_items) {
//         // println!("allocation_digit: {:?}", allocation_digit);
//         let digit = digit.to_digit(36).unwrap() as usize;

//         allocation_val[digit] += utility[digit][j];
//         allocation[digit].push(j);
//     }
//     (allocation, allocation_val)
// }

fn create_allocation_digit(
    id: usize,
    num_agents: usize,
    num_items: usize,
    utility: &[Vec<i32>],
) -> (Vec<Vec<usize>>, Vec<i32>) {
    let mut allocation = vec![Vec::with_capacity(num_items); num_agents];
    let mut allocation_val = vec![0; num_agents];
    let mut temp_id = id;
    
    for j in 0..num_items {
        let agent = temp_id % num_agents;
        allocation_val[agent] += utility[agent][j];
        allocation[agent].push(j);
        temp_id /= num_agents;
    }
    
    (allocation, allocation_val)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_pairs_nsw() {
        let num_agents = 3;
        let num_items = 6;
        let utility = vec![
            vec![1, 1, 0, 0, 3, 0], 
            vec![1, 0, 1, 2, 2, 2], 
            vec![0, 1, 0, 1, 3, 2]
        ];

        let mut nsw_allocations = Vec::new();
        let mut max_val = -1;

        // allocate
        for i in 0..(num_agents as u32).pow(num_items as u32) as usize {
            let (allocation, allocation_val) =
                create_allocation_digit(i, num_agents, num_items, &utility);
            let product: i32 = allocation_val.iter().product();
            match product.cmp(&max_val) {
                Ordering::Greater => {
                    max_val = product;
                    nsw_allocations.clear();
                    nsw_allocations.push(i);
                    println!("Product of allocation_val: {}", product);
                    println!("allocation: {:?}", allocation);
                }
                Ordering::Equal => {
                    nsw_allocations.push(i);
                    println!("Product of allocation_val: {}", product);
                    println!("allocation: {:?}", allocation);
                }
                _ => (),
            }
        }

        println!("The number of nsw_allocations is {}", nsw_allocations.len());

        compare_pairs_nsw(
            nsw_allocations,
            num_agents,
            num_items,
            &utility,
        );
    }
}