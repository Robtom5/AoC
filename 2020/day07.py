import re

with open("./Day07Input.txt") as infile:
    fileContents = infile.readlines()

RULE_REGEX = re.compile(r'(?P<desc>^\D+ \D+) bags contain ')
CONTENT_REGEX = re.compile(r'(\d+) (\D+ \D+) bag')

def load_rules(rawInput):
    for raw_line in rawInput:
        key = RULE_REGEX.match(raw_line)['desc']
        raw_contents = CONTENT_REGEX.findall(raw_line)
        contents = [(int(num), col) for num, col in raw_contents]
        rules[key] = contents

def load_contents(bagColor):
    nestedBags = rules[bagColor]
    if nestedBags:
        nestedContent = []
        for number, color in nestedBags:
            nestedContent += number * [color]
            nestedContent += number * load_contents(color)
        return nestedContent
    else:
        return []

def find_color(bagColor):
    nestedBags = rules[bagColor]
    if nestedBags:
        containsTarget = False
        for number, color in nestedBags:
            if color == TARGET:
                return True
            else:
                containsTarget |= find_color(color)
        return containsTarget
    else:
        return False

if __name__ == "__main__":
    rules = {}

    load_rules(fileContents)

    TARGET = 'shiny gold'

    total_with_target = 0

    for key in rules:
        if (find_color(key)):
            total_with_target += 1

    print(f"Total containing gold: {total_with_target}")

    print(f"Total inside gold: {len(load_contents(TARGET))}")
