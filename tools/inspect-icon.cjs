// Verify the real PE resource directory without loading or starting the EXE.
// node tools/inspect-icon.cjs <kairos.exe> [report.json]
const fs = require('node:fs');
const crypto = require('node:crypto');
const path = require('node:path');
const exe = process.argv[2];
if (!exe) throw Error('EXE path required');
const bytes = fs.readFileSync(exe);
const hash = data => crypto.createHash('sha256').update(data).digest('hex');
const pe = bytes.readUInt32LE(0x3c);
if (bytes.toString('ascii', pe, pe+4) !== 'PE\0\0') throw Error('not a PE file');
const count = bytes.readUInt16LE(pe+6), optional = pe+24;
const magic = bytes.readUInt16LE(optional);
if (magic !== 0x20b) throw Error('expected PE32+');
const table = optional + bytes.readUInt16LE(pe+20);
const sections = Array.from({length: count}, (_, i) => {
  const p = table+i*40;
  return {rva:bytes.readUInt32LE(p+12), size:Math.max(bytes.readUInt32LE(p+8),bytes.readUInt32LE(p+16)), raw:bytes.readUInt32LE(p+20)};
});
function offset(rva) {
  const s = sections.find(s=>rva>=s.rva && rva<s.rva+s.size);
  if (!s) throw Error('RVA outside sections');
  return s.raw+rva-s.rva;
}
const base = offset(bytes.readUInt32LE(optional+112+16));
function entries(relative=0) {
  const p=base+relative, n=bytes.readUInt16LE(p+12)+bytes.readUInt16LE(p+14);
  return Array.from({length:n},(_,i)=>{
    const q=p+16+i*8, id=bytes.readUInt32LE(q), value=bytes.readUInt32LE(q+4);
    return {id, dir:!!(value & 0x80000000), value:value & 0x7fffffff};
  });
}
function resource(type,id) {
  const t=entries().find(e=>e.id===type);
  if(!t?.dir) throw Error('missing resource type '+type);
  const item=entries(t.value).find(e=>id===undefined||e.id===id);
  if(!item?.dir) throw Error('missing resource item '+id);
  const lang=entries(item.value)[0];
  if(!lang||lang.dir) throw Error('invalid language entry');
  const p=base+lang.value, start=offset(bytes.readUInt32LE(p)), size=bytes.readUInt32LE(p+4);
  return bytes.subarray(start,start+size);
}
const group=resource(14), images=group.readUInt16LE(4);
const ico=fs.readFileSync(path.join(__dirname,'../apps/kairos/assets/kairos.ico'));
if(images!==ico.readUInt16LE(4)) throw Error('icon size count differs');
const icons=[];
for(let i=0;i<images;i++) {
  const g=6+i*14, c=6+i*16;
  const width=group[g]||256, height=group[g+1]||256, id=group.readUInt16LE(g+12);
  const actual=resource(3,id), expected=ico.subarray(ico.readUInt32LE(c+12),ico.readUInt32LE(c+12)+ico.readUInt32LE(c+8));
  if(!actual.equals(expected)) throw Error('PE icon differs from original artwork at '+width);
  icons.push({id,width,height,bytes:actual.length,sha256:hash(actual)});
}
const report={exe:path.resolve(exe),bytes:bytes.length,sha256:hash(bytes),resourceTypes:entries().map(e=>e.id),icons};
const json=JSON.stringify(report,null,2)+'\n';
if(process.argv[3]) fs.writeFileSync(process.argv[3],json);
process.stdout.write(json);
