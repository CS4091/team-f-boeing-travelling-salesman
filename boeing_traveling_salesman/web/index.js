import init, { GraphWrapper } from '/pkg/boeing_traveling_salesman.js';

function plotData(graph, event, nodePos, nodes, edges, canvas) {
  
  const file = event.target.files[0];
  if (!file) return;

  const reader = new FileReader();
  reader.onload = function () {
    const csvData = reader.result;
    try {
      graph.load_csv_from_string(csvData);
      // Now get nodes and edges
      nodes = graph.get_nodes();
      console.log("Nodes: ");
      nodes.forEach(node => {
        console.log(`Id: ${node.id}, Label: ${node.label}`);
      });
      edges = graph.get_edges();
      console.log("Edges: ");
      edges.forEach(edge => {
        console.log(`From: ${edge.from}, To: ${edge.to}, Cost: ${edge.cost}`)
      });

      forceDirected(nodes, nodePos, edges, .5, 35, 1000);
      drawGraph(nodes, edges, nodePos, canvas);
    } catch (error) {
      console.error("Error loading CSV: ", error);
    }
  };
  reader.readAsText(file);
}

function forceDirected(nodes, nodePos, edges, threshold, ideal_len, max_iter) {
  let iter = 1;
  let forces = [];
  let cooling_factor = 1;
  for (let i = 0; i < nodes.length; i++) {
    nodePos[i] = [Math.random() * ideal_len,Math.random() * ideal_len]; //randomized initial x and y positions
    console.log(nodePos[i][0] + ', ' + nodePos[i][1]);
    forces[i] = [0,0]; //x and y forces
  }
  let max_force = threshold;
  while (iter < max_iter && max_force >= threshold) {
    for (let i = 0; i < nodes.length; i++) {
      forces[i] = [0,0]; //x and y forces
    }
    //calculate the forces on every node
    for (let u = 0; u < nodes.length; u++) {
      //calculates the repulsion force from every other node (v) onto the node in question (u)
      for (let v = 0; v < nodes.length; v++) {
        if (u == v) {
          continue;
        }
        let pos_diff = [nodePos[u][0] - nodePos[v][0], nodePos[u][1] - nodePos[v][1]] //difference in x and y from NODE V to NODE U
        let euclidean_dist = Math.sqrt(pos_diff[0]**2 + pos_diff[1]**2);
        let f_rep = 0;
        if (euclidean_dist != 0) {
          f_rep = ideal_len**2 / euclidean_dist;
        }
        forces[u][0] += f_rep * (pos_diff[0] / euclidean_dist);
        forces[u][1] += f_rep * (pos_diff[1] / euclidean_dist);
      }
      //calculates the attraction force from every other CONNECTED node (v) onto the node in question (u)
      edges.forEach(edge => {
        if (edge.to == edge.from) {
          return;
        }
        let v = 0;
        let u_label = 0;
        nodes.forEach(node => {
          if (u == node.id) {
            u_label = node.label;
            //u ID is already equal to the node in question, the label is needed to check whether a given edge affects it
          }
          if (edge.from == node.label) {
            v = node.id;
          }
        });
        if (edge.to == u_label) {
          console.log(edge)
          let pos_diff = [nodePos[v][0] - nodePos[u][0], nodePos[v][1] - nodePos[u][1]] //difference in x and y from NODE U to NODE V
          let euclidean_dist = Math.sqrt(pos_diff[0]**2 + pos_diff[1]**2);
          let f_attr = Math.min((euclidean_dist * (1 + (edge.cost / edge.cost + .25)))**2 / ideal_len, 5 * ideal_len); 
          console.log(f_attr)
          forces[u][0] += f_attr * (pos_diff[0] / euclidean_dist);
          forces[u][1] += f_attr * (pos_diff[1] / euclidean_dist);
        }
      });
    }
    //adjust the position of each node based on the forces calculated earlier
    max_force = 0;
    for (let u = 0; u < nodes.length; u++) {
      if (forces[u][0] * cooling_factor > max_force) { max_force = forces[u][0] * cooling_factor }
      if (forces[u][1] * cooling_factor > max_force) { max_force = forces[u][1] * cooling_factor }
      nodePos[u][0] += (forces[u][0] * cooling_factor);
      nodePos[u][1] += (forces[u][1] * cooling_factor);
    }
    cooling_factor /= 1.01;
    iter++;

  }
  nodes.forEach(node => {
    console.log(`Id: ${node.id}, Label: ${node.label}, X: ${nodePos[node.id][0]}, Y: ${nodePos[node.id][1]}`);
  });
  console.log("Number of iterations: " + iter.toString())
}

function drawGraph (nodes, edges, nodePos, canvas) {
  let ctx = canvas.getContext("2d");
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  let width = canvas.width;
  let height = canvas.height;
  //Adjusting all the point positions to be graphed on the canvas
  let resize_coords = [9999,-9999,9999,-9999]; //furthest left, furthest right, furthest up, furthest down points, used to adjust graph based on point positions
  nodePos.forEach(node => {
    if (node[0] < resize_coords[0]) { //If x-coord is less than the current furthest left
      resize_coords[0] = node[0];
    }
    if (node[0] > resize_coords[1]) { //If x-coord is greater than the current furthest right
      resize_coords[1] = node[0];
    }
    if (node[1] < resize_coords[2]) { //If y-coord is less than the current furthest up
      resize_coords[2] = node[1];
    }
    if (node[1] > resize_coords[3]) { //If y-coord is greater than the current furthest down
      resize_coords[3] = node[1];
    }
  });
  nodePos.forEach(node => {
    //Setting furthest left and up points to coordinate 0, shifting all other nodes accordingly
    node[0] -= resize_coords[0];
    node[1] -= resize_coords[2];
    //Setting furthest right and down points to 10/12 times the width or height of the screen, scaling all other nodes accordingly
    node[0] *= ((width/(resize_coords[1] - resize_coords[0])) * (10/12));
    node[1] *= ((height/(resize_coords[3] - resize_coords[2])) * (10/12));
    //Adding 1/12 * width or height to all nodes, giving the canvas a 1/12 width and 1/12 height buffer around the points
    node[0] += width/12;
    node[1] += height/12;
  });

  
  ctx.strokeStyle = 'grey';
  //drawing all edges
  edges.forEach(edge => {
    let to_coords = [0,0];
    let from_coords = [0,0];
    
    nodes.forEach(node => {
      if (node.label == edge.to) {
        to_coords = [nodePos[node.id][0], nodePos[node.id][1]];
      }
      if (node.label == edge.from) {
        from_coords = [nodePos[node.id][0], nodePos[node.id][1]];
      }
    });
    if (edge.to == edge.from) {
      ctx.beginPath();
      ctx.lineWidth = 2;
      let border_point = [to_coords[0] - 8, to_coords[1]];
      let angle = 11 * Math.PI / 6;
      ctx.arc(to_coords[0], to_coords[1] + 18, 20, 0, 2*Math.PI);
      ctx.moveTo(border_point[0], border_point[1]);
      ctx.lineTo(border_point[0] - 10 * Math.cos(angle - Math.PI / 6), border_point[1] - 10 * Math.sin(angle - Math.PI / 6));
      ctx.moveTo(border_point[0], border_point[1]);
      ctx.lineTo(border_point[0] - 10 * Math.cos(angle + Math.PI / 6), border_point[1] - 10 * Math.sin(angle + Math.PI / 6));
      ctx.stroke();
    }
    else {
      ctx.beginPath();
      ctx.lineWidth = 2;
      let dx = to_coords[0] - from_coords[0];
      let dy = to_coords[1] - from_coords[1];
      let angle = Math.atan2(dy, dx);
      ctx.moveTo(from_coords[0], from_coords[1]);
      let border_point = [to_coords[0] - (8 * (dx/Math.sqrt(dx**2 + dy**2))), to_coords[1] - (8 * (dy/Math.sqrt(dx**2 + dy**2)))]
      ctx.lineTo(border_point[0], border_point[1]);
      ctx.lineTo(border_point[0] - 10 * Math.cos(angle - Math.PI / 6), border_point[1] - 10 * Math.sin(angle - Math.PI / 6));
      ctx.moveTo(border_point[0], border_point[1]);
      ctx.lineTo(border_point[0] - 10 * Math.cos(angle + Math.PI / 6), border_point[1] - 10 * Math.sin(angle + Math.PI / 6));
      ctx.stroke();
    }
  });
  //drawing each node
  ctx.strokeStyle = 'black';
  nodePos.forEach(node => {
    ctx.beginPath();
    ctx.arc(node[0], node[1], (width/200), 0, 2*Math.PI);
    ctx.fill();
    ctx.stroke();
  });

}
async function main() {
  await init();

  var c = document.getElementById('canvas');
  const graph = new GraphWrapper();
  const input = document.getElementById('fileUpload');
  let nodePos = [];
  var nodes;
  var edges;

  //EVENT: File is uploaded. OUTCOME: CSV file parsed, force-directed algorithm ran, nodes and edges graphed, unlock buttons to run algorithms
  input.addEventListener('change', (event) => {
    plotData(graph, event, nodePos, nodes, edges, c);
  });
  
}


main();



