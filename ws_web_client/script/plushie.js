import * as create from "./lib/create";
import * as simulation from "./lib/simulation";
import "./jquery";

const peculiarityColors = {
  "normal": 0x00ff00,
  "centroid": 0xffa500,
  "Tip": 0x0000ff,
  "Root": 0x0000ff,
}
const debugDisplay = false;

export default class Plushie {
  constructor(status, gui, pattern) {
    this.edges = [];
    this.stitchSpheres = [];
    this.stitchPositions = [];
    this.centroidSpheres = [];
    this.links = [];
    this.dragged = null;
    this.displayEdges = true;
    this.status = status;
    this.gui = gui;
    this.pattern = pattern;
    this.peculiar = {};
    this.nodeColors = [];
    this.colors = peculiarityColors;
    // those will be immediately overwritten with serialized params from the server
    // but they have to be initialized here so gui can be constructed
    this.params = {
      "centroids": {
        "force": 0.1,
        "min_nodes_per_centroid": 0,
        "number": 0,
      },
      "desired_stitch_distance": 0.1, // decimal point here enables displaying decimal points later
      "floor": false,
      "gravity": 0.1,
      "keep_root_at_origin": false,
      "single_loop_force": 0.1,
      "timestep": 1.1,
    };
  }

  getId(obj) {
    return this.stitchSpheres.findIndex((o) => o == obj);
  }

  drag(obj) {
    let id = this.getId(obj);
    this.dragged = obj;
    simulation.send(`pos ${id} ${obj.position.x} ${obj.position.y} ${obj.position.z}`);
  }

  mouseMove(obj) {
    this.drag(obj);
  }

  mouseUp(obj) {
    this.drag(obj);
  }

  toggleLinks() {
    this.displayEdges = !this.displayEdges;
    this.clearLinks();
    if (this.displayEdges)
      this.drawLinks();
  }

  onAdvance() {
    this.clearLinks();
  }

  clearLinks() {
    for (let link of this.links) {
      if (link.geometry) link.geometry.dispose();
      if (link.material) link.material.dispose();
      create.scene.remove(link);
    }
    this.links = [];
  }

  linkColor(from, to) {
    if (debugDisplay) {
      return "red";
    } else {
      let [r, g, b] = this.nodeColors[to];
      return `rgb(${r}, ${g}, ${b})`;
    }
  }

  drawLinks() {
    for (let from = 0; from < this.edges.length; from++) {
      for (let to of this.edges[from]) {
        if (from >= this.stitchPositions.length || to >= this.stitchPositions.length) {
          continue;
        }
        let width = 0.05;
        let color = this.linkColor(from, to);
        let link = create.link(this.stitchPositions[from], this.stitchPositions[to], width, color)
        this.links.push(link);
      }
    }
  }

  updateLinks() {
    if (!this.displayEdges)
      return;

    this.clearLinks();
    this.drawLinks();
  }

  parseMessage(key, data) {
    switch (key) {
      case "upd":
        this.update(data);
        break;
      case "ini":
        this.init(data);
        break;
      case "status":
        console.log("response", data);
        this.status.innerText = data;
        break;
      case "pattern_update":
        console.log("new pattern");
        this.pattern.value = data;
        break;
      case "params":
        const RECURSIVE = true;
        jQuery.extend(RECURSIVE, this.params, JSON.parse(data))
        console.log("got params: ", this.params);
        break;
      default:
        console.error(`Unrecognized key: ${key}`);
    }
  }

  nodeColor(i) {
    if (debugDisplay) {
      let colorKey = this.peculiar[i] ?? "normal";
      return peculiarityColors[colorKey];
    } else {
      let [r, g, b] = this.nodeColors[i];
      return `rgb(${r}, ${g}, ${b})`;
    }
  }

  init(data) {
    console.log("init:", data);
    const nodes = data["nodes"];
    const points = nodes["points"];
    this.peculiar = nodes["peculiarities"]
    this.nodeColors = nodes["colors"];
    this.edges = data["edges"];
    this.gui.updateDisplay();

    this.stitchPositions = [];
    for (let sph of this.stitchSpheres) {
      if (sph.geometry) sph.geometry.dispose();
      if (sph.material) sph.material.dispose();
      create.scene.remove(sph);
    }
    this.clearLinks();

    this.stitchSpheres = [];
    for (let [i, point] of points.entries()) {
      let sph = create.sphere(point, 0.1, this.nodeColor(i));
      this.stitchPositions.push(sph.position);
      this.stitchSpheres.push(sph);
    }
    this.drawLinks();

    for (let sph of this.centroidSpheres) {
      if (sph.geometry) sph.geometry.dispose();
      if (sph.material) sph.material.dispose();
      create.scene.remove(sph);
    }
    this.centroidSpheres = [];
    const centroids = data["centroids"]["centroids"];
    for (let point of centroids) {
      let sph = create.sphere(point, 0.1, peculiarityColors.centroid);
      this.centroidSpheres.push(sph);
    }
  }

  updateWallPoints(positions) {
    if (positions.length < this.stitchSpheres.length) {
      console.error("Position data got corrupted");
    }

    if (positions.length > this.stitchSpheres.length) {
      for (let i = this.stitchSpheres.length; i < positions.length; i++) {
        let point = positions[i];
        let sph = create.sphere(point, 0.1, this.nodeColor(i));
        this.stitchPositions.push(sph.position);
        this.stitchSpheres.push(sph);
      }
    }

    let id = undefined;
    let save = undefined;
    if (this.dragged) {
      id = this.getId(this.dragged);
      save = {};
      save.x = this.dragged.position.x;
      save.y = this.dragged.position.y;
      save.z = this.dragged.position.z;
    }

    for (let i = 0; i < positions.length; i++) {
      this.stitchPositions[i].x = positions[i][0]
      this.stitchPositions[i].y = positions[i][1]
      this.stitchPositions[i].z = positions[i][2]
    }

    if (this.dragged) {
      this.dragged.position.x = save.x;
      this.dragged.position.y = save.y;
      this.dragged.position.z = save.z;
      this.dragged = null;
    }
  }

  setCentroidNum(num) {
    while (this.centroidSpheres.length < num) {
      let sph = create.sphere([0, 0, 0], 0.1, 0xffa500);
      this.centroidSpheres.push(sph);
    }

    while (this.centroidSpheres.length > num) {
      let sph = this.centroidSpheres.pop();
      create.scene.remove(sph);
    }
  }

  updateCentroids(centroids) {
    if (centroids.length != this.centroidSpheres.length) {
      this.setCentroidNum(centroids.length);
    }

    for (let i in centroids) {
      let centroid = centroids[i];
      let sph = this.centroidSpheres[i];
      sph.position.x = centroid[0]
      sph.position.y = centroid[1]
      sph.position.z = centroid[2]
    }
  }

  update(data) {
    this.updateWallPoints(data["points"]);
    this.updateCentroids(data["centroids"]["centroids"]);

    // this.updateLinks(); // disabled until an efficient solutions is found
  }
}
