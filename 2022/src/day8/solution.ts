import { readFileSync } from "fs";
import path from "path";

const file = readFileSync(path.join(__dirname, "./input.txt"), "utf-8");
const matrix = file
  .split("\n")
  .map((row) => row.split("").map((x) => Number(x)));

const getColAsRow = (matrix: number[][], col: number) => {
  return matrix.map((row) => row[col]);
};

const isVisible = (matrix: number[][], x: number, y: number) => {
  const row = matrix[y];
  const col = getColAsRow(matrix, x);
  const itemHeight = matrix[y][x];
  // console.log({ itemHeight, x, y });

  const coveredFromLeft = row.slice(0, x).some((item) => item >= itemHeight);
  const coveredFromRight = row
    .slice(x + 1, row.length)
    .some((item) => item >= itemHeight);
  const coveredFromTop = col.slice(0, y).some((item) => item >= itemHeight);
  const coveredFromBottom = col
    .slice(y + 1, col.length)
    .some((item) => item >= itemHeight);

  return [
    coveredFromTop,
    coveredFromRight,
    coveredFromBottom,
    coveredFromLeft,
  ].every((x) => x);
};

const getScenicScore = (matrix: number[][], x: number, y: number) => {
  const row = matrix[y];
  const col = getColAsRow(matrix, x);
  const itemHeight = matrix[y][x];

  if (x === 0 || y === 0 || y === col.length || x === row.length) {
    console.log("edge");
    return 0;
  }
  const indexCoveredLeft = row
    .slice(0, x)
    .reverse()
    .findIndex((x) => x >= itemHeight);
  const indexCoveredRight = row
    .slice(x + 1, row.length)
    .findIndex((x) => x >= itemHeight);

  const indexCoveredTop = col
    .slice(0, y)
    .reverse()
    .findIndex((x) => x >= itemHeight);
  const indexCoveredBottom = col
    .slice(y + 1, col.length)
    .findIndex((x) => x >= itemHeight);

  return (
    (indexCoveredTop === -1 ? y : indexCoveredTop + 1) *
    (indexCoveredRight === -1 ? row.length - x - 1 : indexCoveredRight + 1) *
    (indexCoveredBottom === -1 ? col.length - y - 1 : indexCoveredBottom + 1) *
    (indexCoveredLeft === -1 ? x : indexCoveredLeft + 1)
  );
};

let totalCount = 0;
let scenicScores: number[] = [];

for (let rowIndex = 0; rowIndex < matrix.length; rowIndex++) {
  for (let colIndex = 0; colIndex < matrix[rowIndex].length; colIndex++) {
    const visible = isVisible(matrix, colIndex, rowIndex) ? 0 : 1;
    totalCount += visible;

    const scenicScore = getScenicScore(matrix, colIndex, rowIndex);
    scenicScores.push(scenicScore);
  }
}


console.log(Math.max(...scenicScores));
