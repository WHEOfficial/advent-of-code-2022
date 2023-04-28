# got 569 (part 1) and 640 (part 2) in the world with this code 😎

with open('data/day10.txt') as infile:
    lines = infile.read().splitlines()

    # part 1
    cycles = 0
    register = 1
    signal = 0

    for l in lines:
        instructions = l.split(' ')
        if instructions[0] == 'noop':
            cycles += 1
            if (cycles - 20) % 40 == 0:
                signal += cycles * register
        elif instructions[0] == 'addx':
            num = int(instructions[1])
            cycles += 1
            if (cycles - 20) % 40 == 0:
                signal += cycles * register

            cycles += 1
            if (cycles - 20) % 40 == 0:
                signal += cycles * register

            register += num
    
    print(signal)

    # part 2
    cycles = 0
    register = 1
    line = ''
    for l in lines:
        instructions = l.split(' ')
        if instructions[0] == 'noop':
            cycles += 1
            line += '#' if abs(((cycles - 1) % 40) - (register % 40)) <= 1 else '.'
            if cycles % 40 == 0:
                print(line)
                line = ''
        elif instructions[0] == 'addx':
            num = int(instructions[1])

            cycles += 1
            line += '#' if abs(((cycles - 1) % 40) - (register % 40)) <= 1 else '.'
            if cycles % 40 == 0:
                print(line)
                line = ''
            
            cycles += 1
            line += '#' if abs(((cycles - 1) % 40) - (register % 40)) <= 1 else '.'
            if cycles % 40 == 0:
                print(line)
                line = ''
            
            register += num