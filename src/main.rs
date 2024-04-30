use std::time::Instant;
use crate::rain_water::rain_total;
use crate::search_number::search_match_number;

mod search_number;
mod rain_water;

fn main() {
    // 搜索符合条件的三位数
    let start_time = Instant::now();
    let match_numbs = search_match_number();
    let duration = start_time.elapsed().as_micros();
    println!(
        "找到符合条件的三位数有{}个,总耗时{}ms",
        match_numbs.len(),
        duration
    );
    println!("********************************************");

    // 计算能接收到的最多雨水
    let  rocks1 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let  rocks2 = vec![4, 2, 0, 3, 2, 5];
    println!("rock1 能接收的最多雨水量为:{}", rain_total(rocks1));
    println!("rock2 能接收的最多雨水量为:{}", rain_total(rocks2))
}
