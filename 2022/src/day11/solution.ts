import { readFileSync } from "fs";
import { join, resolve } from "path";

interface Monkey {
  id: number;
  items: number[];
  operation: (old: number) => number;
  divisor: number;
  iftrue: number;
  iffalse: number;
  items_passed: number;
}

const data = readFileSync(join(resolve(), "./input.txt"), "utf8").split("\n\n");

function getMonkeys(data: string[]): Monkey[] {
  return data.map((line: string) => {
    const lines = line.split("\n");
    return {
      id: Number(lines[0].split("Monkey ")[1].split(":")[0]),
      items: lines[1]
        .split("Starting items: ")[1]
        .split(",")
        .map((i) => Number(i)),
      operation: (old) => {
        let op = lines[2].split("Operation: new = ")[1];
        return eval(op);
      },
      divisor: Number(lines[3].split("Test: divisible by ")[1]),
      iftrue: Number(lines[4].split("If true: throw to monkey ")[1]),
      iffalse: Number(lines[5].split("If false: throw to monkey ")[1]),
      items_passed: 0,
    } as Monkey;
  });
}

function getSol1() {
  const monkeys = getMonkeys(data);
  for (let round = 0; round < 20; round++) {
    for (const monkey of monkeys) {
      while (monkey.items.length > 0) {
        let item = monkey.items.shift();

        const multiplied = Math.floor(monkey.operation(item!) / 3);
        const target =
          multiplied % monkey.divisor === 0 ? monkey.iftrue : monkey.iffalse;
        monkeys[target].items.push(multiplied);
        monkey.items_passed++;
      }
    }
  }

  const sol = monkeys
    .map(({ items_passed }) => items_passed)
    .sort((a, b) => b - a);

  return sol[0] * sol[1];
}

function getSol2() {
  const monkeys = getMonkeys(data);
  let denominator = monkeys
    .map(({ divisor }) => divisor)
    .reduce((a, b) => a * b);

  for (let round = 0; round < 10000; round++) {
    for (const monkey of monkeys) {
      while (monkey.items.length > 0) {
        let item = monkey.items.shift();

        const multiplied = monkey.operation(item!) % denominator;
        const target =
          multiplied % monkey.divisor === 0 ? monkey.iftrue : monkey.iffalse;
        monkeys[target].items.push(multiplied);
        monkey.items_passed++;
      }
    }
  }

  const sol = monkeys
    .map(({ items_passed }) => items_passed)
    .sort((a, b) => b - a);

  return sol[0] * sol[1];
}

console.log(`Sol 1 ${getSol1()}`);
console.log(`Sol 2 ${getSol2()}`);
