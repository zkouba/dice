// 3D dice rendering + roll animation, built on three.js.
//
// Each die is a real polyhedron whose silhouette matches its type (d4 = tetra,
// d6 = cube, d8 = octa, d10 = pentagonal trapezohedron, d12 = dodeca,
// d20 = icosa). A die tumbles when rolled and then settles so the face bearing
// the engine's value points at the camera. As with the old 2D animation, the
// engine has already decided the value; the tumble is pure theatre and always
// lands on the authoritative result.
//
// One WebGLRenderer is shared across every die: each die owns its own little
// scene + camera and a 2D <canvas> in the DOM, and every frame the shared
// renderer draws that die and blits the pixels into its canvas. This keeps the
// per-die DOM layout (fav badges, wrapping) while using a single GL context.
import * as THREE from "three";
import { type DieType, faces } from "./dice";

// Target bounding radius every die geometry is normalised to, so a d4 and a d20
// occupy roughly the same space on screen.
const DIE_RADIUS = 1.15;

// --- Geometry ------------------------------------------------------------- //

/** A logical (flat) face of a die: where it is and how it is oriented. */
interface Face {
  centroid: THREE.Vector3; // face centre, in the die's local space
  normal: THREE.Vector3; // outward unit normal
  up: THREE.Vector3; // an in-plane unit "up" used to keep numbers upright
  radius: number; // mean distance centre -> vertices, for sizing the number
}

/** Build the raw geometry for a die type, centred on the origin. */
function buildGeometry(die: DieType): THREE.BufferGeometry {
  let geo: THREE.BufferGeometry;
  switch (die) {
    case "d4":
      geo = new THREE.TetrahedronGeometry(1);
      break;
    case "d6":
      geo = new THREE.BoxGeometry(1.3, 1.3, 1.3);
      break;
    case "d8":
      geo = new THREE.OctahedronGeometry(1);
      break;
    case "d10":
      geo = buildD10();
      break;
    case "d12":
      geo = new THREE.DodecahedronGeometry(1);
      break;
    case "d20":
      geo = new THREE.IcosahedronGeometry(1);
      break;
  }
  // Normalise size and make sure every face shades flat.
  const nonIndexed = geo.index ? geo.toNonIndexed() : geo;
  if (nonIndexed !== geo) geo.dispose();
  nonIndexed.computeBoundingSphere();
  const r = nonIndexed.boundingSphere?.radius ?? 1;
  nonIndexed.scale(DIE_RADIUS / r, DIE_RADIUS / r, DIE_RADIUS / r);
  nonIndexed.computeVertexNormals();
  return nonIndexed;
}

/**
 * A pentagonal trapezohedron (the classic d10), built by hand: two apexes and
 * two offset rings of five vertices forming ten kite faces.
 */
function buildD10(): THREE.BufferGeometry {
  const c = Math.cos(Math.PI / 5);
  const h = 1; // apex height
  const ring = 1; // equatorial radius — matching the apex height keeps the
  // die's spindle and belt proportional instead of stretched along the poles.
  // Each kite [apex, U_k, L_k, U_{k+1}] is planar only when the apex, the chord
  // midpoint of the two upper vertices, and the lower vertex are collinear; that
  // pins the ring offset to zig = h·(1−c)/(1+c).
  const zig = (h * (1 - c)) / (1 + c); // ring height (±) for planar faces
  const top = new THREE.Vector3(0, h, 0);
  const bot = new THREE.Vector3(0, -h, 0);
  const upper: THREE.Vector3[] = [];
  const lower: THREE.Vector3[] = [];
  for (let k = 0; k < 5; k++) {
    const aU = (2 * Math.PI * k) / 5;
    const aL = aU + Math.PI / 5;
    upper.push(new THREE.Vector3(Math.cos(aU) * ring, zig, Math.sin(aU) * ring));
    lower.push(
      new THREE.Vector3(Math.cos(aL) * ring, -zig, Math.sin(aL) * ring),
    );
  }

  const verts: number[] = [];
  // Two triangles per kite, wound consistently so each kite has one outward
  // normal (face extraction merges the near-coplanar pair back together).
  const quad = (
    p0: THREE.Vector3,
    p1: THREE.Vector3,
    p2: THREE.Vector3,
    p3: THREE.Vector3,
  ) => {
    for (const p of [p0, p1, p2, p0, p2, p3]) verts.push(p.x, p.y, p.z);
  };
  for (let k = 0; k < 5; k++) {
    const kn = (k + 1) % 5;
    quad(top, upper[k], lower[k], upper[kn]); // upper kite
    quad(bot, lower[k], upper[kn], lower[kn]); // lower kite
  }

  const geo = new THREE.BufferGeometry();
  geo.setAttribute("position", new THREE.Float32BufferAttribute(verts, 3));
  return geo;
}

type PositionAttr = THREE.BufferAttribute | THREE.InterleavedBufferAttribute;

/**
 * Split a die's triangles into logical faces. Regular polyhedra are clustered
 * by shared normal (so a dodecahedron's pentagons come back whole); the d10's
 * kites aren't planar, so its triangles are paired in build order instead.
 */
function extractFaces(geo: THREE.BufferGeometry, die: DieType): Face[] {
  const pos = geo.getAttribute("position") as PositionAttr;
  const triCount = pos.count / 3;

  let groups: number[][];
  if (die === "d10") {
    // buildD10 emits two triangles per kite, in face order.
    groups = [];
    for (let k = 0; k < triCount; k += 2) groups.push([k, k + 1]);
  } else {
    groups = clusterByNormal(pos, triCount);
  }
  return groups.map((g) => describeFace(pos, g));
}

/** Group triangle indices whose outward normals point the same way. */
function clusterByNormal(pos: PositionAttr, triCount: number): number[][] {
  const clusters: { normal: THREE.Vector3; tris: number[] }[] = [];
  const a = new THREE.Vector3();
  const b = new THREE.Vector3();
  const c = new THREE.Vector3();
  const ab = new THREE.Vector3();
  const ac = new THREE.Vector3();
  const n = new THREE.Vector3();
  const tri = new THREE.Vector3();

  for (let t = 0; t < triCount; t++) {
    a.fromBufferAttribute(pos, t * 3);
    b.fromBufferAttribute(pos, t * 3 + 1);
    c.fromBufferAttribute(pos, t * 3 + 2);
    ab.subVectors(b, a);
    ac.subVectors(c, a);
    n.crossVectors(ab, ac).normalize();
    // Orient outward: on a convex solid centred at the origin the outward
    // normal points the same way as the triangle centroid.
    tri.copy(a).add(b).add(c).multiplyScalar(1 / 3);
    if (n.dot(tri) < 0) n.negate();

    let cluster = clusters.find((cl) => n.dot(cl.normal) > 0.98);
    if (!cluster) {
      cluster = { normal: n.clone(), tris: [] };
      clusters.push(cluster);
    }
    cluster.tris.push(t);
  }
  return clusters.map((cl) => cl.tris);
}

/** Describe one logical face from its triangle indices. */
function describeFace(pos: PositionAttr, tris: number[]): Face {
  const normal = new THREE.Vector3();
  const verts: THREE.Vector3[] = [];
  const a = new THREE.Vector3();
  const b = new THREE.Vector3();
  const c = new THREE.Vector3();
  const ab = new THREE.Vector3();
  const ac = new THREE.Vector3();
  const n = new THREE.Vector3();
  const tri = new THREE.Vector3();

  for (const t of tris) {
    a.fromBufferAttribute(pos, t * 3);
    b.fromBufferAttribute(pos, t * 3 + 1);
    c.fromBufferAttribute(pos, t * 3 + 2);
    ab.subVectors(b, a);
    ac.subVectors(c, a);
    n.crossVectors(ab, ac).normalize();
    tri.copy(a).add(b).add(c).multiplyScalar(1 / 3);
    if (n.dot(tri) < 0) n.negate(); // outward
    normal.add(n);
    verts.push(a.clone(), b.clone(), c.clone());
  }
  normal.normalize();

  // De-duplicate vertices so the centroid is the polygon's true centre.
  const uniq = new Map<string, THREE.Vector3>();
  for (const v of verts) {
    uniq.set(`${v.x.toFixed(3)},${v.y.toFixed(3)},${v.z.toFixed(3)}`, v);
  }
  const centroid = new THREE.Vector3();
  for (const v of uniq.values()) centroid.add(v);
  centroid.multiplyScalar(1 / uniq.size);

  // up = the longest in-plane direction from the centre (stable & deterministic).
  let up = new THREE.Vector3(0, 1, 0);
  let best = -1;
  let radius = 0;
  for (const v of uniq.values()) {
    const d = v.clone().sub(centroid);
    radius += d.length();
    const inPlane = d.sub(normal.clone().multiplyScalar(d.dot(normal)));
    const len = inPlane.length();
    if (len > best) {
      best = len;
      up = inPlane.normalize();
    }
  }
  radius /= uniq.size;

  return { centroid, normal, up, radius };
}

// --- Number textures (shared across dice) --------------------------------- //

const numberTextures = new Map<string, THREE.Texture>();

function numberTexture(label: string): THREE.Texture {
  const cached = numberTextures.get(label);
  if (cached) return cached;

  const size = 128;
  const canvas = document.createElement("canvas");
  canvas.width = canvas.height = size;
  const ctx = canvas.getContext("2d")!;
  ctx.clearRect(0, 0, size, size);
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  const fontSize = label.length > 1 ? 64 : 84;
  ctx.font = `800 ${fontSize}px "Segoe UI", system-ui, sans-serif`;
  // Light halo for contrast on any die colour, dark glyph on top.
  ctx.lineWidth = 9;
  ctx.lineJoin = "round";
  ctx.strokeStyle = "rgba(255,255,255,0.9)";
  ctx.strokeText(label, size / 2, size / 2 + 4);
  ctx.fillStyle = "#15171f";
  ctx.fillText(label, size / 2, size / 2 + 4);
  // Underline 6 and 9 so they can be told apart at a glance.
  if (label === "6" || label === "9") {
    ctx.fillRect(size / 2 - 22, size / 2 + 34, 44, 7);
  }

  const tex = new THREE.CanvasTexture(canvas);
  tex.anisotropy = 4;
  tex.needsUpdate = true;
  numberTextures.set(label, tex);
  return tex;
}

/** Read a die's accent colour from the CSS custom properties. */
function dieColor(die: DieType): THREE.Color {
  const css = getComputedStyle(document.documentElement)
    .getPropertyValue(`--${die}`)
    .trim();
  return new THREE.Color(css || "#888888");
}

// --- Shared renderer + animation loop ------------------------------------- //

let renderer: THREE.WebGLRenderer | null = null;
let pixelSize = 0; // drawing-buffer size (css px * dpr)
const live = new Set<Die3D>();
let rafId = 0;
let lastT = 0;

function ensureRenderer(buffer: number) {
  if (!renderer) {
    renderer = new THREE.WebGLRenderer({ alpha: true, antialias: true });
    renderer.setClearColor(0x000000, 0);
  }
  if (buffer !== pixelSize) {
    pixelSize = buffer;
    renderer.setSize(buffer, buffer, false);
  }
}

function tick(now: number) {
  const dt = Math.min((now - lastT) / 1000, 0.05);
  lastT = now;
  let busy = false;
  for (const die of live) {
    if (die.update(dt)) busy = true;
    die.draw(renderer!);
  }
  rafId = busy ? requestAnimationFrame(tick) : 0;
}

function kick() {
  if (!rafId) {
    lastT = performance.now();
    rafId = requestAnimationFrame(tick);
  }
}

const easeOutCubic = (t: number) => 1 - Math.pow(1 - t, 3);

// --- A single die --------------------------------------------------------- //

export class Die3D {
  readonly die: DieType;
  private readonly faceCount: number;
  private readonly scene = new THREE.Scene();
  private readonly camera: THREE.PerspectiveCamera;
  private readonly pivot = new THREE.Group();
  private readonly faces: Face[];
  private readonly labels: THREE.Mesh[] = [];
  private readonly ctx: CanvasRenderingContext2D;

  // Disposables to release on teardown.
  private readonly solidGeo: THREE.BufferGeometry;
  private readonly solidMat: THREE.Material;
  private readonly edgeGeo: THREE.BufferGeometry;
  private readonly edgeMat: THREE.Material;

  // Animation state.
  private phase: "idle" | "spin" | "settle" = "idle";
  private spinAxis = new THREE.Vector3();
  private spinVel = 0;
  private spinLeft = 0;
  private settleLeft = 0;
  private settleDur = 0.55;
  private settleFrom = new THREE.Quaternion();
  private settleTo = new THREE.Quaternion();
  private resolveRoll: (() => void) | null = null;

  constructor(
    die: DieType,
    readonly canvas: HTMLCanvasElement,
    cssSize: number,
  ) {
    this.die = die;
    this.faceCount = faces(die);

    const dpr = Math.min(window.devicePixelRatio || 1, 2);
    const buffer = Math.round(cssSize * dpr);
    canvas.width = buffer;
    canvas.height = buffer;
    canvas.style.width = `${cssSize}px`;
    canvas.style.height = `${cssSize}px`;
    this.ctx = canvas.getContext("2d")!;
    ensureRenderer(buffer);

    this.camera = new THREE.PerspectiveCamera(35, 1, 0.1, 100);
    this.camera.position.set(0, 0, 4.6);

    // Lights: a soft ambient plus a key light from the upper-left.
    this.scene.add(new THREE.AmbientLight(0xffffff, 0.85));
    const key = new THREE.DirectionalLight(0xffffff, 1.15);
    key.position.set(-2, 3, 4);
    this.scene.add(key);
    const rim = new THREE.DirectionalLight(0x9aa0ff, 0.35);
    rim.position.set(3, -2, -3);
    this.scene.add(rim);

    const color = dieColor(die);
    this.solidGeo = buildGeometry(die);
    this.faces = extractFaces(this.solidGeo, die);
    this.solidMat = new THREE.MeshStandardMaterial({
      color,
      flatShading: true,
      metalness: 0.25,
      roughness: 0.45,
      // The hand-built d10 has no guaranteed outward winding; double-siding
      // keeps every face solid (convex dice never show their interior anyway).
      side: THREE.DoubleSide,
    });
    this.pivot.add(new THREE.Mesh(this.solidGeo, this.solidMat));

    // Crisp edges.
    this.edgeGeo = new THREE.EdgesGeometry(this.solidGeo, 1);
    this.edgeMat = new THREE.LineBasicMaterial({
      color: color.clone().multiplyScalar(0.45),
      transparent: true,
      opacity: 0.8,
    });
    this.pivot.add(new THREE.LineSegments(this.edgeGeo, this.edgeMat));

    // A number on every face (face i bears the value i+1).
    this.faces.forEach((face, i) => {
      const label = this.makeLabel(face, String(i + 1));
      this.labels.push(label);
      this.pivot.add(label);
    });

    this.scene.add(this.pivot);
    this.pivot.quaternion.copy(this.idlePose());

    live.add(this);
    this.draw(renderer!); // paint the resting die immediately
  }

  /** Build a textured plane sitting just outside a face, oriented & upright. */
  private makeLabel(face: Face, text: string): THREE.Mesh {
    const right = new THREE.Vector3().crossVectors(face.up, face.normal);
    const basis = new THREE.Matrix4().makeBasis(right, face.up, face.normal);
    const s = face.radius * 1.05;
    const mesh = new THREE.Mesh(
      new THREE.PlaneGeometry(s, s),
      new THREE.MeshBasicMaterial({
        map: numberTexture(text),
        transparent: true,
        depthWrite: false,
      }),
    );
    mesh.quaternion.setFromRotationMatrix(basis);
    mesh.position.copy(face.centroid).addScaledVector(face.normal, 0.01);
    return mesh;
  }

  /** A pleasant tilted resting orientation that shows the die is 3D. */
  private idlePose(): THREE.Quaternion {
    return new THREE.Quaternion().setFromEuler(
      new THREE.Euler(-0.42, 0.6, 0.1),
    );
  }

  /** Orientation that brings `faceIndex` to face the camera, slightly tilted. */
  private faceTowardCamera(faceIndex: number): THREE.Quaternion {
    const face = this.faces[faceIndex];
    const right = new THREE.Vector3().crossVectors(face.up, face.normal);
    const basis = new THREE.Matrix4().makeBasis(right, face.up, face.normal);
    // Invert: map the face's local frame onto the world axes (normal -> +Z,
    // i.e. straight at the camera; up -> +Y, so the number is upright).
    const q = new THREE.Quaternion().setFromRotationMatrix(basis).invert();
    // A gentle tilt so the settled die still reads as a solid, not a flat card.
    const tilt = new THREE.Quaternion().setFromEuler(
      new THREE.Euler(-0.18, 0.22, 0),
    );
    return tilt.multiply(q);
  }

  /**
   * Roll to `value`: tumble, then settle showing that number. The settled face
   * is re-labelled with the literal engine value, so the visible result always
   * matches the authoritative one (even at the range edges).
   */
  roll(value: number): Promise<void> {
    const idx = Math.min(Math.max(value - 1, 0), this.faceCount - 1);
    const mat = this.labels[idx].material as THREE.MeshBasicMaterial;
    mat.map = numberTexture(String(value));
    mat.needsUpdate = true;

    this.settleTo.copy(this.faceTowardCamera(idx));
    this.spinAxis.set(
      Math.random() - 0.5,
      Math.random() - 0.5,
      Math.random() - 0.5,
    );
    if (this.spinAxis.lengthSq() < 1e-4) this.spinAxis.set(0, 1, 0);
    this.spinAxis.normalize();
    this.spinVel = 16 + Math.random() * 8;
    this.spinLeft = 0.6 + Math.random() * 0.35;
    this.settleDur = 0.5 + Math.random() * 0.15;
    this.phase = "spin";

    kick();
    return new Promise((resolve) => {
      this.resolveRoll = resolve;
    });
  }

  /** Advance the animation. Returns true while more frames are needed. */
  update(dt: number): boolean {
    if (this.phase === "idle") return false;

    if (this.phase === "spin") {
      this.pivot.rotateOnWorldAxis(this.spinAxis, this.spinVel * dt);
      this.spinVel *= Math.pow(0.45, dt); // ease the spin down
      this.spinLeft -= dt;
      if (this.spinLeft <= 0) {
        this.settleFrom.copy(this.pivot.quaternion);
        this.settleLeft = this.settleDur;
        this.phase = "settle";
      }
      return true;
    }

    // settle
    this.settleLeft -= dt;
    const t = easeOutCubic(
      Math.min(1, 1 - this.settleLeft / this.settleDur),
    );
    this.pivot.quaternion.slerpQuaternions(
      this.settleFrom,
      this.settleTo,
      t,
    );
    if (this.settleLeft <= 0) {
      this.pivot.quaternion.copy(this.settleTo);
      this.phase = "idle";
      const done = this.resolveRoll;
      this.resolveRoll = null;
      done?.();
      return false;
    }
    return true;
  }

  /** Render this die with the shared renderer and blit into its 2D canvas. */
  draw(r: THREE.WebGLRenderer): void {
    r.render(this.scene, this.camera);
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
    this.ctx.drawImage(
      r.domElement,
      0,
      0,
      this.canvas.width,
      this.canvas.height,
    );
  }

  /** Release GPU resources. Shared number textures are intentionally kept. */
  dispose(): void {
    live.delete(this);
    this.solidGeo.dispose();
    this.solidMat.dispose();
    this.edgeGeo.dispose();
    this.edgeMat.dispose();
    for (const label of this.labels) {
      label.geometry.dispose();
      (label.material as THREE.Material).dispose();
    }
  }
}
