/**
 * // This is the ArrayReader's API interface.
 * // You should not implement it, or speculate about its implementation
 * class ArrayReader {
 *   public:
 *     // Compares the sum of arr[l..r] with the sum of arr[x..y]
 *     // return 1 if sum(arr[l..r]) > sum(arr[x..y])
 *     // return 0 if sum(arr[l..r]) == sum(arr[x..y])
 *     // return -1 if sum(arr[l..r]) < sum(arr[x..y])
 *     int compareSub(int l, int r, int x, int y);
 *
 *     // Returns the length of the array
 *     int length();
 * };
 */

/**
 * // This is the ArrayReader's API interface.
 * // You should not implement it, or speculate about its implementation
 * class ArrayReader {
 *   public:
 *     // Compares the sum of arr[l..r] with the sum of arr[x..y]
 *     // return 1 if sum(arr[l..r]) > sum(arr[x..y])
 *     // return 0 if sum(arr[l..r]) == sum(arr[x..y])
 *     // return -1 if sum(arr[l..r]) < sum(arr[x..y])
 *     int compareSub(int l, int r, int x, int y);
 *
 *     // Returns the length of the array
 *     int length();
 * };
 */

class ArrayReader {
public:
    // Compares the sum of arr[l..r] with the sum of arr[x..y]
    // return 1 if sum(arr[l..r]) > sum(arr[x..y])
    // return 0 if sum(arr[l..r]) == sum(arr[x..y])
    // return -1 if sum(arr[l..r]) < sum(arr[x..y])
    int compareSub(int l, int r, int x, int y);
    // Returns the length of the array
    int length();
};

class Solution {
public:
    int getIndex(ArrayReader &reader) {
        int left = 0;
        int right = reader.length() - 1;

        while (left < right) {
            int mid = left + (right - left) / 2;
            bool is_odd = (right - left) % 2 == 1;
            int res = 0;
            if (is_odd) {
                res = reader.compareSub(left, mid, mid + 1, right);
            } else {
                res = reader.compareSub(left, mid - 1, mid + 1, right);
            }
            if (res == 1) {
                if (is_odd) {
                    right = mid;
                } else {
                    right = mid - 1;
                }
            } else if (res == 0) {
                return mid;
            } else {
                left = mid + 1;
            }
        }

        return left;
    }
};

int main() {}