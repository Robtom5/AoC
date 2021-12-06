def read_src():
    with open("src.txt", "r") as fh:
        content = fh.readlines()
    #     content = """7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    # 22 13 17 11  0
    #  8  2 23  4 24
    # 21  9 14 16  7
    #  6 10  3 18  5
    #  1 12 20 15 19

    #  3 15  0  2 22
    #  9 18 13 17  5
    # 19  8  7 25 23
    # 20 11 10 24  4
    # 14 21 16 12  6

    # 14 21 17 24  4
    # 10 16 15  9 19
    # 18  8 23 26 20
    # 22 11 13  6  5
    #  2  0 12  3  7""".split(
    #         "\n"
    #     )
    sequence = (int(num) for num in content.pop(0).split(","))
    # strip blanks
    content = [c for c in content if len(c) > 2]

    grids = []
    while len(content) >= 5:
        grid_def = []
        for x in range(5):
            line = content.pop(0)
            grid_def.extend([int(num.strip()) for num in line.split(" ") if num])

        grids.append(BingoGrid(grid_def))

    return sequence, grids


class BingoGrid:
    def __init__(self, grid_def):
        self.grid = grid_def
        assert len(grid_def) == 25

    def number_called(self, num):
        self.grid = [x if x != num else -1 for x in self.grid]

    def check_for_win(self):
        return any([self.check_row(), self.check_col(), self.check_diag()])

    def check_row(self):
        for x in range(5):
            row = self.grid[x * 5 : (x * 5) + 5]
            if all([val == -1 for val in row]):
                return True
        else:
            return False

    def check_col(self):
        for x in range(5):
            col = self.grid[x::5]
            if all([val == -1 for val in col]):
                return True
        else:
            return False

    def check_diag(self):
        forward_diag = self.grid[0::6]
        read_diag = self.grid[5::4]

        if all([val == -1 for val in forward_diag]):
            return True
        elif all([val == -1 for val in read_diag]):
            return True
        else:
            return False

    def score(self, final_val):
        clean_slate = [x if x != -1 else 0 for x in self.grid]
        return final_val * sum(clean_slate)


def task_1():
    sequ, grids = read_src()

    score = -1
    for call in sequ:
        for grid in grids:
            grid.number_called(call)
            if grid.check_for_win():
                score = grid.score(call)

        if score >= 0:
            break

    else:
        raise Exception("No grid won")

    print(f"task 1: {score}")


def task_2():
    sequ, grids = read_src()

    score = -1
    for call in sequ:
        for grid in grids:
            grid.number_called(call)

        if len(grids) > 1:
            grids = [grid for grid in grids if not grid.check_for_win()]
        else:
            if grids[0].check_for_win():
                score = grids[0].score(call)
                break

    else:
        raise Exception("No grid won")

    print(f"task 2: {score}")


if __name__ == "__main__":
    task_1()
    task_2()
