def get_value(wire, circuit, memo):
    if wire.isdigit():
        return int(wire)
    
    if wire not in memo:
        instr = circuit[wire]
        if len(instr) == 1:  # Direct assignment
            memo[wire] = get_value(instr[0], circuit, memo)
        else:
            op = instr[-2]
            if op == "AND":
                memo[wire] = get_value(instr[0], circuit, memo) & get_value(instr[2], circuit, memo)
            elif op == "OR":
                memo[wire] = get_value(instr[0], circuit, memo) | get_value(instr[2], circuit, memo)
            elif op == "LSHIFT":
                memo[wire] = get_value(instr[0], circuit, memo) << get_value(instr[2], circuit, memo)
            elif op == "RSHIFT":
                memo[wire] = get_value(instr[0], circuit, memo) >> get_value(instr[2], circuit, memo)
            elif op == "NOT":
                memo[wire] = ~get_value(instr[1], circuit, memo) & 0xFFFF
            else:
                raise ValueError("Unknown operation")

def solve_puzzle(input_file):
    circuit = {}
    with open(input_file, 'r') as file:
        for line in file:
            parts = line.strip().split(' -> ')
            circuit[parts[1]] = parts[0].split(' ')
    
    memo = {}
    result = get_value('a', circuit, memo)
    print(f"The signal ultimately provided to wire 'a' is: {result}")
    return result

# Replace 'input.txt' with the path to your actual input file
solve_puzzle('../../inputs/2015/7')
