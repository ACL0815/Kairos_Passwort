import { readFileSync, writeFileSync } from 'node:fs';
import { gunzipSync, gzipSync } from 'node:zlib';
import { fileURLToPath } from 'node:url';
import { resolve, dirname } from 'node:path';

// Source format and morphological substitutions: Princeton WordNet 3.0,
// doc/wninput.5 and doc/morphy.7. No Princeton implementation code is copied.
const root = resolve(dirname(fileURLToPath(import.meta.url)), '../data/languages');
const upstream = resolve(root, 'upstream/wordnet');
const archive = gunzipSync(readFileSync(resolve(upstream, 'WordNet-3.0.tar.gz')));
const files = new Map();
for (let offset = 0; offset + 512 <= archive.length;) {
  const header = archive.subarray(offset, offset + 512);
  const name = header.subarray(0, 100).toString('utf8').replace(/\0.*$/, '');
  if (!name) break;
  const size = Number.parseInt(header.subarray(124, 136).toString('ascii').replace(/\0.*$/, '').trim(), 8);
  if (!Number.isFinite(size)) throw new Error('Invalid WordNet tar member');
  const bytes = archive.subarray(offset + 512, offset + 512 + size);
  if (/\/dict\/(data\.(adj|adv|noun|verb)|(adj|adv|noun|verb)\.exc|lexnames)$/.test(name)) {
    files.set(name.split('/').at(-1), bytes.toString('utf8'));
  }
  offset += 512 + Math.ceil(size / 512) * 512;
}
if (files.size !== 9) throw new Error(`Expected nine WordNet data/exception/lexname files, got ${files.size}`);
const lexnames = new Map(files.get('lexnames').trim().split('\n').map(line => {
  const [number, name] = line.trim().split(/\s+/);
  return [Number(number), name];
}));
const extra = new Map();
for (const line of readFileSync(resolve(root, 'english-extra-evidence.tsv'), 'utf8').trimEnd().split(/\r?\n/)) {
  const [word, date, category, fact, url, ...rest] = line.split('\t');
  if (rest.length || !/^[a-z]{3,9}$/.test(word) || extra.has(word) || !/^\d{4}-\d{2}-\d{2}$/.test(date) || !category || !fact || !url?.startsWith('https://')) {
    throw new Error('Invalid additional English evidence');
  }
  extra.set(word, `Zusatzbeleg ${date} ${category}: ${fact} (${url})`);
}

const rules = {
  noun: [['s', ''], ['ses', 's'], ['xes', 'x'], ['zes', 'z'], ['ches', 'ch'], ['shes', 'sh'], ['men', 'man'], ['ies', 'y']],
  verb: [['s', ''], ['ies', 'y'], ['es', 'e'], ['es', ''], ['ed', 'e'], ['ed', ''], ['ing', 'e'], ['ing', '']],
  adj: [['er', ''], ['est', ''], ['er', 'e'], ['est', 'e']],
  adv: [],
};
const lexicons = new Map();
const exceptions = new Map();
for (const pos of Object.keys(rules).sort()) {
  const entries = new Map();
  for (const line of files.get(`data.${pos}`).split('\n')) {
    if (!/^\d{8} /.test(line)) continue;
    const boundary = line.indexOf('|');
    if (boundary < 0) throw new Error('WordNet record has no gloss');
    const fields = line.slice(0, boundary).trim().split(/\s+/);
    const lexname = lexnames.get(Number(fields[1]));
    if (!lexname) throw new Error('Missing WordNet lexicographer category');
    const count = Number.parseInt(fields[3], 16);
    const gloss = line.slice(boundary + 1).trim();
    for (let i = 0; i < count; i++) {
      const lemma = fields[4 + i * 2].replace(/\((a|p|ip)\)$/, '');
      if (!entries.has(lemma)) entries.set(lemma, []);
      entries.get(lemma).push(`[lex=${fields[1]}:${lexname}] ${pos}:${fields[0]}:${lemma}: ${gloss}`);
    }
  }
  lexicons.set(pos, entries);
  exceptions.set(pos, new Map(files.get(`${pos}.exc`).trim().split('\n').map(line => {
    const [word, ...bases] = line.trim().split(/\s+/);
    return [word, bases];
  })));
}

const words = new Set();
const large = new Set();
for (const source of ['large', 'short1', 'short2']) {
  for (const line of readFileSync(resolve(root, `upstream/eff/${source}.txt`), 'utf8').trim().split('\n')) {
    const word = line.trim().split('\t')[1];
    if (/^[a-z]{3,9}$/.test(word)) {
      words.add(word);
      if (source === 'large') large.add(word);
    }
  }
}
for (const word of readFileSync(resolve(root, 'english-supplement.txt'), 'utf8').trim().split(/\r?\n/)) words.add(word);

let covered = 0;
const rows = [...words].sort().map(word => {
  const senses = new Set();
  for (const [pos, substitutions] of Object.entries(rules)) {
    const bases = new Set([word, ...(exceptions.get(pos).get(word) ?? [])]);
    for (const [ending, replacement] of substitutions) {
      if (word.endsWith(ending)) bases.add(word.slice(0, -ending.length) + replacement);
    }
    for (const base of [...bases].sort()) {
      for (const sense of lexicons.get(pos).get(base) ?? []) senses.add(sense);
    }
  }
  if (senses.size) covered++;
  const source = `${word} | EFF-large=${large.has(word) ? 'yes' : 'no'} | WordNet-3.0 ${senses.size ? [...senses].sort().join(' | ') : 'NO_ENTRY'}${extra.has(word) ? ' | ' + extra.get(word) : ''}`;
  return `${word}\t${source}\n`;
}).join('');
for (const word of extra.keys()) {
  if (!words.has(word)) throw new Error(`Additional evidence is outside the candidate pool: ${word}`);
}
const output = resolve(upstream, 'candidate-senses.tsv.gz');
if (process.argv.slice(2).join(' ') === '--check') {
  if (gunzipSync(readFileSync(output)).toString('utf8') !== rows) throw new Error('English source extract differs');
} else if (process.argv.length === 2) {
  writeFileSync(output, gzipSync(Buffer.from(rows), { level: 9 }));
} else {
  throw new Error('Usage: node prepare-english-senses.mjs [--check]');
}
console.log(`English candidates=${words.size}; WordNet covered=${covered}; no entry=${words.size - covered}; EFF Large=${large.size}`);
