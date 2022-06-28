DEBUG = False


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    return content


def task_1():
    content = read_src()

    res = None
    return res


def task_2():
    content = read_src()

    res = None
    return res



if __name__ == "__main__":
    DEBUG = True
    print(f"task 1: {task_1()}")
    print(f"task 2: {task_2()}")

