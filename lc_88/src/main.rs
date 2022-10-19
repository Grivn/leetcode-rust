fn main() {
    let nums1 = &mut vec![1,2,3,0,0,0];
    let nums2 = &mut vec![2,3,4];
    let m = 3;
    let n = 3;
    Solution::merge(nums1, m, nums2, n);
    println!("{:?}", nums1)
}

struct Solution {

}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = (m + n) as usize - 1;
        let mut m = m as usize;
        let mut n = n as usize;

        while m > 0 && n > 0 {
            if nums1[m - 1] >= nums2[n - 1] {
                nums1[i] = nums1[m - 1];
                m -= 1;
            } else {
                nums1[i] = nums2[n - 1];
                n -= 1;
            }

            i -= 1;
        }

        while n > 0 {
            nums1[i] = nums2[n - 1];
            if n == 0 {
                break;
            }
            n -= 1;
            i -= 1;
        }
    }
}