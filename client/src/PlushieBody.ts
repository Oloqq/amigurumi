import { Mesh } from "three";
import { Display } from "./render3d";
import * as create from "./utils/create";
import * as THREE from "three";
import { GuiData } from "./gui";

// const peculiarityColors = {
//   "normal": 0x00ff00,
//   "centroid": 0xffa500,
//   "Tip": 0x0000ff,
//   "Root": 0x0000ff,
// }

const centroidColor: crapi.RGB = [255, 165, 0];

export default class PlushieBody {
  scene: THREE.Scene;
  nodes: Mesh[];
  centroids: Mesh[];
  links: Mesh[] = [];
  edges: number[][];
  nodeColors: crapi.RGB[];
  peculiarities: crapi.Peculiarities;
  displayMode: "normal" | "debug" = "normal";

  constructor(display: Display, data: crapi.Initialize, guiData: GuiData) {
    this.scene = display.scene;
    this.nodeColors = data.nodes.colors;
    this.peculiarities = data.nodes.peculiarities;
    this.nodes = [];
    for (let [i, point] of data.nodes.points.entries()) {
      let newSphere = create.sphere(this.scene, point, 0.1, this.nodeColor(i));
      this.nodes.push(newSphere);
    }

    this.centroids = [];
    for (let centroidPoint of data.centroids.points) {
      this.centroids.push(
        create.sphere(this.scene, centroidPoint, 0.3, centroidColor)
      );
    }

    this.edges = data.edges;
    if (guiData.showEdges) {
      this.drawLinks();
    }
  }

  destroy() {
    for (let sph of this.nodes) {
      if (sph.geometry) sph.geometry.dispose();
      if (sph.material) (sph.material as THREE.Material).dispose();
      this.scene.remove(sph);
    }
    for (let sph of this.centroids) {
      if (sph.geometry) sph.geometry.dispose();
      if (sph.material) (sph.material as THREE.Material).dispose();
      this.scene.remove(sph);
    }
    this.clearLinks();
  }

  update(data: crapi.Update) {
    this.updatePoints(this.nodes, data.points);
    this.updateCentroids(data.centroids.points);
  }

  updateCentroids(centroids: crapi.Point[]) {
    while (this.centroids.length < centroids.length) {
      this.centroids.push(
        create.sphere(this.scene, [0, 0, 0], 0.3, centroidColor)
      );
    }
    while (this.centroids.length > centroids.length) {
      const removed = this.centroids.pop()!;
      if (removed.geometry) removed.geometry.dispose();
      if (removed.material) (removed.material as THREE.Material).dispose();
      this.scene.remove(removed);
    }

    this.updatePoints(this.centroids, centroids);
  }

  updatePoints(points: Mesh[], newPositions: crapi.Point[]) {
    if (newPositions.length != points.length) {
      console.error("Position data got corrupted");
    }

    for (let i = 0; i < newPositions.length; i++) {
      points[i].position.x = newPositions[i][0];
      points[i].position.y = newPositions[i][1];
      points[i].position.z = newPositions[i][2];
    }
  }

  drawLinks() {
    const width = 0.05;

    for (let from = 0; from < this.edges.length; from++) {
      for (let to of this.edges[from]) {
        if (from >= this.nodes.length || to >= this.nodes.length) {
          continue;
        }
        const color = this.linkColor(from, to);
        let link = create.link(
          this.scene,
          this.nodes[from].position,
          this.nodes[to].position,
          width,
          color
        );
        this.links.push(link);
      }
    }
  }

  clearLinks() {
    for (let link of this.links) {
      if (link.geometry) link.geometry.dispose();
      if (link.material) {
        // hack: link.material is (Material | Material[]). it's always Material here
        const x: any = link.material;
        x.dispose();
      }
      this.scene.remove(link);
    }
    this.links = [];
  }

  nodeColor(index: number): crapi.RGB {
    return this.nodeColors[index];
  }

  linkColor(_from: any, to: any): crapi.RGB {
    if (this.displayMode == "debug") {
      return [52, 51, 48];
    }

    return this.nodeColors[to];
  }
}
