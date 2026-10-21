#include <cassert>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    bool rotateString(string s, string goal) {
        if (goal.size() != s.size()) {
            return false;
        }

        string source = s.append(s.begin(), s.end() - 1);
        auto longest_borders = build_longest_border(goal);

        int border = 0;
        for (int i = 0; i < source.size(); i++) {
            while (border > 0 && source[i] != goal[border]) {
                border = longest_borders[border - 1];
            }

            if (source[i] == goal[border]) {
                border += 1;
            }

            if (border == goal.size()) {
                return true;
            }
        }

        return false;
    }

private:
    vector<int> build_longest_border(string &str) {
        vector<int> longest_borders(str.size(), 0);
        int border = 0;
        for (int i = 1; i < str.size(); i++) {
            while (border > 0 && str[i] != str[border]) {
                border = longest_borders[border - 1];
            }

            if (str[i] == str[border]) {
                border += 1;
            }

            longest_borders[i] = border;
        }

        return longest_borders;
    }
};

int main() {
    assert(Solution().rotateString("abcde", "cdeab") == true);
    assert(Solution().rotateString("abcde", "abced") == false);
    cout << "all pass" << endl;
}