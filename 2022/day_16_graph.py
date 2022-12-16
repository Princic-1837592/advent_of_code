from typing import Dict

import day_16
import random


def to_graphml(graph: Dict[str, day_16.Valve]) -> str:
    enum = list(enumerate(graph.items()))
    graph_ids = {name: i for i, (name, _) in enum}
    nodes = [
        f"""<node positionX="{random.randint(0, 100)}" positionY="{random.randint(0, 100)}" id="{i}" 
        mainText="{name}: {valve.value}" upText="" size="60"></node>"""
        for i, (name, valve) in enum
    ]
    edge_id = 0
    edges = []
    for i, (name, valve) in enum:
        for neighbour in valve.tunnels:
            edges.append(f"""<edge id="{edge_id}" source="{i}" target="{graph_ids[neighbour]}"></edge>""")
            edge_id += 1
    return f"""<?xml version="1.0" encoding="UTF-8"?>
<graphml>
    <graph id="Graph" uidGraph="{len(graph)}" uidEdge="{10000 + len(edges)}">
    {" ".join(nodes + edges)}
    </graph>
</graphml>"""


def main():
    with open("day_16_input.txt", "r") as input_file:
        data = input_file.read().strip()
    graph = day_16.make_graph(data)
    with open("day_16_graph.graphml", "w") as output_file:
        output_file.write(to_graphml(graph))
    print("https://graphonline.ru/en/#")


if __name__ == "__main__":
    main()
