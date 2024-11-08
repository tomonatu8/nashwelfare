pub fn is_position_ef1(
    utility: &[Vec<i32>],
    allocation_1: &[Vec<usize>],
    allocation_2: &[Vec<usize>],
) -> bool {
    let num_agents = utility.len();

    for i in 0..num_agents {
        let mut envy_free_up_to_one_1_2 = false;
        let mut envy_free_up_to_one_2_1 = false;


        let mut utility_1 = 0;
        for &item in &allocation_1[i] {
            utility_1 += utility[i][item];
        }

        let mut utility_2 = 0;
        for &item in &allocation_2[i] {
            utility_2 += utility[i][item];
        }


        if utility_1 >= utility_2 {
            envy_free_up_to_one_1_2 = true;
        } else {
            for &item in &allocation_2[i] {
                let reduced_utility_2 = utility_2 - utility[i][item];
                if utility_1 >= reduced_utility_2 {
                    envy_free_up_to_one_1_2 = true;
                    break;
                }
            }
        }

        if utility_2 >= utility_1 {
            envy_free_up_to_one_2_1 = true;
        } else {
            for &item in &allocation_1[i] {
                let reduced_utility_1 = utility_1 - utility[i][item];
                if utility_2 >= reduced_utility_1 {
                    envy_free_up_to_one_2_1 = true;
                    break;
                }
            }
        }

        if envy_free_up_to_one_1_2 && envy_free_up_to_one_2_1 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_position_ef1() {
        let utility = vec![vec![2, 1, 2], vec![2, 3, 3]];
        let allocation_1 = vec![vec![0, 2], vec![1]];
        let allocation_2 = vec![vec![0], vec![1, 2]];
        assert_eq!(
            is_position_ef1(&utility, &allocation_1, &allocation_2),
            true
        );
    }
}
