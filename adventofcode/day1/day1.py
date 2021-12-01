import argparse

parser = argparse.ArgumentParser()
parser.add_argument('filename', type=str)
args = parser.parse_args()

previous = None

num_increasing = 0

with open(args.filename, 'r') as f:
    while True:
        line = f.readline()

        if not line:
            break       
        value = int(line)
        if previous:
            num_increasing += (1 if value > previous else 0)
        
        previous = value

print(f"Num increasing : {num_increasing}")

