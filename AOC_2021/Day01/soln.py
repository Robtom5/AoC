def task_1():
    with open("./src.txt", "r") as fh:
        lines = fh.readlines()

    task_data = [int(line) for line in lines]

    steps_greater = 0
    last_val = task_data.pop(0)

    while task_data:
        next_val = task_data.pop(0)
        if next_val > last_val:
            steps_greater += 1

        last_val = next_val

    print(steps_greater)


def task_2():
    with open("./src.txt", "r") as fh:
        lines = fh.readlines()

    task_data_shift_f = [int(line) for line in lines][2:]
    task_data_reg = [int(line) for line in lines][1:-1]
    task_data_shift_b = [int(line) for line in lines][:-2]

    task_data = zip(task_data_shift_f, task_data_reg, task_data_shift_b)

    last_window = None
    steps_greater = 0
    for window in task_data:
        if last_window is None:
            last_window = window
            continue
        if sum(window) > sum(last_window):
            steps_greater += 1

        last_window = window

    print(steps_greater)


if __name__ == "__main__":
    task_1()
    task_2()
