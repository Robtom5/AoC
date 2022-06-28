DEBUG = False


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    content = [int(l) for l in content]
    content.sort()
    return content


class Adapter:
    def __init__(self, joltage, conns):
        self.joltage = joltage
        self.conns = conns
        # We remove the straight line permutation from the count
        self.perms = max(len(conns) - 1, 0) + sum([c.perms for c in conns])


def task_1():
    content = read_src()

    socket_joltage = 0
    device_joltage = content[-1] + 3

    all_joltages = [socket_joltage] + content + [device_joltage]

    differences = map(lambda j1, j2: j1 - j2, all_joltages[1:], all_joltages[:-1])
    differences = list(differences)
    return differences.count(1) * differences.count(3)


def task_2():
    content = read_src()
    socket_joltage = 0
    device_joltage = content[-1] + 3

    all_joltages = [socket_joltage] + content
    all_joltages.reverse()
    device = Adapter(device_joltage, {})
    adapters = {device}
    for joltage in all_joltages:
        conns = {a for a in adapters if a.joltage - joltage <= 3}
        newest_adapter = Adapter(joltage, conns)
        adapters.add(newest_adapter)

    # Add the straight line permutation in
    res = newest_adapter.perms + 1
    return res


if __name__ == "__main__":
    # DEBUG = True
    print(f"task 1: {task_1()}")
    print(f"task 2: {task_2()}")
