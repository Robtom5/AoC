DEBUG = True


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    return content


def loop_eval(instructions):
    stuck_in_loop = False
    accumulator = 0

    pointer = 0
    lines_hit = set()
    while not stuck_in_loop and pointer < len(instructions):
        if pointer in lines_hit:
            stuck_in_loop = True
        else:
            lines_hit.add(pointer)
            instr, val = instructions[pointer].split(" ")
            if instr == "jmp":
                pointer += int(val)
                continue
            elif instr == "acc":
                accumulator += int(val)
            pointer += 1
    return stuck_in_loop, accumulator


def task_1():
    content = read_src()
    _, acc = loop_eval(content)

    print(f"task 1: {acc}")


def task_2():
    content = read_src()

    error_cmds = {"nop", "jmp"}
    potential_faults = [
        index
        for index in range(len(content))
        if content[index].split(" ")[0] in error_cmds
    ]
    for index in potential_faults:
        original = content[index]
        cmd, val = original.split(" ")
        if cmd == "jmp":
            cmd = "nop"
        else:
            cmd = "jmp"
        new = " ".join([cmd, val])
        content[index] = new
        fault, acc = loop_eval(content)

        content[index] = original
        if not fault:
            break
    else:
        print("ERROR: No potential fix")

    print(f"task 2: {acc}. Fix at {index}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
