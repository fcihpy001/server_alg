
/// 左右移动值较小的指针向中间遍历，就可以在一次遍历中同时计算出左边的最高度和右边最高高度，直至相遇
pub fn rain_total(height: Vec<u32>) -> u32 {
    let mut total = 0;
    let (mut left, mut right) = (0, height.len() - 1);
    let (mut left_max, mut right_max) = (0, 0);
    while left < right {
        left_max = left_max.max(height[left]);
        right_max = right_max.max(height[right]);
        if height[left] < height[right] {
            total += left_max - height[left];
            left += 1;
        } else {
            total += right_max - height[right];
            right -= 1;
        }
    }
    total
}

#[cfg(test)]
pub mod tests{
    use crate::rain_water::rain_total;

    #[test]
    fn test_rain() {
        let v1 = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        let v2 = vec![4, 2, 0, 3, 2, 5];

        assert_eq!(rain_total(v1),6);
        assert_eq!(rain_total(v2),9);
    }
}

