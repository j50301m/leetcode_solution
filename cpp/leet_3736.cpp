#include <vector>

using namespace std;

class Solution {
public:
    int minMoves(vector<int> &nums) {
        int max = 0;
        for (int num : nums) {
            if (num > max) {
                max = num;
            }
        }

        int count = 0;
        for (int num : nums) {
            count += max - num;
        }

        return count;
    }
};

int main() {}