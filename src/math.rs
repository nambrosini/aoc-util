use num_traits::{PrimInt, Signed, cast::AsPrimitive};

fn mcd<T>(a: T, b: T) -> T
    where
        T: PrimInt + Signed,
{
    let (a, b) = (a.abs(), b.abs());
    let (a, b) = if a < b { (b, a) } else { (a, b) };
    if b.is_zero() {
        return a;
    }
    mcd(b, a % b)
}

fn mcm<T>(a: T, b: T) -> T
    where
        T: PrimInt + AsPrimitive<T> + Signed
{
    a * b / mcd(a.as_(), b.as_())
}


#[cfg(test)]
mod tests {
    use crate::math::{mcd, mcm};

    #[test]
    fn test_mcd() {
        let want = 5;
        let got = mcd(10, 5);

        assert_eq!(got, want);
    }

    #[test]
    fn test_mcd_neg() {
        let want = 5;
        let got = mcd(-10, 5);

        assert_eq!(got, want);
    }

    #[test]
    fn test_mcm() {
        let want = 10;
        let got = mcm(10, 5);

        assert_eq!(got, want);
    }
}
