from collections import defaultdict


def part1(computers):
    three_rings = set()
    for computer in computers:
        neighbours = computers[computer]
        # find two computers in neighbours list which contain each other
        for neighbour1 in neighbours:
            for neighbour2 in neighbours:
                if neighbour1 == neighbour2:
                    continue
                if neighbour2 in computers[neighbour1]:
                    if computer.startswith('t') or neighbour1.startswith('t') or neighbour2.startswith('t'):
                        ordered_tuple = tuple(sorted((computer, neighbour1, neighbour2)))
                        three_rings.add(ordered_tuple)

    return len(three_rings)


def part2(computers):
    nodes = list(computers.keys())
    cliques = bron_kerbosch(computers, [], nodes, [])
    max_clique = max(cliques, key=len)
    sorted_max_clique = sorted(max_clique)
    answer = ','.join(sorted_max_clique)
    return answer


def bron_kerbosch(graph, all_vertices, some_vertices, no_vertices, results=None):
    """
    The basic form of the Bron–Kerbosch algorithm is a recursive backtracking algorithm that searches for all maximal
    cliques in a given graph G. More generally, given three disjoint sets of vertices R, P, and X, it finds the maximal
    cliques that include all of the vertices in R, some of the vertices in P, and none of the vertices in X.
    """
    if results is None:
        results = []

    if len(some_vertices)  == 0 and len(no_vertices) == 0:
        results.append(all_vertices)
        return results

    for vertex in some_vertices[:]:
        vertex_neighbours = graph[vertex]
        all_new = all_vertices.copy() + [vertex]
        some_new = [v for v in some_vertices if v in vertex_neighbours]
        no_new = [v for v in no_vertices if v in vertex_neighbours]
        bron_kerbosch(graph, all_new, some_new, no_new, results)
        some_vertices.remove(vertex)
        no_vertices.append(vertex)

    return results


def parse_data(raw_data):
    computers = defaultdict(list)

    connections = [r.strip() for r in raw_data]
    for connection in connections:
        comp1, comp2 = connection.split('-')
        computers[comp1].append(comp2)
        computers[comp2].append(comp1)

    return computers


def run_puzzles(data):
    import time

    parsed_data = parse_data(data)
    start_time = time.time()
    answer1 = part1(parsed_data)
    print(f"DAY 23, PART 1 RESULT: \033[91m{answer1} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")

    start_time = time.time()
    answer2 = part2(parsed_data)
    print(f"DAY 23, PART 2 RESULT: \033[91m{answer2} \033[92m({time.time() - start_time:.5f} seconds)\033[0m")
