// 3D dice tray, built on three.js.
//
// Every die is a real polyhedron whose silhouette matches its type (d4 = tetra,
// d6 = cube, d8 = octa, d10 = pentagonal trapezohedron, d12 = dodeca,
// d20 = icosa). Unlike the old design — where each die lived in its own little
// scene and was rendered in isolation — all dice now share ONE scene, ONE
// camera and ONE canvas so they can physically tumble around a tray, bounce off
// its walls and collide with one another.
//
// The engine has already decided each die's value before the throw; the physics
// is pure theatre. When the dice come to rest we tip each one flat onto a face
// and paint the authoritative value onto whichever face ends up facing the
// camera, so the visible result always matches the engine result.
import * as THREE from "three";
import { type DieType, type Fav, faces } from "./dice";

// Target bounding (circumscribed) radius every die geometry is normalised to,
// so a d4 and a d20 occupy roughly the same space in the tray. This doubles as
// each die's collision radius.
const DIE_RADIUS = 0.67;

// --- Geometry ------------------------------------------------------------- //

/** A logical (flat) face of a die: where it is and how it is oriented. */
interface Face {
  centroid: THREE.Vector3; // face centre, in the die's local space
  normal: THREE.Vector3; // outward unit normal
  up: THREE.Vector3; // an in-plane unit "up" used to keep numbers upright
  radius: number; // mean distance centre -> vertices, for sizing the number
  verts: THREE.Vector3[]; // the face's unique corner vertices (local space)
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

  return { centroid, normal, up, radius, verts: [...uniq.values()] };
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

/** A round +/− badge texture used for favoured / illfavoured dice. */
const favTextures = new Map<Fav, THREE.Texture>();

function favTexture(fav: Fav): THREE.Texture {
  const cached = favTextures.get(fav);
  if (cached) return cached;

  const size = 64;
  const canvas = document.createElement("canvas");
  canvas.width = canvas.height = size;
  const ctx = canvas.getContext("2d")!;
  const root = getComputedStyle(document.documentElement);
  const good = root.getPropertyValue("--good").trim() || "#6bcf8e";
  const bad = root.getPropertyValue("--bad").trim() || "#ef6f6c";
  ctx.fillStyle = fav === "favoured" ? good : bad;
  ctx.beginPath();
  ctx.arc(size / 2, size / 2, size / 2 - 3, 0, Math.PI * 2);
  ctx.fill();
  ctx.fillStyle = "#15171f";
  ctx.font = `900 ${size * 0.7}px "Segoe UI", system-ui, sans-serif`;
  ctx.textAlign = "center";
  ctx.textBaseline = "middle";
  ctx.fillText(fav === "favoured" ? "+" : "−", size / 2, size / 2 + 3);

  const tex = new THREE.CanvasTexture(canvas);
  tex.needsUpdate = true;
  favTextures.set(fav, tex);
  return tex;
}

/** A soft round blob used as a cheap contact shadow under each die. */
let blobTexture: THREE.Texture | null = null;

function shadowTexture(): THREE.Texture {
  if (blobTexture) return blobTexture;
  const size = 128;
  const canvas = document.createElement("canvas");
  canvas.width = canvas.height = size;
  const ctx = canvas.getContext("2d")!;
  const g = ctx.createRadialGradient(
    size / 2,
    size / 2,
    0,
    size / 2,
    size / 2,
    size / 2,
  );
  g.addColorStop(0, "rgba(0,0,0,0.55)");
  g.addColorStop(0.7, "rgba(0,0,0,0.25)");
  g.addColorStop(1, "rgba(0,0,0,0)");
  ctx.fillStyle = g;
  ctx.fillRect(0, 0, size, size);
  blobTexture = new THREE.CanvasTexture(canvas);
  blobTexture.needsUpdate = true;
  return blobTexture;
}

/** Read a die's accent colour from the CSS custom properties. */
function dieColor(die: DieType): THREE.Color {
  const css = getComputedStyle(document.documentElement)
    .getPropertyValue(`--${die}`)
    .trim();
  return new THREE.Color(css || "#888888");
}

/** Read an arbitrary CSS custom property as a three.js colour. */
function cssColor(name: string, fallback: string): THREE.Color {
  const css = getComputedStyle(document.documentElement)
    .getPropertyValue(name)
    .trim();
  return new THREE.Color(css || fallback);
}

const easeOutCubic = (t: number) => 1 - Math.pow(1 - t, 3);

// --- Physics tuning ------------------------------------------------------- //

const GRAVITY = 34; // world units / s²
const FLOOR_RESTITUTION = 0.42;
const WALL_RESTITUTION = 0.55;
const DIE_RESTITUTION = 0.5;
const AIR_DAMP = 0.999; // per second, horizontal drift
const GROUND_DAMP = 0.12; // remaining fraction of horizontal speed per second
const REST_SPEED = 0.5; // below this (and grounded) a die is "asleep"
const REST_SPIN = 1.2;
const REST_HOLD = 0.28; // seconds a die must stay slow before it counts as settled
const MAX_ROLL = 5; // hard cap on tumble time (s) before we force a settle
const SETTLE_DUR = 0.32; // tip-flat-onto-a-face animation (s)
const WALL_HEIGHT = 0.55;

// --- A single physical die ------------------------------------------------ //

/** One die: its meshes, its logical faces, and its rigid-body state. */
class Body {
  readonly die: DieType;
  readonly faceCount: number;
  readonly group = new THREE.Group();
  readonly faces: Face[];
  readonly labels: THREE.Mesh[] = [];
  // d4 only: numbers live on the corners, not the faces. `vertices` are the four
  // local corner positions; each `cornerLabel` records which vertex it belongs
  // to, since one vertex's number is repeated on all three faces meeting there.
  readonly vertices: THREE.Vector3[] = [];
  private readonly cornerLabels: { vertex: number; mesh: THREE.Mesh }[] = [];
  readonly favSprite: THREE.Sprite | null = null;
  readonly shadow: THREE.Mesh;
  readonly radius = DIE_RADIUS;

  // Rigid-body state (world space).
  readonly pos = new THREE.Vector3();
  readonly vel = new THREE.Vector3();
  readonly angVel = new THREE.Vector3();
  readonly quat = new THREE.Quaternion();
  restTimer = 0;

  // Disposables.
  private readonly solidGeo: THREE.BufferGeometry;
  private readonly solidMat: THREE.Material;
  private readonly edgeGeo: THREE.BufferGeometry;
  private readonly edgeMat: THREE.Material;

  constructor(die: DieType, fav: Fav) {
    this.die = die;
    this.faceCount = faces(die);

    const color = dieColor(die);
    this.solidGeo = buildGeometry(die);
    this.faces = extractFaces(this.solidGeo, die);
    this.solidMat = new THREE.MeshStandardMaterial({
      color,
      flatShading: true,
      metalness: 0.2,
      roughness: 0.5,
      // The hand-built d10 has no guaranteed outward winding; double-siding
      // keeps every face solid (convex dice never show their interior anyway).
      side: THREE.DoubleSide,
    });
    this.group.add(new THREE.Mesh(this.solidGeo, this.solidMat));

    this.edgeGeo = new THREE.EdgesGeometry(this.solidGeo, 1);
    this.edgeMat = new THREE.LineBasicMaterial({
      color: color.clone().multiplyScalar(0.4),
      transparent: true,
      opacity: 0.85,
    });
    this.group.add(new THREE.LineSegments(this.edgeGeo, this.edgeMat));

    if (this.die === "d4") {
      // A d4 has no top face when it rests — a vertex points up — so it carries
      // numbered corners: each vertex's number appears on all faces around it.
      this.buildCornerLabels();
    } else {
      // A number on every face (face i bears the value i+1 by default).
      this.faces.forEach((face, i) => {
        const label = this.makeLabel(face, String(i + 1));
        this.labels.push(label);
        this.group.add(label);
      });
    }

    // Favoured / illfavoured marker that floats above the die.
    if (fav !== "neutral") {
      this.favSprite = new THREE.Sprite(
        new THREE.SpriteMaterial({
          map: favTexture(fav),
          transparent: true,
          depthTest: false,
        }),
      );
      this.favSprite.scale.setScalar(0.7);
    }

    // Cheap contact shadow.
    this.shadow = new THREE.Mesh(
      new THREE.PlaneGeometry(1, 1),
      new THREE.MeshBasicMaterial({
        map: shadowTexture(),
        transparent: true,
        depthWrite: false,
      }),
    );
    this.shadow.rotation.x = -Math.PI / 2;
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

  /**
   * Build the d4's corner numbers: identify the four shared vertices, then drop
   * a small number near each corner of every face (so each vertex's number is
   * printed three times, once on each adjacent face).
   */
  private buildCornerLabels(): void {
    const key = (v: THREE.Vector3) =>
      `${v.x.toFixed(2)},${v.y.toFixed(2)},${v.z.toFixed(2)}`;
    const index = new Map<string, number>();
    for (const f of this.faces) {
      for (const v of f.verts) {
        const k = key(v);
        if (!index.has(k)) {
          index.set(k, this.vertices.length);
          this.vertices.push(v.clone());
        }
      }
    }
    for (const f of this.faces) {
      for (const v of f.verts) {
        const vi = index.get(key(v))!;
        const mesh = this.makeCornerLabel(f, v, String(vi + 1));
        this.cornerLabels.push({ vertex: vi, mesh });
        this.group.add(mesh);
      }
    }
  }

  /**
   * A small number near one corner of a face, oriented so its top points at the
   * corner — that way it reads upright when that corner is the die's apex.
   */
  private makeCornerLabel(
    face: Face,
    vertex: THREE.Vector3,
    text: string,
  ): THREE.Mesh {
    const up = vertex.clone().sub(face.centroid).normalize();
    const right = new THREE.Vector3().crossVectors(up, face.normal);
    const basis = new THREE.Matrix4().makeBasis(right, up, face.normal);
    const s = face.radius * 0.55;
    const mesh = new THREE.Mesh(
      new THREE.PlaneGeometry(s, s),
      new THREE.MeshBasicMaterial({
        map: numberTexture(text),
        transparent: true,
        depthWrite: false,
      }),
    );
    mesh.quaternion.setFromRotationMatrix(basis);
    mesh.position
      .copy(face.centroid)
      .addScaledVector(vertex.clone().sub(face.centroid), 0.52)
      .addScaledVector(face.normal, 0.01);
    return mesh;
  }

  /** Reset every face back to its default number (call before a fresh throw). */
  resetLabels(): void {
    if (this.die === "d4") {
      for (const c of this.cornerLabels) {
        const mat = c.mesh.material as THREE.MeshBasicMaterial;
        mat.map = numberTexture(String(c.vertex + 1));
        mat.needsUpdate = true;
      }
      return;
    }
    this.labels.forEach((label, i) => {
      const mat = label.material as THREE.MeshBasicMaterial;
      mat.map = numberTexture(String(i + 1));
      mat.needsUpdate = true;
    });
  }

  /** Paint `value` onto face `idx`. */
  setFaceValue(idx: number, value: number): void {
    const mat = this.labels[idx].material as THREE.MeshBasicMaterial;
    mat.map = numberTexture(String(value));
    mat.needsUpdate = true;
  }

  /** d4: paint `value` onto vertex `vi` (every corner copy of that vertex). */
  setVertexValue(vi: number, value: number): void {
    for (const c of this.cornerLabels) {
      if (c.vertex !== vi) continue;
      const mat = c.mesh.material as THREE.MeshBasicMaterial;
      mat.map = numberTexture(String(value));
      mat.needsUpdate = true;
    }
  }

  /** d4: index of the vertex pointing most straight up under `quat`. */
  topVertex(quat: THREE.Quaternion): number {
    const wv = new THREE.Vector3();
    let best = -Infinity;
    let idx = 0;
    for (let i = 0; i < this.vertices.length; i++) {
      const y = wv.copy(this.vertices[i]).applyQuaternion(quat).y;
      if (y > best) {
        best = y;
        idx = i;
      }
    }
    return idx;
  }

  /**
   * Given an orientation, find the face most flush with the floor and return
   * the rotation that tips the die to rest flat on it, plus the centre height
   * at which it then sits (its inradius for that face).
   */
  restPose(from: THREE.Quaternion): {
    quat: THREE.Quaternion;
    height: number;
    bottom: number;
  } {
    let bottom = 0;
    let lowest = Infinity;
    const wn = new THREE.Vector3();
    for (let i = 0; i < this.faces.length; i++) {
      wn.copy(this.faces[i].normal).applyQuaternion(from);
      if (wn.y < lowest) {
        lowest = wn.y;
        bottom = i;
      }
    }
    wn.copy(this.faces[bottom].normal).applyQuaternion(from);
    const tip = new THREE.Quaternion().setFromUnitVectors(
      wn,
      new THREE.Vector3(0, -1, 0),
    );
    const quat = tip.multiply(from);
    // Distance centre -> the resting face plane = how high the centre sits.
    const height = this.faces[bottom].centroid.dot(this.faces[bottom].normal);
    return { quat, height, bottom };
  }

  /**
   * Pick the face to bear the result. We want the flat, horizontal top face so
   * the number reads square-on: the face whose normal points most straight up.
   * For dice with parallel opposite faces (everything but the d4) that's an
   * exactly horizontal face once the die rests flat. The d4 has no top face —
   * its three upper faces tie, so we break the tie toward the camera.
   */
  faceForReadout(quat: THREE.Quaternion, camDir: THREE.Vector3): number {
    const wn = new THREE.Vector3();
    const ys = this.faces.map((f) => wn.copy(f.normal).applyQuaternion(quat).y);
    const maxY = Math.max(...ys);
    let best = -Infinity;
    let idx = 0;
    for (let i = 0; i < this.faces.length; i++) {
      if (ys[i] < maxY - 0.05) continue; // only the topmost face(s)
      wn.copy(this.faces[i].normal).applyQuaternion(quat);
      const d = wn.dot(camDir);
      if (d > best) {
        best = d;
        idx = i;
      }
    }
    return idx;
  }

  /** Whether the die is on the floor and barely moving. */
  isAsleep(): boolean {
    return (
      this.pos.y <= this.radius + 0.05 &&
      this.vel.length() < REST_SPEED &&
      this.angVel.length() < REST_SPIN
    );
  }

  /** Push the latest physics state into the three.js objects. */
  sync(): void {
    this.group.position.copy(this.pos);
    this.group.quaternion.copy(this.quat);
    if (this.favSprite) {
      this.favSprite.position.set(
        this.pos.x,
        this.pos.y + this.radius + 0.45,
        this.pos.z,
      );
    }
    // Shadow fades and shrinks as the die rises off the floor.
    const lift = Math.max(0, this.pos.y - this.radius);
    const k = 1 / (1 + lift * 0.6);
    this.shadow.position.set(this.pos.x, 0.02, this.pos.z);
    this.shadow.scale.setScalar(this.radius * 3.1 * k);
    (this.shadow.material as THREE.MeshBasicMaterial).opacity = 0.6 * k;
  }

  dispose(): void {
    this.solidGeo.dispose();
    this.solidMat.dispose();
    this.edgeGeo.dispose();
    this.edgeMat.dispose();
    for (const label of this.labels) {
      label.geometry.dispose();
      (label.material as THREE.Material).dispose();
    }
    for (const c of this.cornerLabels) {
      c.mesh.geometry.dispose();
      (c.mesh.material as THREE.Material).dispose();
    }
    this.shadow.geometry.dispose();
    (this.shadow.material as THREE.Material).dispose();
    this.favSprite?.material.dispose();
  }
}

// --- The tray ------------------------------------------------------------- //

/**
 * A 3D tray holding a set of dice that tumble, bounce off the walls and off
 * each other when rolled, then settle showing the engine's values.
 */
export class DiceTray {
  private readonly renderer: THREE.WebGLRenderer;
  private readonly scene = new THREE.Scene();
  private readonly camera: THREE.PerspectiveCamera;
  private readonly bodies: Body[] = [];
  private readonly floor: THREE.Mesh;
  private readonly walls: THREE.Mesh[] = [];
  private readonly ro: ResizeObserver;

  // Half-extents of the playable floor (recomputed from the camera on resize).
  private hx = 8;
  private hz = 6;

  private phase: "idle" | "roll" | "settle" = "idle";
  private raf = 0;
  private lastT = 0;
  private rollClock = 0;
  private settleT = 0;
  private settleData: {
    fromPos: THREE.Vector3;
    fromQuat: THREE.Quaternion;
    toPos: THREE.Vector3;
    toQuat: THREE.Quaternion;
  }[] = [];
  private resolveRoll: (() => void) | null = null;

  constructor(
    readonly canvas: HTMLCanvasElement,
    dice: { die: DieType; fav: Fav }[],
  ) {
    this.renderer = new THREE.WebGLRenderer({ canvas, antialias: true });
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
    this.scene.background = cssColor("--bg", "#11131a");

    // A 3/4 top-down view (~56° elevation). The distance is chosen so the dice
    // read at a comfortable size; the playable extents are derived from it.
    this.camera = new THREE.PerspectiveCamera(45, 1, 0.1, 200);
    this.camera.position.set(0, 6.5, 4.4);
    this.camera.lookAt(0, 0, 0);

    // Lighting.
    this.scene.add(new THREE.AmbientLight(0xffffff, 0.7));
    const key = new THREE.DirectionalLight(0xffffff, 1.1);
    key.position.set(-6, 12, 6);
    this.scene.add(key);
    const fill = new THREE.DirectionalLight(0x9aa0ff, 0.3);
    fill.position.set(5, 4, -6);
    this.scene.add(fill);

    // Floor (a unit plane laid flat; scaled to the tray extents on resize).
    const floorGeo = new THREE.PlaneGeometry(1, 1).rotateX(-Math.PI / 2);
    this.floor = new THREE.Mesh(
      floorGeo,
      new THREE.MeshStandardMaterial({
        color: cssColor("--panel-2", "#232737"),
        roughness: 0.95,
        metalness: 0,
      }),
    );
    this.floor.position.y = 0;
    this.scene.add(this.floor);

    // Four low walls (unit boxes; scaled & positioned on resize).
    const wallMat = new THREE.MeshStandardMaterial({
      color: cssColor("--border", "#2c3144"),
      roughness: 0.9,
    });
    for (let i = 0; i < 4; i++) {
      const wall = new THREE.Mesh(new THREE.BoxGeometry(1, 1, 1), wallMat);
      this.walls.push(wall);
      this.scene.add(wall);
    }

    // Build the dice and lay them out at rest.
    for (const spec of dice) {
      const body = new Body(spec.die, spec.fav);
      this.bodies.push(body);
      this.scene.add(body.group, body.shadow);
      if (body.favSprite) this.scene.add(body.favSprite);
    }

    this.resize();
    this.layoutAtRest();
    for (const b of this.bodies) b.sync();

    // Track the tray's pixel size.
    this.ro = new ResizeObserver(() => this.resize());
    this.ro.observe(canvas);

    this.renderOnce();
  }

  /** Match the renderer + camera to the canvas's CSS size, reframe the tray. */
  private resize(): void {
    const w = this.canvas.clientWidth || 1;
    const h = this.canvas.clientHeight || 1;
    this.renderer.setSize(w, h, false);
    this.camera.aspect = w / h;
    this.camera.updateProjectionMatrix();
    this.computeExtents();
    this.layoutTray();
    // Keep resting dice inside the (possibly new) walls.
    for (const b of this.bodies) {
      b.pos.x = THREE.MathUtils.clamp(
        b.pos.x,
        -this.hx + b.radius,
        this.hx - b.radius,
      );
      b.pos.z = THREE.MathUtils.clamp(
        b.pos.z,
        -this.hz + b.radius,
        this.hz - b.radius,
      );
      b.sync();
    }
    if (this.phase === "idle") this.renderOnce();
  }

  /**
   * Find the largest floor rectangle centred at the origin that the camera can
   * see (with a safety margin), by ray-casting the viewport corners onto y=0.
   */
  private computeExtents(): void {
    const margin = 0.9;
    const corners = [
      [-1, -1],
      [1, -1],
      [1, 1],
      [-1, 1],
    ];
    let hx = Infinity;
    let hz = Infinity;
    for (const [nx, ny] of corners) {
      const p = new THREE.Vector3(nx * margin, ny * margin, 0.5).unproject(
        this.camera,
      );
      const dir = p.sub(this.camera.position).normalize();
      const t = -this.camera.position.y / dir.y;
      const hit = this.camera.position.clone().addScaledVector(dir, t);
      hx = Math.min(hx, Math.abs(hit.x));
      hz = Math.min(hz, Math.abs(hit.z));
    }
    this.hx = hx;
    this.hz = hz;
  }

  /** Size the floor and reposition the walls to the current extents. */
  private layoutTray(): void {
    this.floor.scale.set(this.hx * 2, 1, this.hz * 2);
    const t = 0.3;
    const place = (
      wall: THREE.Mesh,
      x: number,
      z: number,
      sx: number,
      sz: number,
    ) => {
      wall.scale.set(sx, WALL_HEIGHT, sz);
      wall.position.set(x, WALL_HEIGHT / 2, z);
    };
    place(this.walls[0], -this.hx - t / 2, 0, t, this.hz * 2 + t * 2); // left
    place(this.walls[1], this.hx + t / 2, 0, t, this.hz * 2 + t * 2); // right
    place(this.walls[2], 0, -this.hz - t / 2, this.hx * 2 + t * 2, t); // back
    place(this.walls[3], 0, this.hz + t / 2, this.hx * 2 + t * 2, t); // front
  }

  /** Spread the dice across the floor, each resting flat on a face. */
  private layoutAtRest(): void {
    const n = this.bodies.length;
    if (n === 0) return;
    const cols = Math.ceil(Math.sqrt(n));
    const rows = Math.ceil(n / cols);
    const gx = (this.hx * 1.4) / cols;
    const gz = (this.hz * 1.4) / rows;
    this.bodies.forEach((b, i) => {
      const col = i % cols;
      const row = Math.floor(i / cols);
      const rest = b.restPose(
        new THREE.Quaternion().setFromEuler(
          new THREE.Euler(Math.random(), Math.random(), Math.random()),
        ),
      );
      b.quat.copy(rest.quat);
      b.pos.set(
        (col - (cols - 1) / 2) * gx,
        rest.height,
        (row - (rows - 1) / 2) * gz,
      );
      b.vel.set(0, 0, 0);
      b.angVel.set(0, 0, 0);
    });
  }

  /**
   * Throw the dice: each starts somewhere over the tray with a random velocity
   * and spin. They tumble, bounce and collide, then settle showing `values`
   * (one per die, in order).
   */
  roll(values: number[]): Promise<void> {
    if (this.bodies.length === 0) return Promise.resolve();

    for (let i = 0; i < this.bodies.length; i++) {
      const b = this.bodies[i];
      b.resetLabels();
      // Toss from a random spot, biased toward the back so dice roll forward.
      // Keep the launch height below the camera so dice never fly past it.
      b.pos.set(
        THREE.MathUtils.randFloatSpread(this.hx * 1.2),
        b.radius + 1 + Math.random() * 1.5,
        -this.hz * 0.5 + THREE.MathUtils.randFloatSpread(this.hz * 0.8),
      );
      const dir = Math.random() * Math.PI * 2;
      const speed = 5 + Math.random() * 5;
      b.vel.set(Math.cos(dir) * speed, 0.5 + Math.random() * 1.5, Math.sin(dir) * speed);
      b.angVel.set(
        THREE.MathUtils.randFloatSpread(24),
        THREE.MathUtils.randFloatSpread(24),
        THREE.MathUtils.randFloatSpread(24),
      );
      b.restTimer = 0;
    }
    // Remember the values; they're applied when the dice come to rest.
    this.pendingValues = values;
    this.phase = "roll";
    this.rollClock = 0;
    this.start();
    return new Promise((resolve) => {
      this.resolveRoll = resolve;
    });
  }

  private pendingValues: number[] = [];

  // --- Simulation --------------------------------------------------------- //

  private step(dt: number): void {
    // Fixed sub-steps keep collisions stable regardless of frame rate.
    const sub = 1 / 120;
    let remaining = dt;
    while (remaining > 1e-5) {
      const h = Math.min(sub, remaining);
      this.integrate(h);
      remaining -= h;
    }
  }

  private integrate(dt: number): void {
    const tmp = new THREE.Vector3();
    const axis = new THREE.Vector3();
    const dq = new THREE.Quaternion();

    for (const b of this.bodies) {
      // Gravity + position.
      b.vel.y -= GRAVITY * dt;
      b.pos.addScaledVector(b.vel, dt);

      // Orientation from angular velocity.
      const w = b.angVel.length();
      if (w > 1e-6) {
        axis.copy(b.angVel).multiplyScalar(1 / w);
        dq.setFromAxisAngle(axis, w * dt);
        b.quat.premultiply(dq).normalize();
      }

      // Floor.
      if (b.pos.y < b.radius) {
        b.pos.y = b.radius;
        if (b.vel.y < 0) b.vel.y = -b.vel.y * FLOOR_RESTITUTION;
        // Friction + couple horizontal motion into a rolling spin.
        const damp = Math.pow(GROUND_DAMP, dt);
        b.vel.x *= damp;
        b.vel.z *= damp;
        // Rolling: ω = (up × v) / r, blended in so it looks like it grips.
        const target = tmp.set(b.vel.z, 0, -b.vel.x).multiplyScalar(1 / b.radius);
        b.angVel.lerp(target, 0.25);
      } else {
        const d = Math.pow(AIR_DAMP, dt);
        b.vel.x *= d;
        b.vel.z *= d;
      }

      // Walls.
      if (b.pos.x < -this.hx + b.radius) {
        b.pos.x = -this.hx + b.radius;
        if (b.vel.x < 0) b.vel.x = -b.vel.x * WALL_RESTITUTION;
      } else if (b.pos.x > this.hx - b.radius) {
        b.pos.x = this.hx - b.radius;
        if (b.vel.x > 0) b.vel.x = -b.vel.x * WALL_RESTITUTION;
      }
      if (b.pos.z < -this.hz + b.radius) {
        b.pos.z = -this.hz + b.radius;
        if (b.vel.z < 0) b.vel.z = -b.vel.z * WALL_RESTITUTION;
      } else if (b.pos.z > this.hz - b.radius) {
        b.pos.z = this.hz - b.radius;
        if (b.vel.z > 0) b.vel.z = -b.vel.z * WALL_RESTITUTION;
      }
    }

    this.resolveCollisions();

    // Sleep accounting.
    for (const b of this.bodies) {
      if (b.isAsleep()) b.restTimer += dt;
      else b.restTimer = 0;
    }
  }

  /** Treat every die as a sphere and resolve pairwise overlaps + impulses. */
  private resolveCollisions(): void {
    const n = new THREE.Vector3();
    for (let i = 0; i < this.bodies.length; i++) {
      for (let j = i + 1; j < this.bodies.length; j++) {
        const a = this.bodies[i];
        const b = this.bodies[j];
        n.subVectors(b.pos, a.pos);
        const dist = n.length();
        const min = a.radius + b.radius;
        if (dist >= min || dist < 1e-4) continue;
        n.multiplyScalar(1 / dist);
        // Separate.
        const overlap = (min - dist) / 2;
        a.pos.addScaledVector(n, -overlap);
        b.pos.addScaledVector(n, overlap);
        // Impulse along the contact normal.
        const rel =
          (b.vel.x - a.vel.x) * n.x +
          (b.vel.y - a.vel.y) * n.y +
          (b.vel.z - a.vel.z) * n.z;
        if (rel < 0) {
          const jimp = (-(1 + DIE_RESTITUTION) * rel) / 2;
          a.vel.addScaledVector(n, -jimp);
          b.vel.addScaledVector(n, jimp);
          // A knock also imparts a little extra tumble.
          const kick = jimp * 0.6;
          a.angVel.x += THREE.MathUtils.randFloatSpread(kick);
          a.angVel.z += THREE.MathUtils.randFloatSpread(kick);
          b.angVel.x += THREE.MathUtils.randFloatSpread(kick);
          b.angVel.z += THREE.MathUtils.randFloatSpread(kick);
        }
      }
    }
  }

  private allAsleep(): boolean {
    return this.bodies.every((b) => b.restTimer >= REST_HOLD);
  }

  /** Compute each die's tip-flat target + paint the result, then animate. */
  private beginSettle(): void {
    const camDir = new THREE.Vector3();
    this.settleData = this.bodies.map((b, i) => {
      const rest = b.restPose(b.quat);
      if (b.die === "d4") {
        // The result is the corner pointing up once the die rests on a face.
        const vi = b.topVertex(rest.quat);
        b.setVertexValue(vi, this.pendingValues[i] ?? vi + 1);
      } else {
        camDir.subVectors(this.camera.position, b.pos).normalize();
        const faceIdx = b.faceForReadout(rest.quat, camDir);
        b.setFaceValue(faceIdx, this.pendingValues[i] ?? faceIdx + 1);
      }
      return {
        fromPos: b.pos.clone(),
        fromQuat: b.quat.clone(),
        toPos: new THREE.Vector3(b.pos.x, rest.height, b.pos.z),
        toQuat: rest.quat,
      };
    });
    this.settleT = 0;
    this.phase = "settle";
  }

  // --- Render loop -------------------------------------------------------- //

  private start(): void {
    if (this.raf) return;
    this.lastT = performance.now();
    const loop = (now: number) => {
      const dt = Math.min((now - this.lastT) / 1000, 0.04);
      this.lastT = now;

      if (this.phase === "roll") {
        this.step(dt);
        this.rollClock += dt;
        if (this.allAsleep() || this.rollClock >= MAX_ROLL) this.beginSettle();
      } else if (this.phase === "settle") {
        this.settleT += dt / SETTLE_DUR;
        const t = easeOutCubic(Math.min(1, this.settleT));
        this.bodies.forEach((b, i) => {
          const s = this.settleData[i];
          b.pos.lerpVectors(s.fromPos, s.toPos, t);
          b.quat.slerpQuaternions(s.fromQuat, s.toQuat, t);
        });
        if (this.settleT >= 1) {
          this.phase = "idle";
          const done = this.resolveRoll;
          this.resolveRoll = null;
          done?.();
        }
      }

      for (const b of this.bodies) b.sync();
      this.renderer.render(this.scene, this.camera);

      if (this.phase === "idle") {
        this.raf = 0;
      } else {
        this.raf = requestAnimationFrame(loop);
      }
    };
    this.raf = requestAnimationFrame(loop);
  }

  private renderOnce(): void {
    this.renderer.render(this.scene, this.camera);
  }

  /** Release everything. The tray is unusable afterwards. */
  dispose(): void {
    if (this.raf) cancelAnimationFrame(this.raf);
    this.raf = 0;
    this.ro.disconnect();
    for (const b of this.bodies) b.dispose();
    this.floor.geometry.dispose();
    (this.floor.material as THREE.Material).dispose();
    for (const w of this.walls) w.geometry.dispose();
    (this.walls[0]?.material as THREE.Material)?.dispose();
    this.renderer.dispose();
  }
}
