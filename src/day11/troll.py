from functools import reduce

ops = {
    '+': lambda x, y: x + y,
    '*': lambda x, y: x * y,
    '^': lambda x, y: x * x
}

worried = True

do_part1 = False

class Monkey:
    def __init__(self, items, op, num, cond):
        self.items = items
        self.op = op
        self.num = num
        self.cond = cond
        self.receivers = []
        self.inspected = 0
    
    def add_item(self, item):
        self.items.append(item)
    
    def set_receivers(self, receivers):
        self.receivers = receivers
    
    def simulate_items(self):
        #print(f'Items: {self.items}')
        while len(self.items) > 0:
            item = self.items.pop()
            item = ops[self.op](item, self.num)
            item = item // 3 if worried else item
            if item % self.cond == 0:
                #item = primes(item) if not worried else item
                self.receivers[0].add_item(item)
            else:
                self.receivers[1].add_item(item)
            self.inspected += 1

# from https://stackoverflow.com/questions/16996217/prime-factorization-list
# slightly modified
def primes(n):
    primfac = []
    d = 2
    while d*d <= n:
        while (n % d) == 0:
            primfac.append(d)  # supposing you want multiple factors repeated
            n //= d
        d += 1
    if n > 1:
       primfac.append(n)
    return reduce(lambda x, y: x*y, [*set(primfac)])

def solution(rounds, monkeys):
    for i in range(rounds):
        for m in monkeys:
            m.simulate_items()
        print(f"Round {i+1} done.")

    inspect_amounts = []
    for m in monkeys:
        inspect_amounts.append(m.inspected)

    inspect_amounts.sort(reverse=True)
    print(inspect_amounts[0] * inspect_amounts[1])
        
# hardcoding this time

monkeys = []

monkey0 = Monkey([56, 52, 58, 96, 70, 75, 72], '*', 17, 11)
monkey1 = Monkey([75, 58, 86, 80, 55, 81], '+', 7, 3)
monkey2 = Monkey([73, 68, 73, 90], '^', 0, 5)
monkey3 = Monkey([72, 89, 55, 51, 59], '+', 1, 7)
monkey4 = Monkey([76, 76, 91], '*', 3, 19)
monkey5 = Monkey([88], '+', 4, 2)
monkey6 = Monkey([64, 63, 56, 50, 77, 55, 55, 86], '+', 8, 13)
monkey7 = Monkey([79, 58], '+', 6, 17)

monkey0.set_receivers([monkey2, monkey3])
monkey1.set_receivers([monkey6, monkey5])
monkey2.set_receivers([monkey1, monkey7])
monkey3.set_receivers([monkey2, monkey7])
monkey4.set_receivers([monkey0, monkey3])
monkey5.set_receivers([monkey6, monkey4])
monkey6.set_receivers([monkey4, monkey0])
monkey7.set_receivers([monkey1, monkey5])

monkeys = [monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7]

if do_part1:
    worried = True
    solution(20, monkeys)
else:
    worried = False
    solution(20, monkeys)

# too tired to figure out how to do part 2 rn