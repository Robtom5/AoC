DEBUG = True
import math


class Pair:
    def __init__(self, left, right=None):
        self.left = left
        self.right = right
        self.parent = None

    def set_as_left(self, parent):
        self.parent = parent
        self.is_left = True

    def set_as_right(self, parent):
        self.parent = parent
        self.is_left = False

    @classmethod
    def nest_pairs(cls, left, right):
        new_pair = Pair(left, right)
        left.set_as_left(new_pair)
        right.set_as_right(new_pair)
        return new_pair

    def __repr__(self):
        if not self.is_root:
            return f"[{self.left},{self.right}]"
        else:
            # return f"[{self.left},{self.right}]"
            # return f"{self.left}@{self.depth}"
            return f"{self.left}"

    def __radd__(self, other):
        if self.is_root:
            return self.left + other
        else:
            raise TypeError(f"Can only add root pairs. Tried {self} {other}")

    @property
    def depth(self):
        if self.parent is None:
            return 0
        else:
            return self.parent.depth + 1

    @property
    def is_root(self):
        return self.right is None

    @property
    def magnitude(self):
        if self.right is None:
            return self.left
        if isinstance(self.left, Pair):
            left_mag = self.left.magnitude
        else:
            left_mag = self.left
        if isinstance(self.right, Pair):
            right_mag = self.right.magnitude
        else:
            right_mag = self.right
        return 3 * left_mag + 2 * right_mag

    @property
    def will_split(self):
        if self.is_root:
            return self.left > 9

    @property
    def will_explode(self):
        return not self.is_root and self.depth >= 4

    @property
    def child_need_reduction(self):
        if not self.is_root:
            return self.left.needs_reduction or self.right.needs_reduction
        else:
            return False

    @property
    def needs_reduction(self):
        return self.will_split or self.will_explode or self.child_need_reduction

    def reduce(self):
        # TODO: apply the explodes then splits in correct
        # order. not just first come first serve

        # Loop through all node checking if any can explode,
        # then if nothing explodes, split, and so on
        if self.is_root:
            self.split()
        if self.needs_reduction:
            self.explode()
            if not self.is_root:
                self.left.reduce()
                self.right.reduce()

    def split(self):
        if isinstance(self.left, int) and self.left > 9:
            new_left = Pair(math.floor(self.left / 2))
            new_right = Pair(math.ceil(self.left / 2))
            self.left, self.right = new_left, new_right
            self.left.set_as_left(self)
            self.right.set_as_right(self)

            if self.needs_reduction:
                self.left.reduce()

    def explode(self):
        if self.depth >= 4:
            # Find a node to the left
            curr_node = self
            finding_left = True
            while curr_node.depth > 0:
                if curr_node.is_left:
                    curr_node = curr_node.parent
                else:
                    curr_node = curr_node.parent.left
                    break
            else:
                # no nodes to the left so check right
                finding_left = False
            while finding_left:
                if curr_node.is_root:
                    curr_node.left += self.left
                    finding_left = False
                else:
                    curr_node = curr_node.right

            curr_node = self
            finding_right = True
            while curr_node.depth > 0:
                if not curr_node.is_left:
                    curr_node = curr_node.parent
                else:
                    curr_node = curr_node.parent.right
                    break
            else:
                # no nodes to the right
                finding_right = False
            while finding_right:
                if curr_node.is_root:
                    curr_node.left += self.right
                    finding_right = False
                else:
                    curr_node = curr_node.left

            self.left = 0
            self.right = None


class SnailfishCalculator:
    @classmethod
    def add(cls, number1, number2):
        res = Pair.nest_pairs(number1, number2)
        SnailfishCalculator.reduce(res)

        i = 0
        # while res.needs_reduction and i < 5:
        #     # print(f"..{res}")
        #     SnailfishCalculator.reduce(res)
        #     i += 1
        SnailfishCalculator.reduce(res)
        # while res.needs_reduction:
        #     print(f"..{res}")
        #     res.reduce()
        # else:
        #     print(f"..{res}")
        return res

    @classmethod
    def reduce(cls, pairs):

        while cls.explode_first(pairs):
            # Exploding
            print(f"E.{pairs}")

        while cls.split_first(pairs):
            print(f"S.{pairs}")
            while cls.explode_first(pairs):
                print(f"..{pairs}")
                pass

        # TODO: apply the explodes then splits in correct
        # order. not just first come first serve

        # Loop through all node checking if any can explode,
        # then if nothing explodes, split, and so on

        # After an explode need to start looking from the left again

        # Need to fix the finding of first to explode
        pass

    @classmethod
    def explode_first(cls, pairs):
        curr_node = pairs

        while curr_node is not None:
            # Explode if need to
            if curr_node.will_explode:
                curr_node.explode()
                return True

            if not curr_node.is_root:
                if not curr_node.left.is_root:
                    curr_node = curr_node.left
                    continue
                elif not curr_node.right.is_root:
                    curr_node = curr_node.right
                    continue

            if curr_node.is_left:
                curr_node = curr_node.parent.right
            else:
                while True:
                    if curr_node.depth == 0:
                        return False
                    if curr_node.is_left:
                        curr_node = curr_node.parent.right
                    else:
                        curr_node = curr_node.parent
                else:
                    # No mode nodes to examine
                    curr_node = None
                    break
        return False

    @classmethod
    def split_first(cls, pairs):
        curr_node = pairs
        while curr_node is not None:
            if curr_node.will_split:
                curr_node.split()
                print("split ", end="")
                return True

            if not curr_node.is_root:
                if not curr_node.left.is_root:
                    curr_node = curr_node.left
                    continue
                elif not curr_node.right.is_root:
                    curr_node = curr_node.right
                    continue

            if curr_node.is_left:
                curr_node = curr_node.parent.right
            else:
                # Go up until find next right
                while True:
                    if curr_node.depth == 0:
                        return False
                    if curr_node.is_left:
                        curr_node = curr_node.parent.right
                    else:
                        curr_node = curr_node.parent
                else:
                    # No mode nodes to examine
                    curr_node = None
                    break
        # print("done")
        return False


def load_from_str(string_repr):
    # Remove braces
    contents = string_repr[1:-1]

    # find mid point
    half_point = 0
    depth = 0
    for i in range(len(contents)):
        char = contents[i]
        if char == "[":
            depth += 1
        elif char == "]":
            depth -= 1
        elif char == "," and depth == 0:
            half_point = i
            break
    else:
        raise Exception(f"Never found half point for {string_repr}")

    first_half = contents[:half_point]
    second_half = contents[half_point + 1 :]

    try:
        first_half = Pair(int(first_half), None)
    except ValueError:
        first_half = load_from_str(first_half)

    try:
        second_half = Pair(int(second_half), None)
    except ValueError:
        second_half = load_from_str(second_half)

    return Pair.nest_pairs(first_half, second_half)


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()

    lines = [load_from_str(c.strip()) for c in content]

    return lines


def task_1():
    content = read_src()
    rolling_sum = content.pop(0)
    for number in content:
        print(f"  {rolling_sum}")
        print(f"+ {number}")
        rolling_sum = SnailfishCalculator.add(rolling_sum, number)
        print(f"= {rolling_sum}")
        print()
    # res = SnailfishCalculator.add(content[0], content[1])
    print(rolling_sum)
    # i = 0
    # while i <= 5 and res.needs_reduction:
    #     res.reduce()
    #     print(res)
    #     i += 1
    print(f"task 1: ")


def task_2():
    content = read_src()
    res = content.pop(0)
    print(res)
    i = 0
    while i <= 5 and res.needs_reduction:
        res.reduce()
        print(res)
        i += 1
    # res = SnailfishCalculator.add(content[0], content[1])
    print(f"task 2: ")


if __name__ == "__main__":
    # DEBUG = False
    task_1()
    # task_2()
    # a = "[[[[0,7],4],[15,[0,13]]],[1,1]]"
    # # a = "[[[[[9,8],1],2],3],4]"
    # test_a = load_from_str(a)
    # print(test_a)
    # test_a.reduce()
    # print(test_a)
