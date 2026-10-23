#include <vector>

using namespace std;

class Solution {
public:
    char nextGreatestLetter(vector<char> &letters, char target) {
        int left = 0;
        int right = letters.size();
        int target_idx = target - 'a';

        while (left < right) {
            int mid = left + (right - left) / 2;
            int idx = letters[mid] - 'a';
            if (idx > target_idx) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        if (left == letters.size()) {
            return letters[0];
        }
        return letters[left];
    }
};

int main() {}