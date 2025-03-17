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
    fn test_invalid_weight_distance() {
        // path 0, 1, 2T, 9, 10
        assert_eq!(calculate_shipping_cost(0, 999), -1);
    }

    #[test]
    fn test_valid_cases() {
        // path 0, 1, 2F, 3F, 5F, 7, 8, 9, 10 
        assert_eq!(calculate_shipping_cost(37, 120), 4440000); 

        // path 0, 1, 2F, 3T, 4, 8, 9, 10 
        assert_eq!(calculate_shipping_cost(6, 37), 444000); 

        // path 0, 1, 2F, 3F, 5T, 6, 8, 9, 10
        assert_eq!(calculate_shipping_cost(13, 120), 2340000); 
    }
}