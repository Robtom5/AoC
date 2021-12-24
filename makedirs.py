import os

PYTHONTEMPLATE = """DEBUG = True


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    return content


def task_1():
    content = read_src()
    print(f"task 1: ")


def task_2():
    content = read_src()
    print(f"task 2: ")


if __name__ == "__main__":
    # DEBUG = False
    task_1()
    task_2()

"""

years = range(2015, 2021)
days = range(1, 26)
other_files = ["example.txt", "src.txt"]

for year in years:
    for day in days:
        padded_day = f"{day:0>2}"
        day_path = os.path.join(str(year), padded_day)
        os.makedirs(day_path, exist_ok=True)
        with open(os.path.join(day_path, f"{padded_day}.py"), "w") as fh:
            fh.write(PYTHONTEMPLATE)
        for other_file in other_files:
            with open(os.path.join(day_path, other_file), "w") as fh:
                fh.write("")
