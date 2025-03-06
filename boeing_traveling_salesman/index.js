import init, { GraphWrapper } from '../../pkg/boeing_travelling_salesman.js';

async function run() {
  await init();
  
  const graph = new GraphWrapper();

  const input = document.createElement('input');
  input.type = 'file';
  input.accept = '.csv';
  document.body.appendChild(input); // Append input to the document

  input.addEventListener('change', async (event) => {
    const file = event.target.files[0];
    if (!file) return;

    const reader = new FileReader();
    reader.onload = async() => {
      const csvData = reader.result;
      try {
        // Call load_csv_from_file() to load the graph
        await graph.load_csv_from_string(csvData);

        // Now get nodes and edges
        const nodes = await graph.get_nodes();
        console.log("Nodes: ");
        nodes.forEach(node => {
          console.log(`Id: ${node.id}, Label: ${node.label}`);
        });

        const edges = await graph.get_edges();
        console.log("Edges: ");
        edges.forEach(edge => {
          console.log(`From: ${edge.from}, To: ${edge.to}, Cost: ${edge.cost}`)
        });
      } catch (error) {
        console.error("Error loading CSV: ", error);
      }

    };
    reader.readAsText(file);
  });

}

run();

