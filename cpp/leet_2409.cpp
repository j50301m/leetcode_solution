#include <cassert>
#include <iostream>
#include <sstream>

using namespace std;

class Solution {
public:
    int countDaysTogether(
        string arriveAlice,
        string leaveAlice,
        string arriveBob,
        string leaveBob
    ) {

        vector<int> prefix_days(13, 0);
        for (int i = 0; i < 12; i++) {
            prefix_days[i + 1] = prefix_days[i] + MONTHS[i];
        }

        int a1 = convert_to_days(arriveAlice, prefix_days);
        int a2 = convert_to_days(leaveAlice, prefix_days);
        int b1 = convert_to_days(arriveBob, prefix_days);
        int b2 = convert_to_days(leaveBob, prefix_days);

        int arrive = max(a1, b1);
        int leave = min(a2, b2);
        return max(leave - arrive + 1, 0);
    }

private:
    const array<int, 12> MONTHS =
        {31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31};
    int convert_to_days(string date, vector<int> &prefix_days) {
        int month, day;
        char dash;
        stringstream(date) >> month >> dash >> day;

        return prefix_days[month - 1] + day;
    }
};

int main() {
    assert(Solution().countDaysTogether("08-15", "08-18", "08-16", "08-19") == 3);
    assert(Solution().countDaysTogether("10-01", "10-31", "11-01", "12-31") == 0);
    // 只重疊一天（交界日）
    assert(Solution().countDaysTogether("01-01", "01-10", "01-10", "01-20") == 1);
    // 跨月：02-27 ~ 03-02，2 月有 28 天
    assert(Solution().countDaysTogether("01-30", "03-02", "02-27", "12-31") == 4);
    // 整年
    assert(Solution().countDaysTogether("01-01", "12-31", "01-01", "12-31") == 365);
    cout << "all pass" << endl;
}