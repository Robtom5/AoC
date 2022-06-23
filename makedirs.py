from pathlib import Path

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

years = range(2015, 2022)
days = range(1, 26)
other_files = {
    "example.txt": "",
    "src.txt": "",
    "__init__.py": "from soln import task_1, task_2",
}

for year in years:
    year_path = Path(f"AOC_{str(year)}")
    for day in days:
        padded_day = f"{day:0>2}"
        day_path = Path(year_path, f"Day{padded_day}")
        day_path.mkdir(exist_ok=True)
        soln_file = Path(day_path, "soln.py")
        if not soln_file.exists():
            with open(soln_file, "w") as fh:
                fh.write(PYTHONTEMPLATE)

        for other_file in other_files:
            file_path = Path(day_path, other_file)
            if not file_path.exists():
                with open(file_path, "w") as fh:
                    fh.write(other_files[other_file].format(day=padded_day))
