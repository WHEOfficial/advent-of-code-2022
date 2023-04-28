import json
from functools import cmp_to_key

def compare(left, right):
    i = 0
    while i < len(left) and i < len(right):
        l0, r0 = left[i], right[i]

        if type(l0) == int and type(r0) == int:
            if l0 != r0:
                return l0 < r0
        else:
            l0 = l0 if type(l0) == list else [l0]
            r0 = r0 if type(r0) == list else [r0]
            result = compare(l0, r0)
            if result is not None:
                return result
        
        i += 1

    return None if len(left) == len(right) else len(left) < len(right)

def sort_cmp(x, y):
    compared = (compare(x, y))
    return -1 if compared == True else 1 if compared == False else 0

with open('data/day13.txt') as infile:
    lines = infile.read().splitlines()
    items = []

    left, right = None, None
    times = 1
    sum = 0

    for l in lines:
        if l != '':
            data = json.loads(l)
            items.append(data)
            if left is None:
                left = data
            else:
                right = data

    #part 1
        if left is not None and right is not None:
            result = compare(left, right)
            sum += times if result else 0
            times += 1
            left, right = None, None
    
    print(sum)

    # part 2
    items.append([[2]])
    items.append([[6]])

    sorted_list = sorted(items, key=cmp_to_key(sort_cmp))
    print((sorted_list.index([[2]]) + 1) * (sorted_list.index([[6]]) + 1))