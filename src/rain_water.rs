
/// 双指针解法 左右移动值较小的指针向中间遍历，就可以在一次遍历中同时计算出左边的最高度和右边最高高度，直至相遇
pub fn rain_trap_2points(height: Vec<i32>) -> i32 {
    let mut water_trapped = 0;
    let (mut left, mut right) = (0, height.len() - 1);
    let (mut left_max, mut right_max) = (0, 0);
    while left < right {
        left_max = left_max.max(height[left]);
        right_max = right_max.max(height[right]);
        if height[left] < height[right] {
            water_trapped += left_max - height[left];
            left += 1;
        } else {
            water_trapped += right_max - height[right];
            right -= 1;
        }
    }
    water_trapped
}

/// 单调栈解法
pub fn rain_trap_stack(heights: Vec<i32>) -> i32 {
    // 保存每个柱的索引
    let mut stack: Vec<usize> = Vec::new();
    // 保存总的雨水量
    let mut water_trapped = 0;
    // 记录当前柱的索引
    let mut current = 0;

    while current < heights.len() {
        // 当栈不为空且当前高度大于栈顶高度时
        while !stack.is_empty() && heights[current] > heights[*stack.last().unwrap()] {
            // 弹出栈顶元素
            let top = stack.pop().unwrap();

            // 如果栈变空了，退出循环
            if stack.is_empty() {
                break;
            }

            // 计算当前水的宽度
            let distance = current - stack.last().unwrap() - 1;
            // 找出界限高度
            let bounded_height = heights[current].min(heights[*stack.last().unwrap()]) - heights[top];
            // 加上当前积水量
            water_trapped += distance as i32 * bounded_height;
        }
        // 当前索引入栈
        stack.push(current);
        current += 1;
    }
    water_trapped
}

/// 动态规划触法 通过两个辅助数组存储左右两侧最大高度，然后计算每个位置的最大雨量
pub fn rain_trap_dynamic(height: Vec<i32>) -> i32 {
    let n = height.len();
    if n <= 2 {
        return 0;
    }

    let mut left_max = vec![0; n];
    let mut right_max = vec![0; n];

    left_max[0] = height[0];
    for i in 1..n {
        left_max[i] = left_max[i - 1].max(height[i]);
    }

    right_max[n - 1] = height[n - 1];
    for i in (0..n - 1).rev() {
        right_max[i] = right_max[i + 1].max(height[i]);
    }

    let mut result = 0;
    for i in 0..n {
        result += (left_max[i].min(right_max[i]) - height[i]).max(0);
    }

    result
}

#[cfg(test)]
pub mod tests{
    use crate::rain_water::{rain_trap_2points, rain_trap_dynamic, rain_trap_stack};

    #[test]
    fn test_rain_2points() {
        let v1 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let v2 = vec![4, 2, 0, 3, 2, 5];

        assert_eq!(rain_trap_2points(v1),6);
        assert_eq!(rain_trap_2points(v2),9);
    }

    #[test]
    fn test_rain_stack() {
        let v1 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let v2 = vec![4, 2, 0, 3, 2, 5];

        assert_eq!(rain_trap_stack(v1),6);
        assert_eq!(rain_trap_stack(v2),9);
    }

    #[test]
    fn test_rain_dynamic() {
        let v1 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let v2 = vec![4, 2, 0, 3, 2, 5];

        assert_eq!(rain_trap_dynamic(v1),6);
        assert_eq!(rain_trap_dynamic(v2),9);
    }

}

