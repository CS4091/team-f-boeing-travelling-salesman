import pathlib
import typing
import random

# This script generates two styles of graphs; fully connected or sparsely connected.
# In both cases, the graph data is written as a list of edges to a CSV file and
# a corresponding PlantUML chart. The UML chart provides a simple visualization
# mechanic for the graph structure.

class WorldEdge:
    """
    Defines a graph edge with a cost in a graph.
    """
    def __init__(self, from_node: int, to_node: int, cost: float):
        self.from_node_id: int = from_node
        self.to_node_id: int = to_node
        self.cost: float = cost

def write_puml(output_filepath: pathlib.Path, edges: typing.List[WorldEdge]):
    """
    Writes a list of graph edges into a PlantUML file for ease of visualization.

    Args:
        output_filepath: The PlantUML file will be written to this location.
        edges: A list of all edges in the graph that will be written to the file.
    """
    node_ids_all: typing.List[int] = [edge.from_node_id for edge in edges]
    # Coercing a list to a set forces unique values.
    node_ids_unique: typing.Set[int] = set(node_ids_all)

    with open(output_filepath, 'wt') as output:
        # File header
        output.write('@startuml\n')

        # Declare all the nodes in the graph
        for node_id in node_ids_unique:
            output.write(f'circle {node_id}\n')

        # Declare all the edges in the graph
        for edge in edges:
            output.write(f'{edge.from_node_id} -[#black]-> {edge.to_node_id} : {edge.cost:.2f}\n')

        # File footer
        output.write('@enduml\n')

def write_data_file(output_filepath: pathlib.Path, edges: typing.List[WorldEdge]):
    """
    Writes out all graph edges to a CSV format of <From,To,Cost>.

    Args:
        output_filepath: The data file will be written to this location.
        edges: A list of all edges in the graph that will be written to the file.
    """
    with open(output_filepath, 'wt') as output:
        output.write('From,To,Cost\n')
        for edge in edges:
            output.write(f'{edge.from_node_id},{edge.to_node_id},{edge.cost}\n')

def generate_fully_connected_world(num_nodes: int) -> typing.List[WorldEdge]:
    """
    Generates a fully connected bidirectional graph with all edge costs being 1.0.

    Args:
        num_nodes: The number of nodes to generate in the graph.
    
    Returns:
        A list of WorldEdge objects describing the graph.
    """
    edges: typing.List[WorldEdge] = []
    for i in range(num_nodes):
        for j in range(num_nodes):
            if i != j:
                edges.append(WorldEdge(i, j, random.randint(1,10)))
    return edges


def generate_sparsely_connected_world(num_nodes: int, connectivity_ratio: float, min_cost: float, max_cost: float) -> typing.List[WorldEdge]:
    """
    Generates a sparsely connected unidirectional graph with random edge traversal costs.

    Args:
        num_nodes: The number of nodes to generate in the graph.
        connectivity_ratio: A value [0, 1] indicating what percent of nodes should be connected to every other node.
        min_cost: Used to generate a random edge traversal cost. This is the minimum bound of all random costs.
        max_cost: Used to generate a random edge traversal cost. This is the maximum bound of all random costs.
    
    Returns:
        A list of WorldEdge objects describing the graph.
    """    
    if connectivity_ratio > 1.0 or connectivity_ratio < 0.:
        raise Exception(f'Connectivity ratio must be bounded between [0, 1]. Given {connectivity_ratio}.')

    edges: typing.List[WorldEdge] = []
    node_ids: typing.List[int] = range(num_nodes)

    num_edges_per_node = int(num_nodes * connectivity_ratio)
    for i in range(num_nodes):
        # Guarantee at least one input and one output, otherwise the node
        # should not exist in the world.
        input_id: int = -1
        output_id: int = -1

        while True:
            input_id = random.choice(node_ids)
            # Do not loop back on self
            if input_id != i:
                break

        while True:
            output_id = random.choice(node_ids)
            # Do not loop back on self
            if output_id != i:
                break

        cost = random.uniform(a=min_cost, b=max_cost)
        edges.append(WorldEdge(i, output_id, cost))
        cost = random.uniform(a=min_cost, b=max_cost)
        edges.append(WorldEdge(input_id, i, cost))

        for destination in random.sample(population=node_ids, k=num_edges_per_node):
            cost = random.uniform(a=min_cost, b=max_cost)
            edges.append(WorldEdge(i, destination, cost))

    return edges



def create_world_files(world_name: str, edges: typing.List[WorldEdge]):
    """
    Write out the data files describing the problem for consumption by student projects.
    """
    puml_path = pathlib.Path(f'./{world_name}.puml')
    data_path = pathlib.Path(f'./{world_name}.csv')
    write_puml(output_filepath=puml_path, edges=edges)
    write_data_file(output_filepath=data_path, edges=edges)


if __name__ == '__main__':
    num_nodes = 100
    connectivity_ratio = 0.3
    min_cost = 1.0
    max_cost = 10.0

    full_edges: typing.List[WorldEdge] = generate_fully_connected_world(num_nodes)
    sparse_edges: typing.List[WorldEdge] = generate_sparsely_connected_world(num_nodes=num_nodes, connectivity_ratio=connectivity_ratio,
                                                                             min_cost=min_cost, max_cost=max_cost)
    
    create_world_files('full_world', full_edges)
    create_world_files('sparse_world', sparse_edges)


