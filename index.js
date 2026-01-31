#!/usr/bin/env node
import fs from "fs";
import path from "path";

const args = process.argv.slice(2);
const command = args[0];
const name = args[1];

if (command !== "init" || !name) {
  console.log("Usage: machina init <program-name>");
  process.exit(1);
}

const targetDir = path.join(process.cwd(), name);
const templateDir = path.join(
  new URL(".", import.meta.url).pathname,
  "templates/anchor-program"
);

fs.mkdirSync(`${targetDir}/programs/${name}/src`, { recursive: true });

const template = fs
  .readFileSync(`${templateDir}/lib.rs`, "utf8")
  .replace(/{{NAME}}/g, name);

fs.writeFileSync(
  `${targetDir}/programs/${name}/src/lib.rs`,
  template
);

console.log(`⚙️ MACHINA scaffold created: ${name}`);
console.log(`➡️ cd ${name} && anchor build`);
