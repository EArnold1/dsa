class ListNode<T> {
  next: null | ListNode<T> = null;
  prev: null | ListNode<T> = null;
  data: T;

  constructor(value: T) {
    this.data = value;
  }
}

class LinkedList<T> {
  head: null | ListNode<T> = null;

  init(node: ListNode<T>) {
    this.head = node;
  }

  // O(1)
  append(node: ListNode<T>) {
    node.next = this.head;

    if (this.head) {
      this.head.prev = node;
    }

    this.head = node;
    node.prev = null;
  }

  // O(1)
  delete(node: ListNode<T>) {
    if (node.prev) {
      node.prev.next = node.next;
    } else {
      this.head = node.next;
    }

    if (node.next) {
      node.next.prev = node.prev;
    }
  }

  // O(n)
  search(target: T) {
    let currentNode = this.head;

    while (currentNode) {
      if (currentNode.data === target) {
        return currentNode;
      } else {
        currentNode = currentNode.next;
      }
    }

    return null;
  }

  printList() {
    let val = this.head;

    while (val) {
      console.info('data:', val.data);
      val = val.next;
    }
  }
}

const newList = new LinkedList();

const nodeA = new ListNode('Arnold');
newList.init(nodeA);

const nodeB = new ListNode('Emmanuel');

const nodeC = new ListNode('Wachukwu');

newList.append(nodeB);
newList.append(nodeC);

newList.printList();
newList.delete(nodeB);
newList.printList();

console.info(newList.search('Arnold')?.data);
