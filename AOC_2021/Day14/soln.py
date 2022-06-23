DEBUG = True

import re

ins_regex = re.compile(r"^(?P<pair>\w\w) -> (?P<ins>\w)")


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()

    template = content.pop(0).strip()

    ins = {}
    for line in content:
        match = ins_regex.match(line)
        if match is not None:
            ins[match.group("pair")] = match.group("ins")
    return template, ins


def create_pairs_dict(poly):
    pairs = [(poly[i] + poly[i + 1]) for i in range(len(poly) - 1)]
    pairs_dict = {}
    char_dict = {}
    for p in poly:
        char_dict[p] = char_dict.get(p, 0) + 1

    for p in pairs:
        pairs_dict[p] = pairs_dict.get(p, 0) + 1
    return pairs_dict, char_dict


def apply_ins(poly_dict, char_dict, ins):
    next_dict = poly_dict.copy()

    for pair in poly_dict:
        if pair in ins:
            pair_count = poly_dict[pair]
            pre_pair = pair[0] + ins[pair]
            post_pair = ins[pair] + pair[1]
            char_dict[ins[pair]] = char_dict.get(ins[pair], 0) + pair_count
            next_dict[pre_pair] = next_dict.get(pre_pair, 0) + pair_count
            next_dict[post_pair] = next_dict.get(post_pair, 0) + pair_count
            next_dict[pair] = next_dict[pair] - pair_count
    poly_dict.update(next_dict)


def calc_poly_str(char_dict):
    sorted_values = sorted(char_dict.values())
    return sorted_values[-1] - sorted_values[0]


def task_1():
    poly, ins = read_src()
    poly_dict, char_dict = create_pairs_dict(poly)
    for x in range(10):
        apply_ins(poly_dict, char_dict, ins)
    print(f"task 1: {calc_poly_str(char_dict)}")


def task_2():
    poly, ins = read_src()
    poly_dict, char_dict = create_pairs_dict(poly)
    for x in range(40):
        apply_ins(poly_dict, char_dict, ins)
    print(f"task 2: {calc_poly_str(char_dict)}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
