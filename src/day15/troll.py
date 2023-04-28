import re

with open('data/day15.txt') as infile:
    lines = infile.read().splitlines()
    coords = []
    beacons = []
    ranges = []
    target_y = 2000000

    for l in lines:
        search = re.search('Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)', l)
        if search:
            coord = [int(search.group(i)) for i in range(1, 5)]
            if coord[3] == target_y and (coord[2], coord[3]) not in beacons:
                beacons.append((coord[2], coord[3]))
            coords.append(coord)
    
    for c in coords:
        sensor, beacon = (c[0], c[1]), (c[2], c[3])
        diff = abs(beacon[0] - sensor[0]) + abs(beacon[1] - sensor[1])
        distance_to_target = diff - abs(target_y - sensor[1])
        if distance_to_target >= 0:
            ranges.append((sensor[0] - distance_to_target, sensor[0] + distance_to_target))
        #print(sensor, beacon, diff, distance_to_target)
    
    ranges = sorted(ranges, key=lambda r: r[0])
    new_ranges = [ranges[0]]
    for i in range(1, len(ranges)):
        new_range = ()
        r1, r2 = new_ranges[-1], ranges[i]
        if r1[0] <= r2[0] <= r1[1] or r1[0] <= r2[1] <= r1[1] or \
            r2[0] <= r1[0] <= r2[1] or r2[0] <= r1[1] <= r2[1]:
            new_range = (min(r1[0], r2[0]), max(r1[1], r2[1]))
            new_ranges.pop()
        else:
            new_range = r2
        new_ranges.append(new_range)
    
    no_beacons = 0
    for r in new_ranges:
        no_beacons += r[1] - r[0] + 1
        for b in beacons:
            if b[0] in range(r[0], r[1] + 1):
                no_beacons -= 1

    print(no_beacons)