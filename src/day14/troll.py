from math import copysign

sign = lambda x: int(copysign(1, x))

with open('data/day14.txt') as infile:
    lines = infile.read().splitlines()

    wall_coords = []
    sand_coords = []
    sand_origin = (500, 0)
    max_wall = 0

    voided = False
    # true for p2, false for p1
    # WARNING: p2 takes a bit
    do_bottom = True

    for l in lines:
        wall_formations = l.split(' -> ')
        for i, w in enumerate(wall_formations):
            wall_formations[i] = [int(k) for k in w.split(',')]
        
        for i, w in enumerate(wall_formations):
            if w[1] > max_wall:
                max_wall = w[1]
            if i < len(wall_formations) - 1:
                x1, y1 = w[0], w[1]
                x2, y2 = wall_formations[i + 1][0], wall_formations[i + 1][1]

                if x1 == x2:
                    for y in range(y1, y2, sign(y2 - y1)):
                        wall_coords.append((x1, y))
                else:
                    for x in range(x1, x2, sign(x2 - x1)):
                        wall_coords.append((x, y1))
            else:
                wall_coords.append((w[0], w[1]))

    sand_list = [sand_origin]
    while not voided and len(sand_list) > 0:
        sand = sand_list[-1]
        while True:
            down = (sand[0], sand[1] + 1)
            left = (sand[0] - 1, sand[1] + 1)
            right = (sand[0] + 1, sand[1] + 1)
            if sand[1] > max_wall and not do_bottom:
                voided = True
                break
            elif sand[1] > max_wall and do_bottom:
                sand_coords.append(tuple(sand))
                sand_list.pop()
                break
            if down not in wall_coords and down not in sand_coords:
                sand = list(down)
                sand_list.append(list(down))
            elif left not in wall_coords and left not in sand_coords:
                sand = list(left)
                sand_list.append(list(left))
            elif right not in wall_coords and right not in sand_coords:
                sand = list(right)
                sand_list.append(list(right))
            else:
                sand_coords.append(tuple(sand))
                sand_list.pop()
                break
    
    print(len(sand_coords))
    