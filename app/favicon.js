// https://editor.p5js.org/

const PHI = 1.61803398874989484820458683436563811772030917980576286213544862270526046281890244970720720418939113748475;


let large_params = {
  hor_inset: 20,
  ver_inset: 23,
  canvas_size: 400,
  stroke_weight: 20,
  overlap: 2,
};
let params32 = {
  hor_inset: 2,
  ver_inset: 2.5,
  canvas_size: 32,
  stroke_weight: 1.5,
  overlap: 0,
};
let params16 = {
  hor_inset: 1,
  ver_inset: 1.5,
  canvas_size: 16,
  stroke_weight: 0.5,
  overlap: 0,
};
let params24 = {
  hor_inset: 1,
  ver_inset: 2.5,
  canvas_size: 24,
  stroke_weight: 1,
  overlap: 0,
};

let params = params24;

function setup() {
  createCanvas(params.canvas_size, params.canvas_size);
}

function draw() {
  smooth();
  parametered(params);
}

function parametered(params) {
  background(255, 0);
  let inset = params.hor_inset;
  f(0+inset,0, params);
  f(params.canvas_size/2-inset,0, params);
}

function f(x,y, params) {
  noFill()
  strokeWeight(params.stroke_weight);
  let reduce = params.ver_inset;
  let size = params.canvas_size / 2 - params.ver_inset;
  let offset = params.canvas_size / 4;
  arc(x+offset,y+offset,size,size, HALF_PI, HALF_PI*3);
  let left = x + offset - size/2 + params.overlap;
  line(
    left,
    y + offset + size/2 - params.overlap,
    left,
    params.canvas_size-params.ver_inset
  );
  
  let rightFrac = (2-PHI)/PHI; // mkaes the bounding rectangle golden
  let right = x+offset+(size/2)*rightFrac;
  line(x+offset, y+offset-size/2, right, y+offset-size/2);
  line(x+offset, y+offset+size/2, right, y+offset+size/2);
}