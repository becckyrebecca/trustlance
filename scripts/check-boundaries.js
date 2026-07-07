const fs = require("fs");
const path = require("path");

console.log("[BoundaryCheck] Starting boundary violation check...");

const appSrcDirs = [
  "apps/web/src",
  "apps/mobile/src"
];

let failed = false;

// Scan directory recursively
function scanDir(dir, fileList = []) {
  const files = fs.readdirSync(dir);
  files.forEach(file => {
    const filePath = path.join(dir, file);
    const stat = fs.statSync(filePath);
    if (stat.isDirectory()) {
      scanDir(filePath, fileList);
    } else if (filePath.endsWith(".ts") || filePath.endsWith(".tsx") || filePath.endsWith(".js") || filePath.endsWith(".jsx")) {
      fileList.push(filePath);
    }
  });
  return fileList;
}

appSrcDirs.forEach(srcDir => {
  const fullPath = path.join(__dirname, "..", srcDir);
  if (!fs.existsSync(fullPath)) return;

  const files = scanDir(fullPath);
  files.forEach(file => {
    const content = fs.readFileSync(file, "utf8");
    
    // Check for illegal relative imports pointing outside the app boundary
    // e.g. import from "../api" or "../../stellar-service"
    const relativeViolations = /import\s+.*\s+from\s+["']\.\.\/\.\.\/(api|stellar-service)/g;
    const directViolations = /import\s+.*\s+from\s+["'](apps\/api|apps\/stellar-service)/g;
    
    if (relativeViolations.test(content) || directViolations.test(content)) {
      console.error(`[BoundaryCheck] ERROR: Boundary violation detected in: ${path.relative(path.join(__dirname, ".."), file)}`);
      console.error("  -> UI packages should not import directly from API or Stellar backend services.");
      console.error("  -> Use shared types (@lumina/types) and shared configs (@lumina/config) instead.");
      failed = true;
    }
  });
});

if (failed) {
  console.error("[BoundaryCheck] Boundary validation failed!");
  process.exit(1);
} else {
  console.log("[BoundaryCheck] Boundary validation successful! No workspace boundaries were violated.");
  process.exit(0);
}
