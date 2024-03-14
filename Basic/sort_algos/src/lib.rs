pub fn insert_sort<T: Copy, F: Fn(T, T) -> bool>(v: &mut [T], is_less: F) {
    if v.len() > 1 {
        for i in 1..v.len() {
            for j in (1..=i).rev() {
                if is_less(v[j], v[j - 1]) {
                    v.swap(j, j - 1);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn sort_result_ok() {
        let mut rng = thread_rng();

        let a: Vec<_> = (0..10000).map(|_| rng.gen_range(-10000..10000)).collect();
        let mut ans = a.clone();
        ans.sort();

        let mut b = a.clone();
        insert_sort(&mut b, |a, b| a < b);

        for (a, b) in ans.iter().zip(b.iter()) {
            assert!(a == b);
        }
    }
}
