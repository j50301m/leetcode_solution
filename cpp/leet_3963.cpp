#include <array>
#include <cassert>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    vector<string> createGrid(int m, int n) {
        vector<string> grid(m, string(n, '#'));
        array<pair<int, int>, 2> directions = {{{1, 0}, {0, 1}}};

        vector<pair<int, int>> stack{{0, 0}};
        while (!stack.empty()) {
            pair<int, int> pos = stack.back();
            stack.pop_back();
            grid[pos.first][pos.second] = '.';
            if (pos.first == m - 1 && pos.second == n - 1) {
                break;
            }

            for (pair<int, int> dir : directions) {
                int r = pos.first + dir.first;
                int c = pos.second + dir.second;

                if (r < 0 || r > m - 1 || c < 0 || c > n - 1) {
                    continue;
                }

                stack.push_back({r, c});
            }
        }

        return grid;
    }
};

// 檢查 grid 合法：大小 m x n、只有 '#' / '.'、起點與終點是 '.'、且從起點走得到終點
bool is_valid_grid(const vector<string> &grid, int m, int n) {
    if ((int)grid.size() != m) {
        return false;
    }
    for (const string &row : grid) {
        if ((int)row.size() != n) {
            return false;
        }
        for (char cell : row) {
            if (cell != '#' && cell != '.') {
                return false;
            }
        }
    }
    if (grid[0][0] != '.' || grid[m - 1][n - 1] != '.') {
        return false;
    }

    array<pair<int, int>, 4> directions = {{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}};
    vector<vector<bool>> visited(m, vector<bool>(n, false));
    vector<pair<int, int>> stack{{0, 0}};
    visited[0][0] = true;
    while (!stack.empty()) {
        pair<int, int> pos = stack.back();
        stack.pop_back();
        if (pos.first == m - 1 && pos.second == n - 1) {
            return true;
        }
        for (pair<int, int> dir : directions) {
            int row = pos.first + dir.first;
            int col = pos.second + dir.second;
            if (row < 0 || row > m - 1 || col < 0 || col > n - 1) {
                continue;
            }
            if (visited[row][col] || grid[row][col] != '.') {
                continue;
            }
            visited[row][col] = true;
            stack.push_back({row, col});
        }
    }
    return false;
}

int main() {
    assert(is_valid_grid(Solution().createGrid(3, 4), 3, 4));
    // 只有一格
    assert(is_valid_grid(Solution().createGrid(1, 1), 1, 1));
    // 只有一列 / 一行
    assert(is_valid_grid(Solution().createGrid(1, 5), 1, 5));
    assert(is_valid_grid(Solution().createGrid(5, 1), 5, 1));
    // 正方形
    assert(is_valid_grid(Solution().createGrid(4, 4), 4, 4));
    cout << "all pass" << endl;
}