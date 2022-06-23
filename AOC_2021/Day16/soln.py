DEBUG = True
import sys

if sys.version_info < (3, 8):

    def prod(iterable):
        product = 1
        for i in iterable:
            product *= i
        return product


else:
    from math import prod

SCALE = 16
BIT_SZ = 4


def hex_to_bin(hex_s):
    return bin(int(hex_s, SCALE))[2:].zfill(BIT_SZ)


def binary_to_int(binary):
    return int(binary, 2)


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.readlines()

    binary = "".join([hex_to_bin(h) for h in content[0].strip()])

    return binary


def parse_literal_r(binary):
    if binary.startswith("1"):
        this_char = binary[1:5]
        binary_rep, remaining = parse_literal_r(binary[5:])
        return this_char + binary_rep, remaining
    else:
        return binary[1:5], binary[5:]


def parse_literal(binary):
    value, remaining = parse_literal_r(binary)
    return binary_to_int(value), remaining, binary.strip(remaining)


def gt(iterable):
    if len(iterable) == 2:
        return iterable[0] > iterable[1]
    else:
        raise Exception


def lt(iterable):
    if len(iterable) == 2:
        return iterable[0] < iterable[1]
    else:
        raise Exception


def eq(iterable):
    if len(iterable) == 2:
        return iterable[0] == iterable[1]
    else:
        raise Exception


class Packet:
    def __init__(self, packet_version, type_id):
        self.packet_version = packet_version
        self.type_id = type_id


class ValuePacket(Packet):
    def load_value(self, binary):
        self.value, remaining, self.binary_content = parse_literal(binary)
        return remaining

    @property
    def version_sum(self):
        return self.packet_version


class OperatorPacket(Packet):
    operator_func = {0: sum, 1: prod, 2: min, 3: max, 5: gt, 6: lt, 7: eq}

    def __init__(self, packet_version, type_id, length_type):
        super().__init__(packet_version, type_id)
        self.length_type = length_type

    def load_nested(self, binary):
        if self.length_type == "0":
            remaining = self.load_by_length(binary)

        else:
            remaining = self.load_by_packet_count(binary)

        return remaining

    def load_by_length(self, binary):
        nested_packet_length = binary_to_int(binary[:15])
        end_index = 15 + nested_packet_length
        self.binary_content = binary[15:end_index]
        self.content = []
        internal = self.binary_content
        while internal:
            next_elem, internal = create_packet(internal)
            self.content.append(next_elem)

        remaining = binary[end_index:]
        return remaining

    def load_by_packet_count(self, binary):
        total_nested_packet = binary_to_int(binary[:11])
        remaining = binary[11:]
        self.content = []
        for x in range(total_nested_packet):
            next_elem, remaining = create_packet(remaining)
            self.content.append(next_elem)
        self.binary_content = binary[11:].strip(remaining)

        return remaining

    @property
    def version_sum(self):
        return sum([c.version_sum for c in self.content]) + self.packet_version

    @property
    def value(self):
        return OperatorPacket.operator_func[self.type_id](
            [c.value for c in self.content]
        )


def create_packet(binary):
    if len(binary) <= 6:
        return None, ""

    packet_version = int(binary[0:3], 2)
    type_id = int(binary[3:6], 2)
    if type_id == 4:
        packet = ValuePacket(packet_version, type_id)
        remaining = packet.load_value(binary[6:])
    else:
        length_type = binary[6]
        packet = OperatorPacket(packet_version, type_id, length_type)
        remaining = packet.load_nested(binary[7:])

    return packet, remaining


def task_1():
    binary = read_src()
    p, _ = create_packet(binary)
    print(f"task 1: {p.version_sum}")


def task_2():
    binary = read_src()
    p, _ = create_packet(binary)
    print(f"task 2: {p.value}")


if __name__ == "__main__":
    DEBUG = False
    task_1()
    task_2()
