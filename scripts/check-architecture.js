const fs = require("fs");
const path = require("path");

console.log("[ArchCheck] Starting monorepo architecture check...");

const requiredDirs = [
  "apps/api",
  "apps/web",
  "apps/mobile",
  "apps/stellar-service",
  "packages/config",
  "packages/types"
];

let failed = false;

requiredDirs.forEach(dir => {
  const fullPath = path.join(__dirname, "..", dir);
  if (!fs.existsSync(fullPath)) {
    console.error(`[ArchCheck] ERROR: Missing required directory: ${dir}`);
    failed = true;
  } else {
    // Check for package.json
    const pkgPath = path.join(fullPath, "package.json");
    if (!fs.existsSync(pkgPath)) {
      console.error(`[ArchCheck] ERROR: Missing package.json in ${dir}`);
      failed = true;
    }
  }
});

if (failed) {
  console.error("[ArchCheck] Architecture validation failed!");
  process.exit(1);
} else {
  console.log("[ArchCheck] Architecture validation successful! All workspaces are structured correctly.");
  process.exit(0);
}
