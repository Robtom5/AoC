DEBUG = True

import re

path_regex = re.compile(r"^(?P<node1>\w+)-(?P<node2>\w+)$")


def read_src():
    # NB: No big caves touch other big caves
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()

    node_names = set([])
    nodes = {}
    for line in content:
        match_node_names = path_regex.match(line.strip())
        node_names.add(match_node_names.group("node1"))
        node_names.add(match_node_names.group("node2"))

    # print(sorted(node_names))
    for name in node_names:
        nodes[name] = Node(name)

    for line in content:
        match_node_names = path_regex.match(line.strip())
        nodes[match_node_names.group("node1")].add_link(
            nodes[match_node_names.group("node2")]
        )
        nodes[match_node_names.group("node2")].add_link(
            nodes[match_node_names.group("node1")]
        )

    return nodes


class Node:
    def __init__(self, name):
        self.name = name
        self.connections = set([])

    def add_link(self, otherNode):
        self.connections.add(otherNode)

    @property
    def big(self):
        return self.name.isupper()

    @property
    def isEnd(self):
        return self.name == "end"

    @property
    def isStart(self):
        return self.name == "start"


def explore(node, route):
    if not node.big and node.name in route:
        # Already been to this small node
        return []
    if node.isEnd:
        route.append(node.name)
        return route

    branches = []
    for next_node in node.connections:
        next_route = route.copy()
        next_route.append(node.name)
        branch = explore(next_node, next_route)
        if branch:
            if type(branch[0]) == list:
                for b in branch:
                    branches.append(b)
            else:
                branches.append(branch)

    return branches


def explore_advanced(node, route, has_doubled):
    if not node.big and node.name in route:
        # We reach a small node
        if has_doubled or node.isStart:
            # We've already double visited, or reach the start node again
            return []
        else:
            # Else that was our first double
            has_doubled = True
    if node.isEnd:
        route.append(node.name)
        return route

    branches = []
    for next_node in node.connections:
        next_route = route.copy()
        next_route.append(node.name)
        branch = explore_advanced(next_node, next_route, has_doubled)
        if branch:
            if type(branch[0]) == list:
                for b in branch:
                    branches.append(b)
            else:
                branches.append(branch)

    return branches


def task_1():
    nodes = read_src()
    startNode = nodes["start"]
    endNode = nodes["end"]
    max_depth = len(nodes)
    res = explore(startNode, [])

    print(f"task 1: {len(res)}")


def task_2():
    nodes = read_src()
    startNode = nodes["start"]
    endNode = nodes["end"]
    max_depth = len(nodes)
    res = explore_advanced(startNode, [], False)

    print(f"task 2: {len(res)}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
