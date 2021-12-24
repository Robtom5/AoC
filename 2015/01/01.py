DEBUG = True


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    return content


def task_1():
    content = read_src()
    line = content[0]

    print(f"task 1: final floor {line.count('(')-line.count(')')}")


def task_2():
    content = read_src()
    line = content[0]

    current_floor = 0
    position = 1
    for c in line:
        if c == "(":
            current_floor += 1
        else:
            current_floor -= 1

        if current_floor <= -1:
            break
        else:
            position += 1
    print(f"task 2: enters basement @{position}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
