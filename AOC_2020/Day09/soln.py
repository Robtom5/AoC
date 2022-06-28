import itertools

DEBUG = False


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    content = [int(c) for c in content]

    return content


def find_pair(target, src):
    first_half = filter(lambda n: n < target / 2, src)
    second_half = filter(lambda n: n > target / 2, src)
    for x, y in itertools.product(first_half, second_half):
        if x + y == target:
            return True, (x, y)
    else:
        return False, (None, None)


def task_1():
    preamble_length = 5 if DEBUG else 25
    content = read_src()

    ptr = preamble_length
    while ptr < len(content):
        valid, pair = find_pair(content[ptr], content[ptr - preamble_length : ptr])
        if not valid:
            res = content[ptr]
            break
        ptr += 1
    else:
        res = None

    return res, ptr


def task_2():
    preamble_length = 5 if DEBUG else 25
    target, max_ptr = task_1()
    content = read_src()

    start_ptr = 0
    end_ptr = 0
    running_total = 0
    while running_total < target:
        running_total += content[end_ptr]
        end_ptr += 1

    while running_total != target and end_ptr < max_ptr:
        if running_total > target:
            running_total -= content[start_ptr]
            start_ptr += 1
        else:
            running_total += content[end_ptr]
            end_ptr += 1

    if running_total == target:
        sliced_content = content[start_ptr:end_ptr]
        res = max(sliced_content) + min(sliced_content)
    else:
        res = None
    return res


if __name__ == "__main__":
    DEBUG = True
    print(f"task 1: {task_1()[0]}")
    print(f"task 2: {task_2()}")

    a = [1, 2, 3, 4]
    find_pair(5, a)
