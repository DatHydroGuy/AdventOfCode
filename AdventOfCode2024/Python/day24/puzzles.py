import graphviz


def part1(wires):
    significant_wires = {wire:wires[wire] for wire in wires if wire.startswith('z')}
    shift = -1
    result = 0
    while significant_wires:
        shift += 1
        number_part = str(shift).zfill(2)
        key = 'z' + number_part
        if key in significant_wires:
            result += significant_wires[key] << shift
            significant_wires.pop(key)

    return result


def part2(wires, gates):
    dot = graphviz.Digraph()
    for gate in gates:
        dot.node(f'{gate.input1}-{gate.input2}-{gate.output}', f'{gate.output}\n{gate.input1} {gate.operation} {gate.input2}')
    for wire in wires:
        head_nodes = [g for g in gates if g.input1 == wire or g.input2 == wire]
        tail_nodes = [g for g in gates if g.output == wire]
        if len(tail_nodes) == 0:
            dot.node(wire, wire)
            for head_node in head_nodes:
                dot.edge(wire, f'{head_node.input1}-{head_node.input2}-{head_node.output}', '1' if wires[wire] else '0')
        else:
            for head_node in head_nodes:
                for tail_node in tail_nodes:
                    dot.edge(f'{tail_node.input1}-{tail_node.input2}-{tail_node.output}', f'{head_node.input1}-{head_node.input2}-{head_node.output}', '1' if wires[wire] else '0')

    dot.render('graph', view=False)
    return None

class Wire:
    def __init__(self, name: str, value: bool):
        self.name = name
        self.value = value

class Gate:
    def __init__(self, input1: Wire, input2: Wire, operation, output: str):
        self.input1 = input1
        self.input2 = input2
        self.operation = operation
        self.output = output

def parse_data(raw_data):
    wires = []
    gates = []
    is_wires = True

    for line in raw_data:
        if line == '\n':
            is_wires = False
            continue

        if is_wires:
            wire, is_live = line.strip().split(': ')
            is_live = True if is_live == '1' else False
            wires.append(Wire(wire, is_live))
        else:
            inputs, wire = line.strip().split(' -> ')
            input1, op, input2 = inputs.split(' ')
            gates.append(Gate(input1, input2, op, wire))

    old_live_gates = 0
    wire = None
    while True:
        wire_names = [wire.name for wire in wires]
        live_gates = [g for g in gates if g.input1 in wire_names and g.input2 in wire_names]
        for g in live_gates:
            wire1 = [w for w in wires if w.name == g.input1][0]
            wire2 = [w for w in wires if w.name == g.input2][0]
            if g.operation == 'AND':
                wire = wire1.value & wire2.value
            elif g.operation == 'OR':
                wire = wire1.value | wire2.value
            elif g.operation == 'XOR':
                wire = wire1.value ^ wire2.value
            wires.append(Wire(g.output, wire))

        num_live_gates = len(live_gates)
        if num_live_gates == old_live_gates:
            break

        old_live_gates = num_live_gates

    # turns out, there are no dead wires (wires with no input values)
    # dead_wires = [g for g in gates if g.input1 not in wire_names and g.input2 not in wire_names]
    # print(dead_wires)

    wire_dict = dict()
    for w in wires:
        wire_dict[w.name] = w.value

    return wire_dict, gates


def run_puzzles(data):
    wires, gates = parse_data(data)
    answer1 = part1(wires)
    print(f"DAY 24, PART 1 RESULT: \033[92m{answer1}\033[0m")
    # parsed_data2 = parse_data2(data)
    part2(wires, gates)
    answer2 = "gbf,hdt,jgt,mht,nbf,z05,z09,z30"
    print(f"DAY 24, PART 2 RESULT: \033[92m{answer2}\033[0m")
