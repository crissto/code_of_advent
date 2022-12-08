const { readFileSync } = require("fs");
const { resolve } = require("path");
const process = require("process");

const input = readFileSync("./input.txt").toString();
const lines = input.split("\n");

process.chdir("/");
let path = "/";
const sizes = {};

const handleLine = (line) => {
  const parts = line.split(" ");

  switch (parts[0]) {
    case "$":
      return handleCommand(parts);
    case "dir":
      return;
    default:
      return handleFile(parts);
  }
};

const sanitizePath = (path) =>
  path
    .split("/")
    .filter((c) => c)
    .join(".");

const handleCommand = (parts) => {
  if (parts[1] === "cd") {
    path = resolve(path, parts[2]);
  }
};

const handleFile = (parts) => {
  if (!sizes[sanitizePath(path)]) {
    sizes[sanitizePath(path)] = 0;
  }

  sizes[sanitizePath(path)] += +parts[0];
};

lines.forEach(handleLine);

Object.entries(sizes).forEach(([line, size]) => {
  const parts = line.split(".");

  for (let i = 1; i < parts.length; i++) {
    if (!sizes[parts.slice(0, i).join(".")]) {
      sizes[parts.slice(0, i).join(".")] = 0;
    }

    sizes[parts.slice(0, i).join(".")] += +size;
  }
});

const smallDirs = Object.values(sizes)
  .filter((size) => size < 100000)
  .reduce((sum, curr) => sum + curr, 0);

console.log({ smallDirs });

const topDirsSum = Object.entries(sizes)
  .filter(([path]) => path.indexOf(".") < 0)
  .reduce((sum, [path, value]) => sum + value, 0);

const freeSpace = 70000000 - topDirsSum;
const spaceNeeded = 30000000 - freeSpace;

const bigDirs = Object.entries(sizes)
  .filter(([path, value]) => value > spaceNeeded)
  .sort(([pathA, valueA], [pathB, valueB]) => {
    return valueA - valueB;
  });

const spaceToBeFreed = bigDirs[0][1];

console.log({ spaceToBeFreed });
