with open("./Day08Input.txt") as infile:
    contents = infile.readlines()

NOP = 0
ACC = 1
JMP = 2
END = 3
INSTRUCTS = {'nop': NOP, 'acc': ACC, 'jmp': JMP, 'end': END}

debug = []

def INSTRUCT(instruction):
    for key, value in INSTRUCTS.items():
        if value == instruction:
            return key
    return 'Error'

class Line():
    def __init__(self, lineNumber, instruction, value):
        self.line = lineNumber
        self.instruction = INSTRUCTS[instruction]
        self.value = int(value)
        self.callCount = 0

    def __str__(self):
        return f"{self.line}: {INSTRUCT(self.instruction)} {self.value} \t {self.callCount}"


def InitializeDebugger(rawinput):
    lineIndex = 0
    for line in rawinput:
        instruction, value = line.split(' ')
        debug.append(Line(lineIndex, instruction, value))
        lineIndex += 1
    debug.append(Line(lineIndex, 'end', 0))


def RunCommands():
    current_command = debug[0]
    accumulator = 0
    while (current_command.callCount == 0 and current_command.instruction != END):
        current_command.callCount += 1
        if current_command.instruction == NOP:
            pass
        elif current_command.instruction == ACC:
            accumulator += current_command.value
        elif current_command.instruction == JMP:
            current_command = debug[current_command.line + current_command.value]
            continue

        current_command = debug[current_command.line + 1]
    return current_command, accumulator

if __name__ == "__main__":
    InitializeDebugger(contents)
    _ , accumulator = RunCommands()
    print(accumulator)
    # Part 2

    # Time to brute for this bitch
    number_of_lines = len(contents)
    brute_line = -1
    last_line = Line(-1, 'nop', 0)
    while (brute_line < number_of_lines and last_line.instruction != END):
        brute_line += 1
        debug = []
        InitializeDebugger(contents)
        if (debug[brute_line].instruction == JMP):
            debug[brute_line].instruction = NOP
        elif (debug[brute_line].instruction == NOP):
            debug[brute_line].instruction = JMP

        last_line , accumulator= RunCommands()

    print(f"Accumulator: {accumulator}, Brute Line {brute_line}")

