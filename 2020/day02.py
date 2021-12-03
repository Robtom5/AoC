import re

line_rule = r'(?P<min>\d+)-(?P<max>\d+) (?P<character>\D): (?P<password>\D+)'
line_regex = re.compile(line_rule)

with open("./Day02Input.txt") as infile:
    lines = infile.readlines()

def parse_line(line):
    result = line_regex.match(line)
    minOccurences = int(result.group('min'))
    maxOccurences = int(result.group('max'))
    character = result.group('character')
    password = result.group('password')
    occurences = password.count(character)
    valid = minOccurences <= occurences <= maxOccurences
    return valid

valid = [line for line in lines if parse_line(line)]
print("Part 1 valid: " + str(len(valid)))

# part 2

def parse_line_2(line):
    result = line_regex.match(line)
    firstIndex = int(result.group('min')) - 1
    secondIndex = int(result.group('max')) - 1
    character = result.group('character')
    password = result.group('password')
    presentAtFirst = password[firstIndex] == character
    presentAtSecond = password[secondIndex] == character
    valid = presentAtFirst ^ presentAtSecond
    return valid

valid2 = [line for line in lines if parse_line_2(line)]
print("Part 2 valid: " + str(len(valid2)))