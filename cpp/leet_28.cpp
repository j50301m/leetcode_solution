#include <cassert>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    int strStr(string haystack, string needle) {
        auto longest_borders = build_longest_border(needle);

        int border = 0;
        for (int i = 0; i < haystack.size(); i++) {
            while (border > 0 && haystack[i] != needle[border]) {
                border = longest_borders[border - 1];
            }

            if (haystack[i] == needle[border]) {
                border += 1;
            }

            if (border == needle.size()) {
                return i - border + 1;
            }
        }
        return -1;
    }

    vector<int> build_longest_border(string &str) {
        vector<int> longest_borders(str.size(), 0);
        int border = 0;
        for (int i = 1; i < str.size(); i++) {
            while (border > 0 && str[border] != str[i]) {
                border = longest_borders[border - 1];
            }

            if (str[border] == str[i]) {
                border += 1;
            }

            longest_borders[i] = border;
        }
        return longest_borders;
    }
};

int run(string haystack, string needle) {
    return Solution().strStr(haystack, needle);
}

int main() {
    assert(run("sadbutsad", "sad") == 0);
    assert(run("leetcode", "leeto") == -1);
    assert(run("aaaaab", "aab") == 3);
    assert(run("mississippi", "issip") == 4);
    assert(run("abc", "abcd") == -1);

    string pattern = "ABACABAB";
    assert(
        Solution().build_longest_border(pattern) ==
        vector<int>({0, 0, 1, 0, 1, 2, 3, 2})
    );
    cout << "all pass" << endl;
}