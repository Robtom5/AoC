import string

DEBUG = True
ALPHABET = set(string.ascii_lowercase)


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read()
    content = content.split("\n\n")
    return content


def count_per_group(group):
    return len(set(group).intersection(ALPHABET))


def count_in_group(group):
    this_group = set(group)
    for p in group.split("\n"):
        this_group = this_group.intersection(set(p))

    count = len(this_group)
    return count


def task_1():
    content = read_src()
    total = sum((count_per_group(g) for g in content))
    print(f"task 1: {total}")


def task_2():
    content = read_src()
    total = sum((count_in_group(g) for g in content))
    print(f"task 2: {total}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
