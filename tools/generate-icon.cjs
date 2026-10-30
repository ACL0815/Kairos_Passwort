// Original Kairos artwork: navy tile, neon ring and geometric cyan K.
// Rebuild with: node tools/generate-icon.cjs (Node built-ins only).
const fs = require('node:fs');
const path = require('node:path');
const zlib = require('node:zlib');
const out = path.join(__dirname, '../apps/kairos/assets');
function crc(bytes) {
  let value = 0xffffffff;
  for (const byte of bytes) {
    value ^= byte;
    for (let bit = 0; bit < 8; bit++) value = (value >>> 1) ^ ((value & 1) ? 0xedb88320 : 0);
  }
  return (value ^ 0xffffffff) >>> 0;
}
function chunk(type, data) {
  const body = Buffer.concat([Buffer.from(type), data]);
  const head = Buffer.alloc(4), tail = Buffer.alloc(4);
  head.writeUInt32BE(data.length); tail.writeUInt32BE(crc(body));
  return Buffer.concat([head, body, tail]);
}
function distance(x, y, a, b, c, d) {
  const t = Math.max(0, Math.min(1, ((x-a)*(c-a)+(y-b)*(d-b))/((c-a)**2+(d-b)**2)));
  return Math.hypot(x-a-t*(c-a), y-b-t*(d-b));
}
function pixels(size) {
  const rgba = Buffer.alloc(size * size * 4);
  for (let y=0; y<size; y++) for (let x=0; x<size; x++) {
    const sum = [0,0,0,0];
    for (let sy=0; sy<4; sy++) for (let sx=0; sx<4; sx++) {
      const px=(x+(sx+.5)/4)/size, py=(y+(sy+.5)/4)/size;
      const corner=Math.hypot(Math.max(Math.abs(px-.5)-.31,0), Math.max(Math.abs(py-.5)-.31,0));
      let color = corner <= .16 ? [10,16,48,255] : [0,0,0,0];
      const radius=Math.hypot(px-.5,py-.5);
      if (radius >= .345 && radius <= .395) {
        const t=(Math.atan2(py-.5,px-.5)+Math.PI)/(2*Math.PI);
        const a=t<.5?[139,92,246]:[236,72,153], b=t<.5?[236,72,153]:[251,146,60], blend=(t%0.5)*2;
        color=[...a.map((v,i)=>Math.round(v+(b[i]-v)*blend)),255];
      }
      if (Math.min(distance(px,py,.40,.29,.40,.71),distance(px,py,.40,.51,.63,.29),distance(px,py,.43,.48,.64,.71)) < .035) color=[34,211,238,255];
      color.forEach((v,i)=>sum[i]+=v);
    }
    sum.forEach((v,i)=>rgba[(y*size+x)*4+i]=Math.round(v/16));
  }
  return rgba;
}
function png(size, rgba) {
  const ihdr=Buffer.alloc(13); ihdr.writeUInt32BE(size); ihdr.writeUInt32BE(size,4); ihdr[8]=8; ihdr[9]=6;
  const rows=Buffer.alloc(size*(size*4+1));
  for(let y=0;y<size;y++) rgba.copy(rows,y*(size*4+1)+1,y*size*4,(y+1)*size*4);
  return Buffer.concat([Buffer.from([137,80,78,71,13,10,26,10]),chunk('IHDR',ihdr),chunk('IDAT',zlib.deflateSync(rows,{level:9})),chunk('IEND',Buffer.alloc(0))]);
}
const sizes=[16,24,32,48,64,128,256], images=sizes.map(n=>png(n,pixels(n)));
const header=Buffer.alloc(6+16*sizes.length); header.writeUInt16LE(1,2); header.writeUInt16LE(sizes.length,4);
let offset=header.length;
sizes.forEach((n,i)=>{ const p=6+i*16; header[p]=header[p+1]=n===256?0:n; header.writeUInt16LE(1,p+4); header.writeUInt16LE(32,p+6); header.writeUInt32LE(images[i].length,p+8); header.writeUInt32LE(offset,p+12); offset+=images[i].length; });
fs.writeFileSync(path.join(out,'kairos.ico'),Buffer.concat([header,...images]));
fs.writeFileSync(path.join(out,'kairos-64.rgba'),pixels(64));
fs.writeFileSync(path.join(__dirname,'../assets/icon.svg'),'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64"><defs><linearGradient id="r"><stop stop-color="#8b5cf6"/><stop offset=".5" stop-color="#ec4899"/><stop offset="1" stop-color="#fb923c"/></linearGradient></defs><rect x="2" y="2" width="60" height="60" rx="10" fill="#0a1030"/><circle cx="32" cy="32" r="24" fill="none" stroke="url(#r)" stroke-width="3"/><path d="M26 19v26m0-12 15-14M28 31l13 14" fill="none" stroke="#22d3ee" stroke-width="4" stroke-linecap="round"/></svg>\n');
console.log('Generated original Kairos ICO (16,24,32,48,64,128,256), RGBA and SVG.');
