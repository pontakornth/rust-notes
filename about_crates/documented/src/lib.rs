/// Check if the given number is a positive prime number or not.
///
/// # Example
/// ```
/// use documented::is_prime;
/// assert!(is_prime(3));
/// assert!(!is_prime(4));
/// ````
///
pub fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    };

    for i in 2..n {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_is_prime() {
        assert!(is_prime(3));
        assert!(is_prime(5));
        assert!(is_prime(7));
        assert!(is_prime(11));
        assert!(is_prime(2));
    }

    #[test]
    fn composite_is_not_prime() {
        assert!(!is_prime(4));
        assert!(!is_prime(6));
        assert!(!is_prime(9));
    }

    #[test]
    fn one_is_not_prime() {
        assert!(!is_prime(1));
    }

    #[test]
    fn zero_is_not_prime() {
        assert!(!is_prime(0));
    }
}
