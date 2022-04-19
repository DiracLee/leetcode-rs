#include <iostream>
#include <set>
#include <vector>

using namespace std;

class Solution {
 public:
  bool containsNearbyAlmostDuplicate(vector<int> &nums, int k, int t) {
    int n = nums.size();
    multiset<long long> set;
    set.insert(1e18);
    set.insert(-1e18);
    for (int i = 0, j = 0; i < n; ++i) {
      if (i - j > k) set.erase(set.find(nums[j++]));
      int x = nums[i];
      auto iter = set.upper_bound(x);  // >= x 的最小的数
      if (*iter - x <= t) return true;

      --iter;  // <= x 的最大的数
      if (x - *iter <= t) return true;

      set.insert(x);
    }

    return false;
  }
};

int main() {
  auto nums = vector<int>{1, 5, 9, 1, 5, 9};
  int k = 2, t = 3;
  cout << Solution().containsNearbyAlmostDuplicate(nums, k, t) << endl;
  return 0;
}