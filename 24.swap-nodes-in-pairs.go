package main

/*
 * @lc app=leetcode id=24 lang=golang
 *
 * [24] Swap Nodes in Pairs
 */

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func swapPairs(head *ListNode) *ListNode {
	if head == nil || head.Next == nil {
		return head
	}

	nextNode := head.Next
	head.Next = swapPairs(head.Next.Next)
	nextNode.Next = head

	return nextNode
}

// @lc code=end

type ListNode struct {
	Val  int
	Next *ListNode
}
