use std::collections::HashMap;


fn calculate_mean(v: &mut Vec<i32>) -> i32 {
    let mut calculated_mean = 0;
    for i in v {
        calculated_mean = calculated_mean + *i;
    }
    calculated_mean
}

fn calculate_median(v: &mut Vec<i32>) -> i32 {
    let idx = v.len() / 2;
    v.sort();
    v[idx]
}

fn calculate_mode(v: &mut Vec<i32>) -> i32 {
    let mut hm = HashMap::new();
    let mut pv = 0;
    let mut pk = 0;
    for i in v {
        // Check if key already exists in hashmap,
        // if not, insert "0".
        // In any case, increment the value.
        let count = hm.entry(*i).or_insert(0);
        *count +=1;
    }

    for (k, val) in hm {
        if val > pv {
            pv = val;
            pk = k;
        }
    }
    pk
}


fn main() {
    let mut list = vec![1, 4, 15, 4, 3, 2, 1, 3, 7, 2, 6, 4];

    let mean = calculate_mean(&mut list);
    let median = calculate_median(&mut list);
    let mode = calculate_mode(&mut list);

    println!("Mean: {}\nMedian: {}\nMode: {}", mean, median, mode);
}
