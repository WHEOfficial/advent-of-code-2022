from collections import deque

# modified version of:
# https://stackoverflow.com/questions/47896461/get-shortest-path-to-a-cell-in-a-2d-array-in-python
def bfs(grid, start, end):
    queue = deque([[start]])
    visited = set((start))
    while queue:
        path = queue.popleft()
        x, y = path[-1]
        if (x, y) == end:
            return path
            break
        for x2, y2 in ((x+1,y), (x-1,y), (x,y+1), (x,y-1)):
            if 0 <= x2 < WIDTH and 0 <= y2 < HEIGHT and \
                grid[y][x] + 1 >= grid[y2][x2] and (x2, y2) not in visited:
                queue.append(path + [(x2, y2)])
                visited.add((x2, y2))

with open('data/day12.txt', 'r') as infile:
    lines = infile.read().splitlines()

    start, end = (), ()
    starts = []
    grid = []
    for y, l in enumerate(lines):
        grid.append([])
        for x, c in enumerate(l):
            o = 0 if c == 'S' else 25 if c == 'E' else ord(c) - 97
            if c == 'S':
                start = (x, y)
            elif c == 'a':
                starts.append((x, y))
            elif c == 'E':
                end = (x, y)
            grid[y].append(o)
    
    WIDTH, HEIGHT = len(grid[0]), len(grid)

    # part 1
    print(len(bfs(grid, start, end)) - 1)

    # part 2 
    # first solution that takes longer than five seconds
    min_distance = float('inf')
    for s in starts:
        if (returned := bfs(grid, s, end)) is not None:
            distance = len(returned) - 1
            if distance < min_distance:
                min_distance = distance
    
    print(min_distance)