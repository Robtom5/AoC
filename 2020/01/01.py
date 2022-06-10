DEBUG = True

def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    return content

def find_soln(values, *,first_val=0,target=2020):
    while len(values):
        v1 = values.pop(0) 
        for v2 in values:
            if v1 + v2 + first_val == target:
                return (v1, v2)
        else:
            continue
        break
    else:
        return None


def task_1():
    content = read_src()
    values = [int(c) for c in content]

    soln = find_soln(values)
    res = soln[0] * soln[1] if not soln is None else "Not found"

    print(f"task 1: {res}")


def task_2():
    content = read_src()
    values = [int(c) for c in content]

    while len(values):
        v1 = values.pop(0)
        copy = values.copy()
        soln = find_soln(copy, first_val=v1)
        if not soln is None:
            res = soln[0] * soln[1] * v1
            break
    else:
        res = "Not found"  

    print(f"task 2: {res}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()

