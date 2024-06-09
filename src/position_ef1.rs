

pub fn is_position_ef1(
    utility: &Vec<Vec<i32>>, 
    allocation_1: &Vec<Vec<usize>>, 
    allocation_2: &Vec<Vec<usize>>
) -> bool {

let num_agents = utility.len();

for i in 0..num_agents {

    let mut envy_free_up_to_one = false;
    
    
    let mut utility_1 = 0;
    for &item in &allocation_1[i] {
        utility_1 += utility[i][item];
    }
    
    
    let mut utility_2 = 0;
    for &item in &allocation_2[i] {
        utility_2 += utility[i][item];
    }
    
    for &item in &allocation_2[i] {
        let reduced_utility_2 = utility_2 - utility[i][item];
        if utility_1 >= reduced_utility_2 {
            envy_free_up_to_one = true;
            break;
        }
    }

    for &item in &allocation_1[i] {
        let reduced_utility_1 = utility_1 - utility[i][item];
        if utility_2 >= reduced_utility_1 {
            envy_free_up_to_one = true;
            break;
        }
    }
    
    if !envy_free_up_to_one {
        return false;
    }
}
true
}