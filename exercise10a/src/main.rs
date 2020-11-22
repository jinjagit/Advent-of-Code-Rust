fn main() {
    let dir: (i32, i32) = direction((-64, 4));
    println!("dir: {:?}", dir);
}

// Reduce a two-value coordinate vector (dx, dy) to the smallest possible integer vector representing the same direction.
// Example: direction((9, -3)) => (3, -1) 
fn direction((a, b): (i32, i32)) -> (i32, i32) {
    let gcd = find_gcd(a, b);
    return (a / gcd, b / gcd);
}

// Find greatest common denominator of 2 integers
fn find_gcd(a: i32, b: i32) -> i32 {
    let mut a = a.abs() as f32;
    let mut b = b.abs() as f32;
    let gcd: i32;

    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }

    loop {
        if a == 0.0 {
            gcd = b as i32;
            break;
        } else if b == 0.0 {
            gcd = a as i32;
            break;
        }

        let temp = a;
        let ratio = a / b;
        if ratio.floor() != ratio {
            b = temp - (b * ratio.floor());
            a = ratio.floor() - b;
        } else {
            a = 0.0;
        }
    }

    gcd
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn find_gcd_test() {
        let gcd = find_gcd(9, 3);
        assert_eq!(gcd, 3);

        let gcd = find_gcd(3, 21);
        assert_eq!(gcd, 3);

        let gcd = find_gcd(-64, 4);
        assert_eq!(gcd, 4);

        let gcd = find_gcd(19, 7);
        assert_eq!(gcd, 1);

        let gcd = find_gcd(7, 19);
        assert_eq!(gcd, 1);

        let gcd = find_gcd(0, -19);
        assert_eq!(gcd, 19);

        let gcd = find_gcd(19, 0);
        assert_eq!(gcd, 19);
    }
}
