import math

dirs = {
    'L': [-1, 0],
    'R': [1, 0],
    'D': [0, -1],
    'U': [0, 1],
}

sign = lambda x: int(math.copysign(1, x))

def visualize_rope(head, tail, dims):
    for y in range(dims[1] - 1, -1, -1):
        row = ''
        for x in range(dims[0]):
            if (x, y) == head:
                row += 'H'
            elif (x, y) == tail:
                row += 'T'
            else:
                row += '.'
        print(row)
    print()

with open("data/day09.txt", 'r') as infile:
    lines = infile.read().splitlines()
    part1 = False
    part2 = True

    if part1:
        head_pos = [0, 0]
        tail_pos = [0, 0]
        tail_visits = [[0, 0]]

        for l in lines:
            step = l.split(' ')
            dir_letter, num = step[0], int(step[1])
            dir = dirs[dir_letter]

            for i in range(num):
                head_pos[0] += dir[0]
                head_pos[1] += dir[1]

                x_diff, y_diff = head_pos[0] - tail_pos[0], head_pos[1] - tail_pos[1]

                if abs(x_diff) > 1 and y_diff == 0:
                    tail_pos[0] += sign(x_diff)
                elif abs(y_diff) > 1 and x_diff == 0:
                    tail_pos[1] += sign(y_diff)
                elif abs(x_diff) > 1 and abs(y_diff) > 0 or \
                    abs(y_diff) > 1 and abs(x_diff) > 0:
                    tail_pos[0] += sign(x_diff)
                    tail_pos[1] += sign(y_diff)
                
                if tail_pos not in tail_visits:
                    tail_visits.append(tail_pos.copy())

        print(len(tail_visits))
    
    if part2:
        knots = 9
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
    
    print(len(tail_visits))
