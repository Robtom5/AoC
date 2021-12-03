import string

with open("./Day06Input.txt") as infile:
    contents = infile.read()

groups = contents.split('\n\n')

ALPHABET = list(string.ascii_lowercase)

def count_per_group(group):
    count = 0
    for letter in ALPHABET:
        count += 1 if letter in group else 0
    return count

total = sum([count_per_group(g) for g in groups])
print(total)

# Part 2

def count_in_group(group):
    count = 0
    people = group.split('\n')
    people = [p for p in people if p != '']
    countOfGroup = len(people)
    for letter in ALPHABET:
        countInGroup = group.count(letter)
        count += 1 if countInGroup == countOfGroup else 0
    return count

total2 = sum([count_in_group(g) for g in groups])
print(total2)