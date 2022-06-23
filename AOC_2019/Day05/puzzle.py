import math
import inspect
from enum import Enum
import numpy as np
import queue

## PART 1

#PARAM MODES
# Read right to left for arguments, leading 0s are omitted. 
class ParamMode(Enum):
    POSITION = 0
    IMMEDIATE = 1
    TARGET = 2

class IntComputer():
    # OPCODES:
    # 1 - Add together numbers read from two positions and store in third position
    #   First two integetrs refer to the two positions to read values form, third indicates the position to store
    # 2 - Same as above but multiply
    # 99 - End

    def __init__(self):
        self.OPCODES = {
            1: self._OP_ADD, 
            2: self._OP_MUL, 
            3: self._OP_IN,
            4: self._OP_OUT,
            5: self._OP_JMPIT,
            6: self._OP_JMPIF,
            7: self._OP_LT,
            8: self._OP_EQ,
            99: None}

    def addToInput(self, inputIter):
        self.input = inputIter

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
        self.position = 0
        lastOpCode = 0
        while (self.position < len(self.commands)) and lastOpCode != 99:
            opcode_full = self.commands[self.position]
            opcode, argSpec = self.parseOpCode(opcode_full)
            if opcode in self.OPCODES.keys():
                self.position += 1
                if opcode != 99:
                    program = self.OPCODES[opcode]
                    argCount = len(argSpec)
                    params =  [0] * argCount
                    for i in range(argCount):
                        params[i] = self.readParam(self.position+i, argSpec[i])

                    print(f"{opcode}\t{params}\t{self.position}")
                    program(*params)
                    lastOpCode = opcode
                else:
                    lastOpCode = 99
            else:
                raise Exception("Bad OPCODE")

    def parseOpCode(self, opcode_raw):
        opcode = opcode_raw % 100
        argSpec = None

        if opcode != 99:
            program = self.OPCODES[opcode]
            #Functions are internal to computer so also string self
            argCount = len(inspect.getfullargspec(program).args) -1
            #Strip opCode
            argSpec =  [0] * argCount
            opcode_params = int(opcode_raw / 100)
            for i in range(argCount):
                argSpec[i] = ParamMode(int(opcode_params % 10))
                opcode_params /= 10

            if (inspect.getfullargspec(program).args[-1] == 'trgt'):
                argSpec[-1] = ParamMode.TARGET

        return(opcode, argSpec)

    def readParam(self, address, paramMode=ParamMode.POSITION):
        value = self.commands[address]
        if (paramMode == ParamMode.POSITION):
            value = self.commands[value]
        return int(value)

    def _OP_ADD(self, src1, src2, trgt):
        self.commands[trgt] = src1 + src2
        self.position += 3

    def _OP_MUL(self, src1, src2, trgt):
        self.commands[trgt] = src1 * src2
        self.position += 3

    def _OP_IN(self, trgt):
        userInput = next(self.input)
        self.commands[trgt] = userInput
        self.position += 1

    def _OP_OUT(self, trgt):
        print(f"Output: {self.commands[trgt]}")
        self.position += 1

    def _OP_JMPIT(self, src1, src2):
        if (src1 != 0):
            self.position = src2
        else:
            self.position += 2

    def _OP_JMPIF(self, src1, src2):
        if (src1 == 0):
            self.position = src2
        else:
            self.position += 2

    def _OP_LT(self, src1, src2, trgt):
        if (src1 < src2):
            self.commands[trgt] = 1
        else:
            self.commands[trgt] = 0
        self.position += 3

    def _OP_EQ(self, src1, src2, trgt):
        if (src1 == src2):
            self.commands[trgt] = 1
        else:
            self.commands[trgt] = 0
        self.position += 3


comp = IntComputer()
comp.addToInput(iter([1]))
comp.loadCommands('source.txt')
comp.execute()

