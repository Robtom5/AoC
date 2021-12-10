DEBUG = True

expected_close = {
    "(": ")",
    "[": "]",
    "{": "}",
    "<": ">",
}

illegal_score = {")": 3, "]": 57, "}": 1197, ">": 25137}
complete_score = {")": 1, "]": 2, "}": 3, ">": 4}


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()

    return content


def task_1():
    content = read_src()
    total_score = 0
    for line in content:
        open_chunks = []
        chars = line.strip()
        for char in chars:
            if char in expected_close:
                open_chunks.append(expected_close[char])
            elif open_chunks[-1] == char:
                open_chunks.pop()
            else:
                total_score += illegal_score.get(char)
                break
        else:
            if open_chunks:

                # Incomplete
                pass

    print(f"task_1: {total_score}")


def task_2():

    content = read_src()
    total_scores = []
    for line in content:
        open_chunks = []
        chars = line.strip()
        for char in chars:
            if char in expected_close:
                open_chunks.append(expected_close[char])
            elif open_chunks[-1] == char:
                open_chunks.pop()
            else:
                # Invalid
                break
        else:
            if open_chunks:
                score = 0
                open_chunks.reverse()
                for chunk in open_chunks:
                    score *= 5
                    score += complete_score.get(chunk)
                total_scores.append(score)

    total_scores = sorted(total_scores)
    total_score = total_scores[int(len(total_scores) / 2)]

    print(f"task_2: {total_score}")


if __name__ == "__main__":
    DEBUG = False

    task_1()

    task_2()
