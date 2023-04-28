with open('data/day18.txt', 'r') as infile:
    lines = infile.read().splitlines()
    coords = [[int(n) for n in c.split(',')] for c in lines]
    
    sides = 0
    dirs = [
        [1, 0, 0],
        [-1, 0, 0],
        [0, 1, 0],
        [0, -1, 0],
        [0, 0, 1],
        [0, 0, -1]
    ]

    for c in coords:
        x, y, z = c
        adjacents = sum([[x + d[0], y + d[1], z + d[2]] in coords for d in dirs])
        sides += len(dirs) - adjacents

    print(sides)