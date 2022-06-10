DEBUG = True

TREE = "#"

# SLOPE
SLOPE = 3


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    return content


def task_1():
    content = read_src()
    width = len(content[0])
    trees = 0
    index = 0
    for line in content:
        hit = line[index] == TREE
        trees += hit
        index = (index + SLOPE) % width

    print(f"task 1: {trees}")


def task_2():
    def traverse_content(h_v, w_v):
        trees = 0
        h_index = 0
        w_index = 0

        h_v = max(h_v, 1)
        while h_index < len(content):
            hit = content[h_index][w_index] == TREE
            trees += hit
            w_index = (w_index + w_v) % width
            h_index += h_v

        return trees

    content = read_src()
    width = len(content[0])

    slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]

    trees = 1
    for h_v, w_v in slopes:
        trees *= traverse_content(h_v, w_v)

    print(f"task 2: {trees}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
