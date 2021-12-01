import argparse

parser = argparse.ArgumentParser()
parser.add_argument('filename', type=str)
args = parser.parse_args()

previous = None

num_increasing = 0

with open(args.filename, 'r') as f:
    num_increasing = 0
    depths = [int(v) for v in f.readlines()]
    num_windows = len(depths) - 2

    for i in range(0, num_windows-1):
        if depths[i+3] > depths[i]:
            num_increasing += 1 


print(f"Num increasing : {num_increasing}")

