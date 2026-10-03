#include <cassert>
#include <iostream>
#include <vector>
using namespace std;

class Solution {
public:
    vector<long long>
    minimumCosts(vector<int> &regular, vector<int> &express, int expressCost) {
        int n = regular.size();
        vector<long long> reg(n + 1, 0);
        vector<long long> exp(n + 1, 0);
        vector<long long> result(n, 0);
        exp[0] = (long long)expressCost;

        for (int i = 1; i <= n; i++) {
            long long r_cost = regular[i - 1];
            long long e_cost = express[i - 1];
            reg[i] = min(reg[i - 1] + r_cost, exp[i - 1] + r_cost);
            exp[i] =
                min(exp[i - 1] + e_cost, reg[i - 1] + expressCost + e_cost);

            result[i - 1] = min(reg[i], exp[i]);
        }

        return result;
    }
};

vector<long long>
run(vector<int> regular, vector<int> express, int expressCost) {
    return Solution().minimumCosts(regular, express, expressCost);
}

void test_examples() {
    assert(
        (run({1, 6, 9, 5}, {5, 2, 3, 10}, 8) == vector<long long>{1, 7, 14, 19})
    );
    assert((run({11, 5, 13}, {7, 10, 6}, 3) == vector<long long>{10, 15, 24}));
}

void test_single_stop() { assert((run({5}, {1}, 10) == vector<long long>{5})); }

void test_switch_back_to_regular_is_free() {
    assert((run({1, 100, 1}, {100, 1, 100}, 1) == vector<long long>{1, 3, 4}));
}

void test_large_sum_exceeds_int() {
    int n = 100000;
    vector<long long> result =
        run(vector<int>(n, 100000), vector<int>(n, 100000), 100000);
    assert(result[n - 1] == 10000000000LL);
}

int main() {
    test_examples();
    test_single_stop();
    test_switch_back_to_regular_is_free();
    test_large_sum_exceeds_int();
    cout << "all tests passed\n";
}
