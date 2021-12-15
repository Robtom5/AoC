DEBUG = True
import operator
import numpy as np


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()

    width = len(content[0].strip())
    height = len(content)

    data_array = np.zeros((height, width))
    for y in range(height):
        if len(content[y].strip()) == width:
            for x in range(width):
                data_array[y][x] = content[y][x]

    return data_array


def increment_risk_scalar(risk):
    risk += 1
    return risk if risk <= 9 else 1


increment_risk = np.vectorize(increment_risk_scalar)


def dijkstra(cavern_map):
    start_index = (0, 0)
    height, width = cavern_map.shape
    end_index = (height - 1, width - 1)
    visited = {}
    unvisited = {(y, x): np.inf for y in range(height) for x in range(width)}
    unvisited[start_index] = 0

    next_nodes = {(0, 0): 0}
    while unvisited:
        node = min(next_nodes, key=next_nodes.get)
        next_nodes.pop(node)

        assess_node(node, cavern_map, visited, unvisited, next_nodes)
        if node == end_index:
            break
    return visited[end_index]


def assess_node(node, cavern_map, visited, unvisited, next_nodes):
    this_node_distance = unvisited[node]
    this_node_value = cavern_map[node]
    adjacent = [(-1, 0), (1, 0), (0, -1), (0, 1)]
    adjacent_indices = [
        tuple(map(operator.add, node, direction)) for direction in adjacent
    ]
    found_next = False
    for adj_node in adjacent_indices:
        y, x = adj_node
        temp_distance = unvisited.get(adj_node, None)
        if temp_distance is not None:
            found_next = True
            value = cavern_map[y][x]
            distance = this_node_distance + value
            if temp_distance > distance:
                unvisited[adj_node] = distance
                pass
                next_nodes[adj_node] = distance
        else:
            # node not exists or already visited
            pass
    visited[node] = this_node_distance
    unvisited.pop(node)


def task_1():
    cavern_map = read_src()
    risk = dijkstra(cavern_map)
    print(f"task 1: {int(risk)}")


def task_2():
    cavern_map = read_src()
    chunk = cavern_map.copy()
    for _ in range(4):
        chunk = increment_risk(chunk)
        cavern_map = np.concatenate((cavern_map, chunk), axis=0)

    chunk = cavern_map.copy()
    for _ in range(4):
        chunk = increment_risk(chunk)
        cavern_map = np.concatenate((cavern_map, chunk), axis=1)
    risk = dijkstra(cavern_map)

    print(f"task 2: {int(risk)}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
