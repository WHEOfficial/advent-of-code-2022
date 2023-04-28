import math

dirs = {
    'L': [-1, 0],
    'R': [1, 0],
    'D': [0, -1],
    'U': [0, 1],
}

sign = lambda x: int(math.copysign(1, x))

def visualize_rope(rope, dims, start=[0, 0]):
    for y in range(dims[1] - 1, start[1] - 1, -1):
        row = ''
        for x in range(start[0], dims[0]):
            found = False
            for i, k in enumerate(rope):
                if k == [x, y]:
                    row += 'H' if i == 0 else str(i)
                    found = True
                    break
            row += '.' if not found else ''

        print(row)
    print()

def simulate_rope(knots, visualize=False):
    rope = [[0, 0] for i in range(knots + 1)]
    tail_visits = [[0, 0]]
    
    for l in lines:
        step = l.split(' ')
        dir_letter, num = step[0], int(step[1])
        dir = dirs[dir_letter]

        for i in range(num):
            rope[0][0] += dir[0]
            rope[0][1] += dir[1]

            for i in range(1, knots + 1):
                x_diff, y_diff = rope[i-1][0] - rope[i][0], rope[i-1][1] - rope[i][1]

                if abs(x_diff) > 1 and y_diff == 0:
                    rope[i][0] += sign(x_diff)
                elif abs(y_diff) > 1 and x_diff == 0:
                    rope[i][1] += sign(y_diff)
                elif abs(x_diff) > 1 and abs(y_diff) > 0 or \
                    abs(y_diff) > 1 and abs(x_diff) > 0:
                    rope[i][0] += sign(x_diff)
                    rope[i][1] += sign(y_diff)
            
            if rope[knots] not in tail_visits:
                tail_visits.append(rope[knots].copy())
            
            if visualize:
                visualize_rope(rope, [10, 10], [-10, -10])
    
    return len(tail_visits)

with open("data/day09.txt", 'r') as infile:
    lines = infile.read().splitlines()
    part1 = True
    part2 = True

    if part1:
        print(simulate_rope(1))
    
    if part2:
        print(simulate_rope(9))