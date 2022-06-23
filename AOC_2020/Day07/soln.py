import re

DEBUG = True
CONTENT_REGEX = re.compile(r"(\d+) (\D+) bag")


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    return content


def load_rules():
    raw_text = read_src()

    rules = {}
    for line in raw_text:
        line = line.rstrip(".")
        color, content = line.split(" bags contain ")

        rules[color] = [(int(num), col) for num, col in CONTENT_REGEX.findall(content)]

    return rules


def find_color(color, target_color, rules):
    nested_bags = rules[color]
    if nested_bags:
        contains_target = False
        for _, col in nested_bags:
            if col == target_color:
                return True
            else:
                contains_target |= find_color(col, target_color, rules)
        return contains_target
    else:
        return False


def load_contents(color, rules):
    nested_bags = rules[color]
    nested_content = 0
    if nested_bags:
        for num, col in nested_bags:
            nested_content += num
            nested_content += num * load_contents(col, rules)
    return nested_content


def task_1(rules):
    target_color = "shiny gold"
    res = sum([find_color(c, target_color, rules) for c in rules])

    print(f"task 1: {res}")


def task_2(rules):
    target_color = "shiny gold"
    print(f"task 2: {load_contents(target_color, rules)}")


if __name__ == "__main__":
    DEBUG = False
    ruleset = load_rules()
    task_1(ruleset)
    task_2(ruleset)
