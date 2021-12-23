DEBUG = True


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()

    return content


def task_1():
    content = read_src()
    print(f"task 1: ")


def task_2():
    content = read_src()
    print(f"task 2: ")


if __name__ == "__main__":
    # DEBUG = False
    task_1()
    task_2()
