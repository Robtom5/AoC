with open("./Day10Input.txt") as infile:
    contents = infile.readlines()


sorted_adapters = [int(l) for l in contents]
sorted_adapters.sort()
last_joltage = 0

differences_of_1 = 0
differences_of_3 = 1 # we know the last difference is the one to out device

for jolt in sorted_adapters:
    difference = jolt - last_joltage
    differences_of_1 += 1 if difference == 1 else 0
    differences_of_3 += 1 if difference == 3 else 0
    last_joltage = jolt

print(f"Do1: {differences_of_1} Do3: {differences_of_3} Product: {differences_of_1 * differences_of_3}")

# Part 2

class adapter_repository():
    MAX_DIFF = 3
    MIN_DIFF = 1

    def __init__(self, adapters):
        self.initialize_adapters(adapters)

    def initialize_adapters(self, raw_adapters):
        self.adapters = [adapter(jolt) for jolt in raw_adapters]
        for adap in self.adapters:
            self.load_connections(adap)

    def load_connections(self, adapter):
        adapter_index = self.adapters.index(adapter)
        for remaining in self.adapters[adapter_index:]:
            if (remaining.joltage > adapter.joltage and remaining.joltage <= adapter.joltage + adapter_repository.MAX_DIFF):   
                adapter.connections.append(remaining)
        return adapter

    def permutations(self):
        return self.adapters[0].total_permutations() + 1


class adapter():
    def __init__(self, joltage):
        self.joltage = joltage
        self.connections = []
        self._total = None

    def num_connections(self):
        return len(self.connections)

    def total_permutations(self):
        if self._total is None:
            impact = max(self.num_connections() - 1, 0) # we discount the 'straight line' permutation
            running_total = impact
            for con in self.connections:
                # print(f"{self.joltage} -> {con.joltage} ", end = '') # ive counted each jump, not permutations
                running_total += con.total_permutations()
            self._total = running_total
        return self._total

# Add the socket
sorted_adapters = [0] + sorted_adapters

repo = adapter_repository(sorted_adapters)
print(repo.permutations())
