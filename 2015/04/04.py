DEBUG = True

import hashlib


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    hash_root = content[0]
    return hash_root


def task_1():
    hash_root = read_src()

    starts_with_zeroes = False
    i = 0
    while not starts_with_zeroes:
        i += 1
        test_str = f"{hash_root}{i}"
        hashed = hashlib.md5(test_str.encode())
        starts_with_zeroes = hashed.hexdigest().startswith("00000")

    print(f"task 1: {i}")


def task_2():
    hash_root = read_src()

    starts_with_zeroes = False
    i = 0
    while not starts_with_zeroes:
        i += 1
        test_str = f"{hash_root}{i}"
        hashed = hashlib.md5(test_str.encode())
        starts_with_zeroes = hashed.hexdigest().startswith("000000")

    print(f"task 2: {i}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
