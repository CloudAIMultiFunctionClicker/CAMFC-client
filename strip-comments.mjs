import { readFileSync, writeFileSync, readdirSync, statSync } from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

const extensions = new Set(['.ts', '.tsx', '.js', '.jsx', '.rs', '.vue', '.css', '.scss']);
const ignoreDirs = new Set(['node_modules', 'target', 'dist', '.git', '.vite']);

let fileList = [];

function walk(dir) {
  let entries;
  try {
    entries = readdirSync(dir);
  } catch { return; }
  for (const entry of entries) {
    const full = path.join(dir, entry);
    let stat;
    try { stat = statSync(full); } catch { continue; }
    if (stat.isDirectory()) {
      if (!ignoreDirs.has(entry)) walk(full);
    } else if (stat.isFile()) {
      const ext = path.extname(entry);
      if (extensions.has(ext)) fileList.push(full);
    }
  }
}

walk(__dirname);

console.log(`Found ${fileList.length} files to process.`);

function stripJsComments(code) {
  let result = '';
  let i = 0;
  const len = code.length;

  while (i < len) {
    if (code[i] === '/' && code[i + 1] === '/') {
      while (i < len && code[i] !== '\n') i++;
      continue;
    }
    if (code[i] === '/' && code[i + 1] === '*') {
      i += 2;
      while (i < len - 1 && !(code[i] === '*' && code[i + 1] === '/')) i++;
      i += 2;
      // Add a newline if the block comment was on its own line-ish
      // to avoid joining lines
      continue;
    }
    if (code[i] === '"' || code[i] === "'" || code[i] === '`') {
      const quote = code[i];
      result += quote;
      i++;
      while (i < len && code[i] !== quote) {
        if (code[i] === '\\') {
          result += code[i] + (code[i + 1] || '');
          i += 2;
          continue;
        }
        if (quote === '`' && code[i] === '$' && code[i + 1] === '{') {
          result += '${';
          i += 2;
          let depth = 1;
          let inner = '';
          while (i < len && depth > 0) {
            if (code[i] === '{') depth++;
            else if (code[i] === '}') depth--;
            if (depth > 0) { inner += code[i]; i++; }
          }
          result += stripJsComments(inner) + '}';
          i++;
          continue;
        }
        result += code[i];
        i++;
      }
      if (i < len) { result += code[i]; i++; }
      continue;
    }
    result += code[i];
    i++;
  }
  return result;
}

function stripRustComments(code) {
  let result = '';
  let i = 0;
  const len = code.length;

  while (i < len) {
    if (code[i] === '/' && code[i + 1] === '/') {
      while (i < len && code[i] !== '\n') i++;
      continue;
    }
    if (code[i] === '/' && code[i + 1] === '*') {
      i += 2;
      while (i < len - 1 && !(code[i] === '*' && code[i + 1] === '/')) i++;
      i += 2;
      continue;
    }
    if (code[i] === '"' || code[i] === "'") {
      const quote = code[i];
      result += quote;
      i++;
      while (i < len && code[i] !== quote) {
        if (code[i] === '\\') { result += code[i] + (code[i + 1] || ''); i += 2; continue; }
        result += code[i];
        i++;
      }
      if (i < len) { result += code[i]; i++; }
      continue;
    }
    result += code[i];
    i++;
  }
  return result;
}

function stripCssComments(code) {
  return code.replace(/\/\*[\s\S]*?\*\//g, '');
}

function stripVueComments(code) {
  code = code.replace(/<!--[\s\S]*?-->/g, '');
  const scriptRegex = /<script[\s\S]*?>([\s\S]*?)<\/script>/g;
  code = code.replace(scriptRegex, (match, scriptContent) => {
    return match.replace(scriptContent, stripJsComments(scriptContent));
  });
  const styleRegex = /<style[\s\S]*?>([\s\S]*?)<\/style>/g;
  code = code.replace(styleRegex, (match, styleContent) => {
    return match.replace(styleContent, stripCssComments(styleContent));
  });
  return code;
}

function cleanBlankLines(code) {
  let lines = code.split('\n');
  lines = lines.map(l => l.trimEnd());
  lines = lines.filter(l => l.trim().length > 0 || l.length === 0);
  code = lines.join('\n');
  return code.replace(/\n{3,}/g, '\n\n');
}

for (const fullPath of fileList) {
  const ext = path.extname(fullPath);
  try {
    let code = readFileSync(fullPath, 'utf-8');
    if (ext === '.vue') code = stripVueComments(code);
    else if (ext === '.rs') code = stripRustComments(code);
    else if (ext === '.css' || ext === '.scss') code = stripCssComments(code);
    else code = stripJsComments(code);
    code = cleanBlankLines(code);
    writeFileSync(fullPath, code, 'utf-8');
    console.log(`OK: ${path.relative(__dirname, fullPath)}`);
  } catch (err) {
    console.error(`FAIL: ${path.relative(__dirname, fullPath)} - ${err.message}`);
  }
}

console.log('\nDone!');
