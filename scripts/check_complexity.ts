import * as fs from "node:fs";
import * as path from "node:path";
import ts from "../bindings/node/node_modules/typescript";

const rootDir = path.resolve(__dirname, "..");
const srcDir = path.join(rootDir, "bindings/node/src");

interface FunctionComplexity {
  name: string;
  startLine: number;
  endLine: number;
  lines: number;
  complexity: number;
}

interface FileReport {
  filePath: string;
  relPath: string;
  totalLines: number;
  codeLines: number;
  commentLines: number;
  blankLines: number;
  functions: FunctionComplexity[];
  exportedSymbols: string[];
  imports: string[];
}

function calculateCyclomaticComplexity(node: ts.Node): number {
  let complexity = 1;

  function visit(n: ts.Node) {
    switch (n.kind) {
      case ts.SyntaxKind.IfStatement:
      case ts.SyntaxKind.ConditionalExpression:
      case ts.SyntaxKind.ForStatement:
      case ts.SyntaxKind.ForInStatement:
      case ts.SyntaxKind.ForOfStatement:
      case ts.SyntaxKind.WhileStatement:
      case ts.SyntaxKind.DoWhileStatement:
      case ts.SyntaxKind.CaseClause:
      case ts.SyntaxKind.CatchClause:
        complexity++;
        break;
      case ts.SyntaxKind.BinaryExpression: {
        const binExpr = n as ts.BinaryExpression;
        const op = binExpr.operatorToken.kind;
        if (
          op === ts.SyntaxKind.AmpersandAmpersandToken ||
          op === ts.SyntaxKind.BarBarToken ||
          op === ts.SyntaxKind.QuestionQuestionToken
        ) {
          complexity++;
        }
        break;
      }
      default:
        break;
    }
    ts.forEachChild(n, visit);
  }

  ts.forEachChild(node, visit);
  return complexity;
}

function countLines(lines: string[]) {
  let blankLines = 0;
  let commentLines = 0;
  let inBlockComment = false;

  for (const line of lines) {
    const trimmed = line.trim();
    if (!trimmed) {
      blankLines++;
      continue;
    }
    if (inBlockComment) {
      commentLines++;
      if (trimmed.includes("*/")) {
        inBlockComment = false;
      }
      continue;
    }
    if (trimmed.startsWith("/*")) {
      commentLines++;
      if (!trimmed.includes("*/")) {
        inBlockComment = true;
      }
      continue;
    }
    if (trimmed.startsWith("//")) {
      commentLines++;
    }
  }

  return {
    blankLines,
    commentLines,
    codeLines: lines.length - blankLines - commentLines,
  };
}

function getFunctionName(node: ts.Node, sourceFile: ts.SourceFile): string {
  if (ts.isFunctionDeclaration(node) && node.name) {
    return node.name.text;
  }
  if (ts.isMethodDeclaration(node) && node.name) {
    return node.name.getText(sourceFile);
  }
  if (ts.isConstructorDeclaration(node)) {
    return "constructor";
  }
  if (ts.isArrowFunction(node) || ts.isFunctionExpression(node)) {
    const parent = node.parent;
    if (parent && ts.isVariableDeclaration(parent) && parent.name) {
      return parent.name.getText(sourceFile);
    }
    if (parent && ts.isPropertyAssignment(parent) && parent.name) {
      return parent.name.getText(sourceFile);
    }
    return "<anonymous>";
  }
  return "<unknown>";
}

function extractExportedSymbols(node: ts.Node, symbols: string[]) {
  if (
    ts.isFunctionDeclaration(node) ||
    ts.isClassDeclaration(node) ||
    ts.isInterfaceDeclaration(node) ||
    ts.isTypeAliasDeclaration(node)
  ) {
    const isExported = node.modifiers?.some((m) => m.kind === ts.SyntaxKind.ExportKeyword);
    if (isExported && node.name) {
      let kind = "type";
      if (ts.isFunctionDeclaration(node)) kind = "fn";
      else if (ts.isClassDeclaration(node)) kind = "class";
      else if (ts.isInterfaceDeclaration(node)) kind = "interface";
      symbols.push(`${kind} ${node.name.text}`);
    }
  } else if (
    ts.isExportDeclaration(node) &&
    node.exportClause &&
    ts.isNamedExports(node.exportClause)
  ) {
    for (const el of node.exportClause.elements) {
      symbols.push(el.name.text);
    }
  }
}

function extractFunctionInfo(
  node: ts.Node,
  sourceFile: ts.SourceFile,
  functions: FunctionComplexity[],
) {
  const isFunction =
    ts.isFunctionDeclaration(node) ||
    ts.isMethodDeclaration(node) ||
    ts.isConstructorDeclaration(node) ||
    ts.isArrowFunction(node) ||
    ts.isFunctionExpression(node);

  if (!isFunction) return;

  const name = getFunctionName(node, sourceFile);
  const start = sourceFile.getLineAndCharacterOfPosition(node.getStart(sourceFile)).line + 1;
  const end = sourceFile.getLineAndCharacterOfPosition(node.getEnd()).line + 1;
  const funcLines = end - start + 1;

  if (name !== "<anonymous>" || funcLines > 5) {
    functions.push({
      name,
      startLine: start,
      endLine: end,
      lines: funcLines,
      complexity: calculateCyclomaticComplexity(node),
    });
  }
}

function analyzeFile(filePath: string): FileReport {
  const content = fs.readFileSync(filePath, "utf8");
  const relPath = path.relative(srcDir, filePath);
  const lines = content.split("\n");
  const { blankLines, commentLines, codeLines } = countLines(lines);

  const sourceFile = ts.createSourceFile(filePath, content, ts.ScriptTarget.Latest, true);

  const functions: FunctionComplexity[] = [];
  const exportedSymbols: string[] = [];
  const imports: string[] = [];

  function visit(node: ts.Node) {
    if (ts.isImportDeclaration(node)) {
      imports.push(node.moduleSpecifier.getText(sourceFile).replace(/['"]/g, ""));
    }
    extractExportedSymbols(node, exportedSymbols);
    extractFunctionInfo(node, sourceFile, functions);
    ts.forEachChild(node, visit);
  }

  ts.forEachChild(sourceFile, visit);

  return {
    filePath,
    relPath,
    totalLines: lines.length,
    codeLines,
    commentLines,
    blankLines,
    functions,
    exportedSymbols: Array.from(new Set(exportedSymbols)),
    imports: Array.from(new Set(imports)),
  };
}

function getAllTsFiles(dir: string): string[] {
  const entries = fs.readdirSync(dir, { withFileTypes: true });
  const files: string[] = [];
  for (const entry of entries) {
    const fullPath = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      files.push(...getAllTsFiles(fullPath));
    } else if (entry.isFile() && entry.name.endsWith(".ts")) {
      files.push(fullPath);
    }
  }
  return files.sort();
}

console.log("===============================================================================");
console.log("             multilingual-katakana Code Complexity & Structure Report           ");
console.log("===============================================================================\n");

const tsFiles = getAllTsFiles(srcDir);
const reports = tsFiles.map(analyzeFile);

let totalTotalLines = 0;
let totalCodeLines = 0;
let totalFunctions = 0;
let maxComplexity = 0;
let maxComplexityFunc = "";

console.log(
  `| ${"File".padEnd(30)} | ${"Total".padStart(6)} | ${"Code".padStart(6)} | ${"Comments".padStart(8)} | ${"Funcs".padStart(5)} | ${"Max CC".padStart(6)} |`,
);
console.log(
  `|${"-".repeat(32)}|${"-".repeat(8)}|${"-".repeat(8)}|${"-".repeat(10)}|${"-".repeat(7)}|${"-".repeat(8)}|`,
);

for (const rep of reports) {
  totalTotalLines += rep.totalLines;
  totalCodeLines += rep.codeLines;
  totalFunctions += rep.functions.length;

  const fileMaxCC = rep.functions.reduce((max, f) => (f.complexity > max ? f.complexity : max), 0);
  if (fileMaxCC > maxComplexity) {
    maxComplexity = fileMaxCC;
    const topFunc = rep.functions.find((f) => f.complexity === fileMaxCC);
    maxComplexityFunc = `${rep.relPath} > ${topFunc?.name || "unknown"}`;
  }

  console.log(
    `| ${rep.relPath.padEnd(30)} | ${String(rep.totalLines).padStart(6)} | ${String(rep.codeLines).padStart(6)} | ${String(rep.commentLines).padStart(8)} | ${String(rep.functions.length).padStart(5)} | ${String(fileMaxCC).padStart(6)} |`,
  );
}

console.log(
  `|${"-".repeat(32)}|${"-".repeat(8)}|${"-".repeat(8)}|${"-".repeat(10)}|${"-".repeat(7)}|${"-".repeat(8)}|`,
);
console.log(
  `| ${"TOTAL".padEnd(30)} | ${String(totalTotalLines).padStart(6)} | ${String(totalCodeLines).padStart(6)} | ${String(totalTotalLines - totalCodeLines).padStart(8)} | ${String(totalFunctions).padStart(5)} | ${String(maxComplexity).padStart(6)} |`,
);

console.log("\n--- Top Functions by Cyclomatic Complexity ---");
const allFunctions: (FunctionComplexity & { file: string })[] = [];
for (const r of reports) {
  for (const f of r.functions) {
    allFunctions.push({ ...f, file: r.relPath });
  }
}
allFunctions.sort((a, b) => b.complexity - a.complexity);

const threshold = 15;

for (const f of allFunctions.slice(0, 10)) {
  const status = f.complexity > threshold ? `⚠️ EXCEEDS (${threshold})` : `✓ OK (<=${threshold})`;
  console.log(
    `  • [CC: ${String(f.complexity).padStart(2)}] ${f.file} -> ${f.name} (L${f.startLine}-${f.endLine}, ${f.lines} lines) [${status}]`,
  );
}

console.log("\n--- Module Structure Summary ---");
for (const rep of reports) {
  console.log(`\n📄 ${rep.relPath}`);
  if (rep.imports.length > 0) {
    console.log(`   Imports: ${rep.imports.join(", ")}`);
  }
  if (rep.exportedSymbols.length > 0) {
    console.log(`   Exports: ${rep.exportedSymbols.join(", ")}`);
  }
  if (rep.functions.length > 0) {
    const fnList = rep.functions
      .map((f) => `${f.name} (CC:${f.complexity}, ${f.lines}L)`)
      .join(", ");
    console.log(`   Functions: ${fnList}`);
  }
}

console.log("\n===============================================================================");
console.log("                         Complexity Audit Conclusion                            ");
console.log("===============================================================================");
const violations = allFunctions.filter((f) => f.complexity > threshold);

if (violations.length === 0) {
  console.log(
    `✅ PASS: All ${allFunctions.length} analyzed functions are within complexity threshold <= ${threshold}.`,
  );
  console.log(`   Max complexity function: ${maxComplexityFunc} (CC: ${maxComplexity})`);
  console.log(
    `   Biome rule 'complexity.noExcessiveCognitiveComplexity' is configured with maxAllowedComplexity: ${threshold}.`,
  );
} else {
  console.log(
    `❌ FAIL: ${violations.length} function(s) exceed complexity threshold ${threshold}:`,
  );
  for (const v of violations) {
    console.log(`   - ${v.file} -> ${v.name}: complexity ${v.complexity}`);
  }
  process.exit(1);
}
