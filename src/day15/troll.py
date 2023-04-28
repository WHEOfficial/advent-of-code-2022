import re

with open('data/day15.txt') as infile:
    lines = infile.read().splitlines()
    coords = []

    for l in lines:
        search = re.search('Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)', l)
        if search:
            coords.append([int(search.group(i)) for i in range(1, 5)])
    
    print(coords)