TARGET = 2020

with open("./Day01Input.txt") as infile:
    values = []
    for l in infile.readlines():
        values.append(int(l))

values.sort()
greater = [v for v in values if v >= TARGET/2]
lesser = [v for v in values if v < TARGET/2]

brute = [G * L for G in greater for L in lesser if G + L == TARGET]
greatest = max(brute)

# Part 2

brute_2 = [A * B * C
           for A in values
           for B in values
           for C in values
           if A + B + C == TARGET]

greatest_2 = max(brute_2)
print(greatest_2)