// @leet start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res:Vec<i32> = vec![0,0];

        for index in nums.iter(){
            let final_target:i32 = target - index;

        for (i, val) in nums.iter().enumerate() {
                if i == final_target {
                    res.0 = index;
                    res.1 = i;
                    break;
                }
            }
        }
        res
    }
}
// @leet end
