import cv2
import textwrap
import numpy as np

## PART 1
with open('source.txt') as f:
    puzzleData=f.read()

print(len(puzzleData))
layers = textwrap.wrap(puzzleData, 150)
#len = 15000
#each layer is 150 (25 x 6)

counts = [(layer.count('0'), layer.count('1'), layer.count('2'), layer) for layer in layers]

#validation
for count in counts:
    totalcount = count[0] + count[1] + count[2]
    if (totalcount != len(count[3])):
        raise Exception("Items in layer not accounted for")

minZeros = min(count[0] for count in counts)
minLayer = next(count for count in counts if count[0] == minZeros)

print(minLayer[1] * minLayer[2])

## PART 2
# 0 is black 1 is white 2 is transparent

def pixelFromData(datum):
    if (datum == 2):
        return (0,0,0,0)
    else:
        return (255*datum, 255*datum, 255*datum, 255) 

def split(layerString):
    return [pixelFromData(int(char)) for char in layerString]

convertedLayers = [np.reshape(split(layer), (6,25,4)) for layer in layers]
convertedLayers.reverse()
added_image = convertedLayers[0]


for layer in convertedLayers:
    mask = np.array([alpha  <= 0 for alpha in layer[:,:,3]], dtype=np.uint8)
    bg = np.multiply(added_image, np.dstack((mask,mask,mask,mask)))
    added_image = cv2.add(layer, bg)

#testArray = np.reshape(split(minLayer[3]), (6,25,4))

cv2.imwrite('test.png', added_image)