def read_src():
    with open("src.txt", "r") as fh:
        content = fh.readlines()

    return content


def task_1():
    lines = read_src()

    num_lines = len(lines)

    average_bits = [0] * len(lines[0].strip())

    for line in lines:
        line = line.strip()
        index = 0
        for col in line:
            average_bits[index] += int(col)
            index += 1

    gamma_binary = [(x / num_lines) >= 0.5 for x in average_bits]
    epsilon_binary = [(x / num_lines) < 0.5 for x in average_bits]

    gamma = 0
    for bit in gamma_binary:
        gamma = (gamma << 1) | bit

    epsilon = 0
    for bit in epsilon_binary:
        epsilon = (epsilon << 1) | bit

    print(f"gamma: {gamma} epsilon: {epsilon} product: {gamma*epsilon}")


def task_2():
    lines = [x.strip() for x in read_src()]

    lines_length = len(lines[0].strip())

    def calc_mcb(all_lines):
        average_bits = [0] * lines_length

        num_lines = len(all_lines)
        for line in all_lines:
            line = line.strip()
            index = 0
            for col in line:
                average_bits[index] += int(col)
                index += 1

        return "".join(["1" if (x / num_lines) >= 0.5 else "0" for x in average_bits])

    def calc_lcb(all_lines):
        average_bits = [0] * lines_length

        num_lines = len(all_lines)
        for line in all_lines:
            line = line.strip()
            index = 0
            for col in line:
                average_bits[index] += int(col)
                index += 1

        return "".join(["0" if (x / num_lines) >= 0.5 else "1" for x in average_bits])

    mcb_binary = calc_mcb(lines)
    lcb_binary = calc_lcb(lines)

    ox_r = 0
    co_r = 0

    depth = 0

    filtered_lines = lines
    most_common = calc_mcb(filtered_lines)
    while depth <= lines_length:
        filtered_lines = [
            x for x in filtered_lines if x.startswith(most_common[:depth])
        ]
        if len(filtered_lines) == 1:
            for bit in filtered_lines[0]:
                bit = int(bit)
                ox_r = (ox_r << 1) | bit
            break
        if len(filtered_lines) <= 0:
            continue
        depth += 1
        most_common = calc_mcb(filtered_lines)
    else:
        raise Exception("can't find ox")

    depth = 0
    filtered_lines = lines
    less_common = calc_lcb(filtered_lines)
    while depth <= lines_length:

        filtered_lines = [x for x in filtered_lines if x[depth] == less_common[depth]]

        if len(filtered_lines) == 1:
            for bit in filtered_lines[0]:
                bit = int(bit)
                co_r = (co_r << 1) | bit
            break
        if len(filtered_lines) <= 0:
            continue

        depth += 1
        less_common = calc_lcb(filtered_lines)

    else:
        raise Exception("can't find co")

    print(f"ox_r: {ox_r} co_r: {co_r} product: {ox_r*co_r}")


if __name__ == "__main__":
    task_1()
    task_2()
