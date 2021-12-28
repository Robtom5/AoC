DEBUG = True

disallowed = ["ab", "cd", "pq", "xy"]
vowels = ["a", "e", "i", "o", "u"]


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    return content


def is_string_nice(input_str):
    # check for not allowed
    for d in disallowed:
        if d in input_str:
            return False

    # check enough vowels
    vowel_count = sum(c in vowels for c in input_str)
    if vowel_count < 3:
        return False

    last_char = ""
    for c in input_str:
        if c == last_char:
            return True
        last_char = c

    return False


def is_string_nice_2(input_str):
    padded_str = input_str + "__"
    pairs = {}
    last_pair = "__"
    for i in range(len(input_str)):
        pair = padded_str[i : i + 2]
        pairs[pair] = pairs.get(pair, 0) + 1

        if pairs[pair] > 1 and pair != last_pair:
            break
        elif pairs[pair] > 2:
            break

        last_pair = pair
    else:
        return False

    for i in range(len(input_str)):
        if padded_str[i] == padded_str[i + 2]:
            break
    else:
        return False

    return True


def task_1():
    content = read_src()
    running_sum = 0
    for l in content:
        if is_string_nice(l):
            running_sum += 1
    print(f"task 1: {running_sum}")


def task_2():
    content = read_src()
    running_sum = 0
    for l in content:
        if is_string_nice_2(l):
            running_sum += 1
    print(f"task 2: {running_sum}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
