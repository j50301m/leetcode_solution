package main

/*
 * @lc app=leetcode id=20 lang=golang
 *
 * [20] Valid Parentheses
 */

// @lc code=start

func isValid(s string) bool {
	stack := make([]rune, 0)

	brackets := map[rune]rune{
		')': '(',
		']': '[',
		'}': '{',
	}

	for _, char := range s {
		if closeStr, ok := brackets[char]; ok {
			if len(stack) == 0 || stack[len(stack)-1] != closeStr {
				return false
			}
			stack = stack[:len(stack)-1]
		} else {
			stack = append(stack, char)
		}
	}

	return len(stack) == 0
}

// @lc code=end

func _main() {
	result := isValid("{{[]}}")
	println(result)
}
