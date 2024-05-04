use std::time::Instant;
use crate::rain_water::{rain_trap_2points, rain_trap_stack};
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

    // 计算能接收到的最多雨水 -- 双指针
    let  rocks1 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let  rocks2 = vec![4, 2, 0, 3, 2, 5];
    println!("双指针：rock1 能接收的最多雨水量为:{}", rain_trap_2points(rocks1));
    println!("双指针：rock2 能接收的最多雨水量为:{}", rain_trap_2points(rocks2));

    // 计算能接收到的最多雨水 -- 单调栈
    let  rocks3 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let  rocks4 = vec![4, 2, 0, 3, 2, 5];
    println!("单调栈：rock3 能接收的最多雨水量为:{}", rain_trap_stack(rocks3));
    println!("单调栈：rock4 能接收的最多雨水量为:{}", rain_trap_stack(rocks4));

    // 计算能接收到的最多雨水 -- 动态规划
    let  rocks5 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    let  rocks6 = vec![4, 2, 0, 3, 2, 5];
    println!("动态规划：rock5 能接收的最多雨水量为:{}", rain_trap_stack(rocks5));
    println!("动态规划：rock6 能接收的最多雨水量为:{}", rain_trap_stack(rocks6));
}
