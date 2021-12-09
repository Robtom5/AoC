from math import prod

DEBUG = True


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()

    return content


def surround_greater(index, pt_list, width):
    l_pt = pt_list[index - 1] > pt_list[index] if index % width != 0 else True
    r_pt = (
        pt_list[index + 1] > pt_list[index]
        if index % width != width - 1 and index + 1 < len(pt_list)
        else True
    )
    u_pt = pt_list[index - width] > pt_list[index] if index - width >= 0 else True
    d_pt = (
        pt_list[index + width] > pt_list[index]
        if index + width < len(pt_list)
        else True
    )
    return l_pt, r_pt, u_pt, d_pt


def surround_indexes(index, pt_list, width):
    # DEVNOTE: very inefficiently done
    l, r, u, d = surround_greater(index, pt_list, width)
    l_idx = index - 1 if l and index % width != 0 else None
    r_idx = (
        index + 1
        if r and index % width != width - 1 and index + 1 < len(pt_list)
        else None
    )
    u_idx = index - width if u and index - width >= 0 else None
    d_idx = index + width if d and index + width < len(pt_list) else None
    return l_idx, r_idx, u_idx, d_idx


def is_low_point(index, pt_list, width):
    l_pt, r_pt, u_pt, d_pt = surround_greater(index, pt_list, width)
    return l_pt and r_pt and u_pt and d_pt


def print_around_point(index, pt_list, width):
    l = "\U0001f868"
    u = "\U0001f869"
    r = "\U0001f86a"
    d = "\U0001f86b"
    ul = "\U0001f86c"
    ur = "\U0001f86d"
    dr = "\U0001f86e"
    dl = "\U0001f86f"

    l_pt, r_pt, u_pt, d_pt = surround_greater(index, pt_list, width)

    print(f"{d if u_pt else u:^9}")
    print(
        f"{(r if l_pt else l) + (point_dir(index, pt_list, width)) + (l if r_pt else r):^9}"
    )
    print(f"{u if d_pt else d:^9}")


def point_dir(index, pt_list, width):
    l = "\U0001f868"
    u = "\U0001f869"
    r = "\U0001f86a"
    d = "\U0001f86b"
    ul = "\U0001f86c"
    ur = "\U0001f86d"
    dr = "\U0001f86e"
    dl = "\U0001f86f"
    bot = "X"
    top = " "
    l_pt, r_pt, u_pt, d_pt = surround_greater(index, pt_list, width)

    if pt_list[index] == 9:
        return top
    if l_pt and r_pt and u_pt and d_pt:
        return bot
    elif u_pt:
        if l_pt and not r_pt:
            return dr
        elif r_pt and not l_pt:
            return dl
        elif d_pt:
            return top
        else:
            return d
    elif d_pt:
        if l_pt and not r_pt:
            return ur
        elif r_pt and not l_pt:
            return ul
        elif u_pt:
            return top
        else:
            return u
    elif l_pt:
        return r
    elif r_pt:
        return l
    else:
        return top

    print(f" {d if u_pt else u} ")
    print(f"{r if l_pt else l} {l if r_pt else r}")
    print(f" {u if d_pt else d} ")


def task_1():
    content = read_src()
    flat_map = [int(x) for y in content for x in y.strip()]
    width = len(content[0].strip())

    def low_point_risk(index, pt_list, width):
        if is_low_point(index, pt_list, width):
            return pt_list[index] + 1
        else:
            return 0

    print(
        f"task 1: {sum([low_point_risk(i, flat_map, width) for i in range(len(flat_map))])}"
    )


def task_2():
    content = read_src()
    flat_map = [int(x) for y in content for x in y.strip()]
    width = len(content[0].strip())

    max_height = 9

    def basin_size(start_index):
        if flat_map[start_index] == 9:
            return set()
        covered_pts = set([start_index])
        greater_idx = surround_indexes(start_index, flat_map, width)
        for pt in greater_idx:
            if pt is not None:
                covered_pts = covered_pts.union(basin_size(pt))
        return covered_pts

    low_pts = [i for i in range(len(flat_map)) if is_low_point(i, flat_map, width)]
    sizes = sorted([len(basin_size(low_pt)) for low_pt in low_pts])

    print(f"task 2: {prod(sizes[-3:])}")


def print_map():
    DEBUG = True
    content = read_src()
    flat_map = [int(x) for y in content for x in y.strip()]
    width = len(content[0].strip())
    dirs = [point_dir(i, flat_map, width) for i in range(len(flat_map))]
    for row in range(0, len(flat_map), width):
        print("".join(dirs[row : row + width]))


if __name__ == "__main__":
    DEBUG = False
    task_1()

    task_2()
