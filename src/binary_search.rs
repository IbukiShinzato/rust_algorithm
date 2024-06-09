use proconio::input;

//二分探索
pub fn run() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n],
    }

    a.sort();
    let mut left = 0;
    let mut right = n;
    while left < right {
        let mid = (left + right) / 2;
        if a[mid] > k {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    println!("{}", left);
}
