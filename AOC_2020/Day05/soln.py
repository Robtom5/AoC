DEBUG = True

row_man = 127
col_max = 7

# Equivalent to binary with MSB


def parse_loc(line):
    row_loc = line[:-3].replace("B", "1").replace("F", "0")
    col_loc = line[-3:].replace("R", "1").replace("L", "0")

    row = int(row_loc, 2)
    col = int(col_loc, 2)

    return row, col


def seat_id(row, col):
    return (8 * row) + col


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    return content


def task_1():
    content = read_src()

    id_max = max([seat_id(*parse_loc(l)) for l in content])

    print(f"task 1: {id_max}")


def task_2():
    content = read_src()

    locations = [parse_loc(l) for l in content]

    min_row = min(locations, key=lambda x: x[0])[0]
    max_row = max(locations, key=lambda x: x[0])[0]

    min_id = (min_row + 1) * 8
    max_id = (max_row) * 8

    available_ids = set(range(min_id, max_id))
    taken_ids = set([seat_id(*parse_loc(l)) for l in content])

    assert len(available_ids - taken_ids) == 1
    print(f"task 2: {(available_ids - taken_ids).pop()}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
