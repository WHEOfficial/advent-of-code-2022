def get_covered(start_coords, use_x, step, data):
    x, y = start_coords
    tree = data[y][x]
    if use_x:
        x += step
    else:
        y += step

    while x >= 0 and x < WIDTH and y >= 0 and y < WIDTH:
        if tree <= data[y][x]:
            return False
        if use_x:
            x += step
        else:
            y += step
    
    return True

def get_views(start_coords, use_x, step, data):
    x, y = start_coords
    tree = data[y][x]
    if use_x:
        x += step
    else:
        y += step

    s = 0
    while x >= 0 and x < WIDTH and y >= 0 and y < WIDTH:
        s += 1
        if tree <= data[y][x]:
            return s
        if use_x:
            x += step
        else:
            y += step
    
    return s
    

with open("data/day08.txt", 'r') as infile:
    lines = infile.read().splitlines()

    data = [[int(c) for c in x] for x in lines]
    WIDTH = len(data)

    # part 1
    s = (WIDTH * WIDTH) - (WIDTH - 2) * (WIDTH - 2)
    for y in range(1, WIDTH - 1):
        for x in range(1, WIDTH - 1):
            tree = data[y][x]
            if get_covered((x, y), True, -1, data) or \
                get_covered((x, y), True, 1, data) or \
                get_covered((x, y), False, -1, data) or \
                get_covered((x, y), False, 1, data):
                s += 1
    print(s)

    # part 2
    s = 0
    for y in range(1, WIDTH - 1):
        for x in range(1, WIDTH - 1):
            sum_to_compare = get_views((x, y), True, -1, data) * \
                get_views((x, y), True, 1, data) * \
                get_views((x, y), False, -1, data) * \
                get_views((x, y), False, 1, data)
            
            if sum_to_compare > s:
                s = sum_to_compare
    print(s)