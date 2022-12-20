from typing import Optional, List


class Node:
    def __init__(self, data: int):
        self.data = data
        self.next: Optional[Node] = None
        self.prev: Optional[Node] = None

    def __repr__(self):
        return str(self.data)


class LinkedList:
    def __init__(self, nodes):
        self.original: List[Optional[Node]] = [None] * len(nodes)
        self.length_minus_one = len(nodes) - 1
        node = Node(nodes[0])
        self.original[0] = node
        self.head = node
        head = node
        iter_nodes = iter(nodes)
        next(iter_nodes)
        for i, elem in zip(range(1, len(nodes)), iter_nodes):
            node.next = Node(elem)
            node.next.prev = node
            node = node.next
            self.original[i] = node
            if node.data == 0:
                head = node
        node.next = self.head
        self.head.prev = node
        self.head = head

    def __repr__(self):
        node = self.head
        nodes = []
        while True:
            nodes.append(node.data)
            node = node.next
            if node == self.head:
                break
        return " -> ".join(map(str, nodes))

    def move(self, node: Node):
        if node.data == 0:
            return
        next_node = node
        node.prev.next = node.next
        node.next.prev = node.prev

        steps = abs(node.data) % self.length_minus_one
        if node.data > 0:
            for i in range(steps):
                next_node = next_node.next
            node.prev.next = node.next
            node.next.prev = node.prev

            next_node.next.prev = node
            node.next = next_node.next

            next_node.next = node
            node.prev = next_node
        else:
            for i in range(steps):
                next_node = next_node.prev
            node.prev.next = node.next
            node.next.prev = node.prev

            next_node.prev.next = node
            node.prev = next_node.prev

            next_node.prev = node
            node.next = next_node

    def get(self, index: int):
        node = self.head
        for i in range(index):
            node = node.next
        return node


def part1(data: str):
    l = LinkedList(list(map(int, data.splitlines())))
    for node in l.original:
        l.move(node)
    return l.get(1000).data + l.get(2000).data + l.get(3000).data


def part2(data: str):
    l = LinkedList(list(map(lambda n: int(n) * 811589153, data.splitlines())))
    for _ in range(10):
        for node in l.original:
            l.move(node)
    return l.get(1000).data + l.get(2000).data + l.get(3000).data


if __name__ == "__main__":
    test = True
    test = False
    test_input = """1
2
-3
3
-2
0
4"""
    if test:
        puzzle_input = test_input
    else:
        with open("day_20_input.txt", "r") as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
