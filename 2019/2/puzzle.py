import math

## PART 1
# OPCODES:
# 1 - Add together numbers read from two positions and store in third position
#   First two integetrs refer to the two positions to read values form, third indicates the position to store
# 2 - Same as above but multiply
# 99 - End
OPCODES = {
    1: lambda a,b: a+b, 
    2: lambda a,b: a*b, 
    99: None}

# After each opcode move forward four positions
# with open('source.txt') as f:
    # puzzleData=f.read()

class IntComputer():
    def __init__(self):
        pass

    def loadCommands(self, filename):
        self._filename = filename
        self._originalCommands = None
        self.reloadCommands()

    def reloadCommands(self):
        if self._originalCommands is None:
            with open(self._filename) as f:
                puzzleData = f.read()
            self._originalcommands = [int(command) for command in puzzleData.split(',')]
        self.commands = self._originalcommands

    def finalResult(self):
        return self.commands[0]

    def setMemory(self, address, value):
        self.commands[address] = value

    def execute(self):
        position = 0
        lastOpCode = 0
        while (position < len(self.commands)) and lastOpCode != 99:
            opcode = self.commands[position]    
            if opcode in OPCODES.keys():
                if opcode != 99:
                    src1 = self.commands[position + 1]
                    src2 = self.commands[position + 2]
                    trgt = self.commands[position + 3]

                    data1 = self.commands[src1]
                    data2 = self.commands[src2]

                    res = OPCODES[opcode](data1, data2)
                    if res is None:
                        raise Exception("Bad OPCODE Execution")
                    self.commands[trgt] = res
                    lastOpCode = opcode
                    position = position + 4
                else:
                    lastOpCode = 99
            else:
                raise Exception("Bad OPCODE")

comp = IntComputer()
comp.loadCommands('source.txt')
comp.setMemory(1, 12)
comp.setMemory(2,2)
comp.execute()
print(comp.finalResult())

## PART 2
#Find a value for mem1 and mem2 (noun and verb) such that the result is 19690720
def FindAnswer(computer, answer):
    for noun in range(0,100):
        for verb in range(0,100):
            computer.reloadCommands()
            computer.setMemory(1,noun)
            computer.setMemory(2,verb)
            computer.execute()
            if(computer.finalResult() == answer):
                return(noun, verb)
noun, verb = FindAnswer(comp, 19690720)
print(f"Noun: {noun}\nVerb: {verb}")
print(100*noun + verb)
