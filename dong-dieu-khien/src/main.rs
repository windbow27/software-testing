fn calculate_shipping_cost(weight: i32, distance: i32) -> i32 {
    let mut result: i32 = -1;

    if weight <= 0 || distance <= 0 || weight > 50 || distance > 500{
        return result;
    }

    let price: i32 = if weight <= 10 {
        2000
    } else if weight <= 30 {
        1500
    } else {
        1000
    };

    result = weight * distance * price;

    return result;
}

fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_path_1() {
        // path 0,1,2(T),9,10: invalid weight
        assert_eq!(calculate_shipping_cost(-1, 37), -1);
    }

    #[test]
    fn test_path_2() {
        // path 0,1,2(F),3(T),4,8,9,10: weight ≤ 10
        assert_eq!(calculate_shipping_cost(6, 37), 6 * 37 * 2000);
        assert_eq!(calculate_shipping_cost(6, 37), 444000);
    }

    #[test]
    fn test_path_3() {
        // path 0,1,2(F),3(F),5(T),6,8,9,10: 10 < weight ≤ 30
        assert_eq!(calculate_shipping_cost(13, 120), 13 * 120 * 1500);
        assert_eq!(calculate_shipping_cost(13, 120), 2340000);
    }

    #[test]
    fn test_path_4() {
        // path 0,1,2(F),3(F),5(F),7,8,9,10: weight > 30
        assert_eq!(calculate_shipping_cost(37, 120), 37 * 120 * 1000);
        assert_eq!(calculate_shipping_cost(37, 120), 4440000);
    }
}   