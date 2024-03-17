use std::collections::HashMap;

fn main() {
    // related to howSum
    let mut sum_cache = HashMap::<i64,Option<Vec<i64>>>::new();
    let nums: [i64;5] = [2,6,3,7,1];
    dbg!(how_sum(14124, &nums, &mut sum_cache));

    // related to canSum
    /*
    let mut cache = HashMap::<i64,bool>::new();
    //let nums: [i64;9] = [4414,2,3414,5412,4142,20,502, 142, 242];
    //dbg!(can_sum(412401, &nums, &mut cache));
    let nums: [i64;9] = [12, 24,5, 6, 7, 3 ,2, 17, 19];
    dbg!(can_sum(41, &nums, &mut cache));
    */

    // related to gridtraveller
    /*
    let mut path_cache = HashMap::<(u64,u64),u64>::new();
    println!("{}",grid_traveller((25,25), &mut path_cache));
    */


    // related to fibonacci
    /*
    const n:u64 = 40;
    let mut mem_cache: [Option<u64>; n as usize] = [None; n as usize];
    dbg!(fib(n, &mut mem_cache));
    //fib(n,&mut mem_cache);
    dbg!(mem_cache);
    */

    //fib_slow(40);
}
fn how_sum(
    target_sum:i64, 
    nums: &[i64], 
    sum_cache: &mut HashMap<i64,Option<Vec<i64>>>
    ) -> Option<Vec<i64>> 
{
    if let Some(nums) = sum_cache.get(&target_sum) {
        match nums {
            Some(nums) => { 
                if !nums.is_empty() { return Some((*nums.clone()).to_vec()); }
            }
            None => { return None; }
        }
    }

    if target_sum == 0 { return Some(vec![]); }
    if target_sum <  0 { return None; }

    for num in nums {
        let remainder = target_sum - num;
        let mut result: Option<Vec<i64>> = how_sum(remainder, nums, sum_cache);

        match result {
            None => {
                ()
            }
            Some(mut summable_nums) => {
                summable_nums.push(*num);
                sum_cache.insert(target_sum, Some(summable_nums.clone()));
                return Some(summable_nums);
            }
        }
    }
    None
}

fn can_sum(target_sum:i64, nums: &[i64], target_sum_cache: &mut HashMap<i64, bool>) -> bool {
    if let Some(result) = target_sum_cache.clone().get(&target_sum) { return *result; }
    if target_sum == 0 { return true;  }
    if target_sum <  0 { return false; } 

    for num in nums {
        let remainder = target_sum - num;
        if can_sum(remainder, nums, target_sum_cache) {
            target_sum_cache.insert(target_sum, true);
            return true;
        }
    }

    target_sum_cache.insert(target_sum, false);
    return false;
}

fn grid_traveller(
    grid: (u64,u64), 
    path_cache: &mut HashMap<(u64,u64), u64>) -> u64 
{ 
    let (x, y) = (grid.0,grid.1);
    if x == 1 && y == 1{
        return 1;
    }
    if x == 0 || y == 0{
        return 0;
    }
    if let Some(n_of_paths) = path_cache.get(&grid) {
        return *n_of_paths;
    }

    let n_of_paths1 = grid_traveller((x-1,y), path_cache);
    let n_of_paths2 = grid_traveller((x,y-1), path_cache);
    let total_n_of_paths = n_of_paths1 + n_of_paths2;
    path_cache.insert(grid, total_n_of_paths);

    return *path_cache.get(&grid).unwrap();
}

fn grid_traveller_slow (
    grid: (u64,u64), 
    //path_cache: &mut HashMap<(u64,u64), u64>) -> u64 
    ) -> u64
{
    let (x, y) = (grid.0,grid.1);
    if x == 1 && y == 1{
        return 1;
    }
    if x == 0 || y == 0{
        return 0;
    }
    let n_of_paths = grid_traveller_slow((x-1,y)) + grid_traveller_slow((x,y-1));
    n_of_paths
}

fn fib(n:u64, mem_cache: &mut [Option<u64>]) -> u64 {
    match mem_cache[n as usize -1] {
        Some(n) => {return n;},
        None => (),
    }
    if n<= 2 {return 1;}
    
    mem_cache[n as usize -1] = Some(fib(n-1, mem_cache) + fib(n-2, mem_cache));
    mem_cache[n as usize -1].unwrap()
}

fn fib_slow(n:u64) -> u64 {
    if n <= 2 {return 1;}
    return fib_slow(n-1)+fib_slow(n-2);
}
