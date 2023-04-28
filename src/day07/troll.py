class File:
    def __init__(self, name, size):
        self.name = name
        self.size = size
    
    def __str__(self) -> str:
        return f"{self.name}: {self.size}"

class Directory:
    def __init__(self, name, parent=None):
        self.name = name
        self.parent = parent
        self.dirs = []
        self.files = []
    
    def __str__(self) -> str:
        return f"{self.name}: dirs: {[d.__str__() for d in self.dirs]}, files: {[f.__str__() for f in self.files]}"
    
    def add_dir(self, dir):
        self.dirs.append(dir)

    def add_file(self, file):
        self.files.append(file)
    
    def get_directory(self, dir_name, recursive=False):
        for d in self.dirs:
            if d.name == dir_name:
                return d

    def get_directories(self, all_dirs=None):
        if all_dirs is None:
            all_dirs = []
        for d in self.dirs:
            all_dirs = d.get_directories(all_dirs)
        all_dirs.append(self)
        return all_dirs
    
    def get_size(self):
        size = 0
        for f in self.files:
            size += f.size
        for d in self.dirs:
            size += d.get_size()
        return size


with open('data/day07.txt') as infile:
    data = infile.read().splitlines()
    root = Directory('/')
    pwd = root

    for line in data:
        args = line.split(' ')

        if args[0] == '$':
            if args[1] == 'cd':
                if args[2] == '/':
                    pwd = root
                elif args[2] == '..':
                    pwd = pwd.parent
                else:
                    pwd = pwd.get_directory(args[2])
        elif args[0] == 'dir':
            pwd.add_dir(Directory(args[1], pwd))
        else:
            pwd.add_file(File(args[1], int(args[0])))

    # part 1
    sum = 0
    for d in root.get_directories():
        if (size := d.get_size()) <= 100000:
            sum += size
    
    print(sum)

    # part 2
    root_size = root.get_size()
    size_to_remove = 30000000 - (70000000 - root_size)
    min_size = root_size

    for d in root.get_directories():
        if (size := d.get_size()) >= size_to_remove and size < min_size:
            min_size = size

    print(min_size)