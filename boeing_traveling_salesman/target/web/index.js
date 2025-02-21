import init, { Graph } from '../../pkg/boeing_travelling_salesman.js';

async function run() {
    await init();
    
    let graph = new Graph();
    let node1 = graph.add_node("A");
    let node2 = graph.add_node("B");
    graph.add_edge(node1, node2);

    console.log("Number of nodes:", graph.node_count());
}

run();

