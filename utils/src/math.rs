use num::Num;

pub fn gcd<T: Num + Copy>(a: T, b: T) -> T {
    if b == T::zero() {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm<T: Num + Copy + Ord>(a: T, b: T) -> T {
    if a > b {
        (a / gcd(a, b)) * b
    } else {
        (b / gcd(a, b)) * a
    }
}
