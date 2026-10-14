#include <vector>

using namespace std;

class Solution {
public:
    int hIndex(vector<int> &citations) {
        int n = citations.size();
        int left = 0;
        int right = n;

        while (left < right) {
            int mid = left + (right - left) / 2;
            int papers = n - mid;

            if (citations[mid] >= papers) {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        return n - left;
    }
};

int main() {}