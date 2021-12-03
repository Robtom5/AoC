import math
from collections import namedtuple
from enum import Enum
import re

class Direction(Enum):
    UP = 1
    DOWN = 2
    LEFT = 3
    RIGHT = 4

    @property
    def isHorizontal(self):
        return self is Direction.LEFT or self is Direction.RIGHT

    @property
    def isPositive(self):
        return self is Direction.UP or self is Direction.RIGHT

    @staticmethod
    def FromString(direction):
        if direction is 'U':
            return Direction.UP
        elif direction is 'D':
            return Direction.DOWN
        elif direction is 'L':
            return Direction.LEFT
        else:
            return Direction.RIGHT

class Instruction():
    Regex = re.compile(r"([a-z]+\d+)", re.I)
    SplitRegex = re.compile(r"([a-z]+)(\d+)", re.I)

    def __init__(self, encoded):
        parsed = Instruction.SplitRegex.match(encoded).groups()
        self.direction = Direction.FromString(parsed[0])
        self.distance = int(parsed[1])

    @staticmethod
    def ListFromRaw(raw):
        return [Instruction(rawI) for rawI in Instruction.Regex.findall(raw)]
        
class Point():
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def add(self, otherPoint):
        newX = self.x + otherPoint.x
        newY = self.y + otherPoint.y
        return Point(newX, newY)

    def multiply(self, factor):
        newX = self.x * factor
        newY = self.y * factor
        return Point(newX, newY)

    def distanceToPoint(self, otherPoint):
        # Manhattan Distance
        deltaX = abs(self.x - otherPoint.x)
        deltaY = abs(self.y - otherPoint.y)
        return (deltaX + deltaY)

    def __eq__(self, obj):
        return self.x is obj.x and self.y is obj.y

class WireSegment():
    def __init__(self, startPoint, endPoint):
        self.startPoint = startPoint
        self.endPoint = endPoint

    @property
    def deltaX(self):
        return self.startPoint.x - self.endPoint.x
        pass

    @property
    def maxX(self):
        return max(self.startPoint.x, self.endPoint.x)

    @property
    def maxY(self):
        return max(self.startPoint.y, self.endPoint.y)

    @property
    def minX(self):
        return min(self.startPoint.x, self.endPoint.x)

    @property
    def minY(self):
        return min(self.startPoint.y, self.endPoint.y)

    @property
    def isHorizontal(self):
        return abs(self.deltaX) > 0

    def intersects(self, otherWire):
        if (self.isHorizontal != otherWire.isHorizontal):
            verticalWire = otherWire if self.isHorizontal else self
            horizontalWire = self if self.isHorizontal else otherWire

            if (verticalWire.maxY >= horizontalWire.maxY
                and verticalWire.minY <= horizontalWire.maxY
                and horizontalWire.maxX >= verticalWire.maxX
                and horizontalWire.minX <= verticalWire.maxX):
                return (True, Point(verticalWire.maxX, horizontalWire.maxY))
            else:
                return (False, None)
            # check that the wire are in different directions
            # check that they cross ( eg start/end above and below)
            pass
            #return (true, Point(intersection))
        elif (self.startPoint == otherWire.endPoint
            or self.startPoint == otherWire.startPoint):
            return (True, self.startPoint)
        elif (self.endPoint == otherWire.startPoint 
            or self.endPoint == otherWire.endPoint):
            return (True, self.endPoint)
        else:
            return (False, None)

    @staticmethod
    def FromInstruction(startPoint, instruction):
        distanceFactor = instruction.distance if instruction.direction.isPositive else -instruction.distance
        unitVector = Point(1,0) if instruction.direction.isHorizontal else Point(0,1)
        vector = unitVector.multiply(distanceFactor)
        endPoint = startPoint.add(vector)
        return WireSegment(startPoint, endPoint)

## PART 1
with open('source.txt') as f:
    puzzleData=f.read()

origin = Point(0,0)

testData = "R8,U5,L5,D3\nU7,R6,D4,L4"

wires = puzzleData.splitlines()
wireSegments = []
for wire in wires:
    segments = []
    allInstructions = Instruction.ListFromRaw(wire)
    startPoint = origin
    latestPoint = origin
    for instruction in allInstructions:
        segment = WireSegment.FromInstruction(latestPoint, instruction)
        latestPoint = segment.endPoint
        segments.append(segment)
    wireSegments.append(segments)

wire1Segments = wireSegments[0]
wire2Segments = wireSegments[1]

intersections = []
for segment_1 in wire1Segments:
    for segment_2 in wire2Segments:
        res, pt = segment_1.intersects(segment_2)
        if(res):
            if (pt.distanceToPoint(origin) > 0):
                intersections.append((pt, pt.distanceToPoint(origin)))

print(min(intersec[1] for intersec in intersections))

## PART 2
# Find closest in terms of wire length.