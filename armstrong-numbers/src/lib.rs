pub fn is_armstrong_number(num: u32) -> bool {
    num.to_string()
        .chars()
        .try_fold(0, |sum, val| {
            val.to_digit(10)
                .map(|val| sum + val.pow(num.to_string().len() as u32))
        })
        .map_or(false, |sum| sum == num)
}

// let num_str = num.to_string();
// let num_len = num_str.len() as u32;

// num_str.chars()
//     .filter_map(|c| c.to_digit(10))
//     .try_fold(0, |sum, val| Some(sum + val.pow(num_len)))
//     .map_or(false, |sum| sum == num)
// }
