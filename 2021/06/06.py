from multiprocessing import Pool


def fish_dict():
    fish = {}
    for x in range(9):
        fish[x] = 0
    return fish


def read_src():
    with open("src.txt", "r") as fh:
        content = fh.readlines()

    # content = "3,4,3,1,2"
    content = content[0]
    fish_count = fish_dict()
    for num in content.split(","):
        fish_count[int(num)] = fish_count.get(int(num), 0) + 1

    return fish_count


def task_1():
    fish = read_src()

    # days = 18
    days = 80
    for day in range(days):

        next_fish = fish_dict()
        for x in range(0, 8):
            next_fish[x] = fish[x + 1]
        next_fish[8] = fish[0]
        next_fish[6] += fish[0]

        fish = next_fish

    total_fish = sum([fish[days] for days in fish])
    print(f"task 1: {total_fish}")


def task_2():
    fish = read_src()

    days = 256
    for day in range(days):

        next_fish = fish_dict()
        for x in range(0, 8):
            next_fish[x] = fish[x + 1]
        next_fish[8] = fish[0]
        next_fish[6] += fish[0]

        fish = next_fish

    total_fish = sum([fish[days] for days in fish])
    print(f"task 2: {total_fish}")


if __name__ == "__main__":
    task_1()
    task_2()
