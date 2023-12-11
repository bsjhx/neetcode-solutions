/**
 * Definition for singly-linked learning_with_linked_list.
 * function ListNode(val, next) {
 *     this.val = (val===undefined ? 0 : val)
 *     this.next = (next===undefined ? null : next)
 * }
 */
/**
 * @param {ListNode} head
 * @return {ListNode}
 */

function ListNode(val, next) {
    this.val = (val===undefined ? 0 : val)
    this.next = (next===undefined ? null : next)
}
var reverseList = function(head) {
    let [ prev, curr, next ] = [ null, head, null ];

    while (curr) {
        next = curr.next;
        curr.next = prev;

        prev = curr;
        curr = next;
    }

    return prev;
};