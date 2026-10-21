#include <cassert>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    string longestPrefix(string s) {
        vector<int> long_borders(s.size(), 0);
        int border = 0;
        for (int i = 1; i < s.size(); i++) {
            while (border > 0 && s[border] != s[i]) {
                border = long_borders[border - 1];
            }

            if (s[border] == s[i]) {
                border += 1;
            }
            long_borders[i] = border;
        }

        int last_pos = *(long_borders.end() - 1);
        return s.substr(0, last_pos);
    }
};

int main() {
    assert(Solution().longestPrefix("ababab") == "abab");
    cout << "all pass" << endl;
}