import * as create from "./lib/create";

export default class Plushie {
  constructor() {
    this.edges = edges_tmp;
    this.stitchSpheres = [];
    this.stitchPositions = [];
    this.links = [];
    for (let point of points_tmp) {
      let sph = create.sphere(point, 0.1);
      this.stitchPositions.push(sph.position);
      this.stitchSpheres.push(sph);
    }
    for (let from = 0; from < this.edges.length; from++) {
      for (let to of this.edges[from]) {
        let link = create.link(this.stitchPositions[from], this.stitchPositions[to], 0.02, "red")
        this.links.push(link);
      }
    }
  }

  getId(obj) {
    return this.stitchSpheres.findIndex((o) => o == obj);
  }

  parse(data) {
    if (data.length != this.stitchSpheres.length) {
      throw "WHYYYYY";
    }

    for (let i = 0; i < data.length; i++) {
      this.stitchPositions[i].x = data[i][0]
      this.stitchPositions[i].y = data[i][1]
      this.stitchPositions[i].z = data[i][2]
    }

    for (let link of this.links) {
      if (link.geometry) link.geometry.dispose();
      if (link.material) link.material.dispose();
      create.scene.remove(link);
    }
    this.links = [];

    for (let from = 0; from < this.edges.length; from++) {
      for (let to of this.edges[from]) {
        let link = create.link(this.stitchPositions[from], this.stitchPositions[to], 0.02, "red")
        this.links.push(link);
      }
    }
  }
}

const points_tmp = [
  [
    0.0,
    0.0,
    0.0
  ],
  [
    0.0,
    4.939392566680908,
    0.0
  ],
  [
    1.0,
    0.0,
    0.0
  ],
  [
    0.4999999701976776,
    0.0,
    0.866025447845459
  ],
  [
    -0.5000000596046448,
    0.0,
    0.8660253882408142
  ],
  [
    -1.0,
    0.0,
    -8.742277657347586e-8
  ],
  [
    -0.49999991059303284,
    0.0,
    -0.866025447845459
  ],
  [
    0.505050539970398,
    0.010101273655891418,
    -0.857277512550354
  ],
  [
    0.9949493408203125,
    0.9898987412452698,
    -0.00874795950949192
  ],
  [
    0.4999999701976776,
    1.0,
    0.866025447845459
  ],
  [
    -0.5000000596046448,
    1.0,
    0.8660253882408142
  ],
  [
    -1.0,
    1.0,
    -8.742277657347586e-8
  ],
  [
    -0.49999991059303284,
    1.0,
    -0.866025447845459
  ],
  [
    0.505050539970398,
    1.010101318359375,
    -0.857277512550354
  ],
  [
    0.9949493408203125,
    1.989898681640625,
    -0.00874795950949192
  ],
  [
    0.4999999701976776,
    2.0,
    0.866025447845459
  ],
  [
    -0.5000000596046448,
    2.0,
    0.8660253882408142
  ],
  [
    -1.0,
    2.0,
    -8.742277657347586e-8
  ],
  [
    -0.49999991059303284,
    2.0,
    -0.866025447845459
  ],
  [
    0.505050539970398,
    2.010101318359375,
    -0.857277512550354
  ],
  [
    0.9949493408203125,
    2.989898681640625,
    -0.00874795950949192
  ],
  [
    0.4999999701976776,
    3.0,
    0.866025447845459
  ],
  [
    -0.5000000596046448,
    3.0,
    0.8660253882408142
  ],
  [
    -1.0,
    3.0,
    -8.742277657347586e-8
  ],
  [
    -0.49999991059303284,
    3.0,
    -0.866025447845459
  ],
  [
    0.505050539970398,
    3.010101318359375,
    -0.857277512550354
  ],
  [
    1.9848480224609375,
    4.0,
    -0.008747939951717854
  ],
  [
    0.9949493408203125,
    4.010101318359375,
    1.723302960395813
  ],
  [
    -0.994949460029602,
    4.010101318359375,
    1.7233028411865234
  ],
  [
    -1.989898681640625,
    4.010101318359375,
    -1.54094252025061e-7
  ],
  [
    -0.994949221611023,
    4.010101318359375,
    -1.723302960395813
  ],
  [
    0.994949221611023,
    4.010101318359375,
    -1.723302960395813
  ]
];

const edges_tmp = [
  [
    2,
    3,
    4,
    5,
    6,
    7
  ],
  [
    26,
    27,
    28,
    29,
    30,
    31
  ],
  [
    3,
    8
  ],
  [
    4,
    9
  ],
  [
    5,
    10
  ],
  [
    6,
    11
  ],
  [
    7,
    12
  ],
  [
    8,
    13
  ],
  [
    9,
    14
  ],
  [
    10,
    15
  ],
  [
    11,
    16
  ],
  [
    12,
    17
  ],
  [
    13,
    18
  ],
  [
    14,
    19
  ],
  [
    15,
    20
  ],
  [
    16,
    21
  ],
  [
    17,
    22
  ],
  [
    18,
    23
  ],
  [
    19,
    24
  ],
  [
    20,
    25
  ],
  [
    21,
    26
  ],
  [
    22,
    27
  ],
  [
    23,
    28
  ],
  [
    24,
    29
  ],
  [
    25,
    30
  ],
  [
    26,
    31
  ],
  [
    27
  ],
  [
    28
  ],
  [
    29
  ],
  [
    30
  ],
  [
    31
  ],
  []
]