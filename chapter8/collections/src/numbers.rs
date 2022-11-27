use std::collections::HashMap;

#[derive(Debug)]
pub enum NumberStat {
    Average(f32),
    Median(i32),
    Moda(i32),
}
pub fn number_stats(nums: &Vec<i32>) -> Vec<NumberStat> {
    let length = nums.len() as f32;
    let average:f32 = nums.iter().sum::<i32>() as f32 / length;
    let mut sorted_nums = nums.clone();
    sorted_nums.sort();
    let half = sorted_nums.len() / 2;
    let median = sorted_nums[half];
    
    let moda = calc_moda(nums);
    vec![
        NumberStat::Average(average),
        NumberStat::Median(median),
        NumberStat::Moda(moda),
        ]
}
fn calc_moda(nums: &Vec<i32>) -> i32 {
    let mut hm:HashMap<i32, i32> = HashMap::new();
    for num in nums {
        let count = hm.entry(*num).or_insert(0);
        *count += 1;
    }
    let max = max_key(&hm);
    *max.expect("Max exists")
}

fn max_key<K,V>(hm: &HashMap<K, V>) -> Option<&K> where V: Ord, {
    hm.iter()
    .max_by(|a, b| a.1.cmp(b.1))
    .map(|(k, _v)| k)
}