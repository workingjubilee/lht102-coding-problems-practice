use std::collections::HashMap;

#[allow(dead_code)]
struct Solution;

impl Solution {
    #[allow(dead_code)]
    pub fn minimum_rounds(tasks: Vec<i32>) -> i32 {
        let arr: Vec<i32> = tasks
            .iter()
            .fold(HashMap::<i32, i32>::new(), |mut map, &num| {
                *map.entry(num).or_default() += 1;
                map
            })
            .into_values()
            .map(|num| {
                if num < 2 {
                    return -1;
                }
                (num + 2) / 3
            })
            .collect();
        if arr.iter().find(|&&x| x == -1).is_some() {
            -1
        } else {
            arr.iter().sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tasks = vec![2, 2, 3, 3, 2, 4, 4, 4, 4, 4];
        assert_eq!(Solution::minimum_rounds(tasks), 4);

        let tasks = vec![2, 3, 3];
        assert_eq!(Solution::minimum_rounds(tasks), -1);
    }
}
