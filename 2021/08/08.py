DEBUG = True

num_lights = {1: [], 2: [1], 3: [7], 4: [4], 5: [2, 3, 5], 6: [0, 6, 9], 7: [8]}


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()

    split_content = [
        (line.strip().split(" | ")[0], line.strip().split(" | ")[1]) for line in content
    ]

    return split_content


def task_1():
    split_content = read_src()

    count = 0
    for _, output in split_content:
        for digit in output.split(" "):
            lights_on = len(digit.strip())
            if len(num_lights[lights_on]) == 1:
                count += 1

    print(f"task 1: {count}")


def task_2():
    ideal = {
        0: set("abcefg"),
        1: set("cf"),
        2: set("acdeg"),
        3: set("acdfg"),
        4: set("bcdf"),
        5: set("abdfg"),
        6: set("abdefg"),
        7: set("acf"),
        8: set("abcdefg"),
        9: set("abcdfg"),
    }
    potential = {}
    for letter in "abcdefg":
        potential[letter] = [x for x in ideal if letter in ideal[x]]

    split_content = read_src()

    total_sum = 0
    for patterns, output in split_content:
        num_codes = [set(c) for c in patterns.split(" ")]
        # dumb way
        """
            [t]op
        [ul]    [ur]
            [c]entre
        [ll]    [lr]
            [b]ot

          tttt
        ul    ur
        ul    ur
        ul    ur
          cccc
        ll    lr
        ll    lr
        ll    lr
          bbbb

        """
        # work out mapping for A
        def_1 = [x for x in num_codes if len(x) == 2][0]
        def_4 = [x for x in num_codes if len(x) == 4][0]
        def_7 = [x for x in num_codes if len(x) == 3][0]
        def_8 = [x for x in num_codes if len(x) == 7][0]

        t = def_7 - def_1
        ulc = def_4 - def_1

        val = 0
        for scrambled in output.split(" "):
            scrambled = set(scrambled)
            val *= 10
            new_val = 0
            if len(scrambled) == 2:
                new_val = 1
            elif len(scrambled) == 3:
                new_val = 7
            elif len(scrambled) == 4:
                new_val = 4
            elif len(scrambled) == 5:
                # work out if 2,3,5
                if def_1 <= scrambled:
                    new_val = 3
                elif ulc <= scrambled:
                    new_val = 5
                else:
                    new_val = 2

            elif len(scrambled) == 6:
                if not ulc <= scrambled:
                    new_val = 0
                elif not def_1 <= scrambled:
                    new_val = 6
                else:
                    new_val = 9
            elif len(scrambled) == 7:
                new_val = 8

            val += new_val
        total_sum += val

    print(f"task 2: {total_sum}")


if __name__ == "__main__":
    DEBUG = False
    task_1()

    task_2()
