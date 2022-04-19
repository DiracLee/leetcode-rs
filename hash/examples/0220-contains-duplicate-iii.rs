use std::collections::BTreeMap;
impl Solution {
    pub fn contains_nearby_almost_duplicate(nums: Vec<i32>, d: i32, t: i32) -> bool {
        let mut map = BTreeMap::new();

        let mut j = 0;
        for (i, &x) in nums.iter().enumerate() {
            if map.len() as i32 > d {
                map.remove(&nums[j]);
                j += 1;
            }

            let (y, _) = lower_bound(&map, x);
            let (z, _) = upper_bound(&map, x);

            if (i64::abs(y as i64 - x as i64) <= t as i64)
                || (i64::abs(z as i64 - x as i64) <= t as i64)
            {
                return true;
            }
            map.insert(x, i);
        }

        false
    }
}

// 不大于 x 的最大的数
pub fn lower_bound(map: &BTreeMap<i32, usize>, x: i32) -> (i64, usize) {
    let mut y = -1 << 40;
    let mut j = 0;
    let mut iter = map.iter();
    while let Some((&num, &idx)) = iter.next() {
        if num > x {
            break;
        }
        y = num as i64;
        j = idx;
    }

    (y, j)
}

// 不小于 x 的最小的数
pub fn upper_bound(map: &BTreeMap<i32, usize>, x: i32) -> (i64, usize) {
    let mut y = 1 << 40 - 1;
    let mut j = 0;
    let mut iter = map.iter();
    while let Some((&num, &idx)) = iter.next() {
        if num >= x {
            y = num as i64;
            j = idx;
            break;
        }
    }

    (y, j)
}

struct Solution {}
fn main() {
    let nums = [1, 5, 9, 1, 5, 9];
    let d = 2;
    let t = 3;
    let ans = Solution::contains_nearby_almost_duplicate(nums.into(), d, t);
    dbg!(ans);
}
