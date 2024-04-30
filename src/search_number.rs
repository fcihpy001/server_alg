
/**
出同时满足以下四个条件的所有三位数（即000-999），并写出最终结果。
   1).和三位数246相比，仅有1个号码相同，且位置相同。
   2).和三位数692相比，仅有2个号码相同，但位置都不相同。
   3).和三位数174相比，没有一个号码相同。
   4).和三位数419相比，有1个号码相同，但位置不相同
    -- 结果为906，936，956，986
 */
pub fn search_match_number() -> Vec<u16> {

    let mut match_numbs = Vec::new();
    for num in 100..=999 {
        if check_match_246(num)
            && check_match_692(num)
            && check_match_174(num)
            && check_match_419(num)
        {
            println!("找到一个符合条件的三位数:{}", num);
            match_numbs.push(num)
        }
    }

    match_numbs
}

/// 判断`num`是否满足条件1:与三位数246相比，有1个号码相同，但位置不相同
///
/// # 参数
///
/// * `num` - 要比对的数字
///
/// # 返回值
///
///  - 数字满足所有条件，则返回 `true`，否则返回 `false`。
///
/// # Examples
///
/// ```
/// let arg = 384;
/// assert_eq!(crate::search::check_match_246(156), true);
/// assert_eq!(crate::search::check_match_246(246), false);
/// ```
///
fn check_match_246(num: u16) -> bool {
    let num_str = num.to_string();
    let condition_str = 246.to_string();
    let mut matches = 0;
    for (index, char) in num_str.chars().enumerate() {
        if char == condition_str.chars().nth(index).unwrap() {
            matches += 1;
        }
    }
    if matches != 1 {
        return false;
    }
    return true;
}

/// 判断`num`是否满足条件2，和三位数692相比，仅有2个号码相同，但位置都不相同。
///
/// # 参数
///
/// * `num` - 要比对的数字
///
/// # 返回值
///
///  - 数字满足所有条件，则返回 `true`，否则返回 `false`。
///
/// # Examples
///
/// ```
/// assert_eq!(crate::search::check_match_692(921), true);
/// assert_eq!(crate::search::check_match_692(690), false);
/// ```
///
fn check_match_692(num: u16) -> bool {
    let num_str = num.to_string();
    let condition_str = 692.to_string();
    let mut matches = 0;
    for (index, char) in num_str.chars().enumerate() {
        if char == condition_str.chars().nth(index).unwrap() {
            return false;
        } else if condition_str.contains(char) {
            matches += 1;
        }
    }
    if matches != 2 {
        return false;
    }
    return true;
}

/// 判断`num`是否满足条件3，和三位数174相比，没有一个号码相同
///
/// # 参数
///
/// * `num` - 要比对的数字
///
/// # 返回值
///
///  - 数字满足所有条件，则返回 `true`，否则返回 `false`。
///
/// # Examples
///
/// ```
/// assert_eq!(crate::search::check_match_174(856), true);
/// assert_eq!(crate::search::check_match_174(194), false);
/// ```
///
fn check_match_174(num: u16) -> bool {
    let num_str = num.to_string();
    let condition_str = 174.to_string();
    let mut matches = 0;
    for digit in num_str.chars() {
        if !condition_str.contains(digit) {
            matches += 1;
        }
    }
    if matches != 3 {
        return false;
    }
    return true;
}

/// 判断`num`是否满足条件，与三位数419相比，有1个号码相同，但位置不相同
///
/// # 参数
///
/// * `num` - 要比对的数字
///
/// # 返回值
///
///  - 数字满足所有条件，则返回 `true`，否则返回 `false`。
///
/// # Examples
///
/// ```
///
/// assert_eq!(crate::search::check_match_692(921), true);
/// assert_eq!(crate::search::check_match_692(690), false);
/// ```
///
fn check_match_419(num: u16) -> bool {
    let num_str = num.to_string();
    let condition_str = 419.to_string();
    let mut matches = 0;
    for (index, digit) in num_str.chars().enumerate() {
        if digit == condition_str.chars().nth(index).unwrap() {
            return false;
        } else if condition_str.contains(digit) {
            matches += 1;
        }
    }
    if matches != 1 {
        return false;
    }
    return true;
}

#[cfg(test)]
pub mod tests {
    use crate::search_number::{check_match_174, check_match_246, check_match_419, check_match_692};

    #[test]
    fn test_check_match_246() {
        assert_eq!(check_match_246(298), true);
        assert_eq!(check_match_246(348), true);
        assert_eq!(check_match_246(156), true);

        assert_eq!(check_match_246(246), false);
        assert_eq!(check_match_246(248), false);
        assert_eq!(check_match_246(296), false);
    }

    #[test]
    fn test_check_match_692() {
        assert_eq!(check_match_692(529), true);
        assert_eq!(check_match_692(369), true);
        assert_eq!(check_match_692(921), true);

        assert_eq!(check_match_692(690), false);
        assert_eq!(check_match_692(592), false);
        assert_eq!(check_match_692(692), false);
    }

    #[test]
    fn test_check_match_174() {
        assert_eq!(check_match_174(298), true);
        assert_eq!(check_match_174(328), true);
        assert_eq!(check_match_174(856), true);

        assert_eq!(check_match_174(194), false);
        assert_eq!(check_match_174(648), false);
        assert_eq!(check_match_174(409), false);
    }

    #[test]
    fn test_check_match_419() {
        assert_eq!(check_match_419(158), true);
        assert_eq!(check_match_419(247), true);
        assert_eq!(check_match_419(692), true);

        assert_eq!(check_match_419(419), false);
        assert_eq!(check_match_419(140), false);
        assert_eq!(check_match_419(901), false);
    }

    #[test]
    fn test_match_number() {
        let num = 906;
        assert_eq!(check_match_246(num), true);
        assert_eq!(check_match_692(num), true);
        assert_eq!(check_match_174(num), true);
        assert_eq!(check_match_419(num), true);
    }
}
