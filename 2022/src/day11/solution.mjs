import { promises as fs } from "fs";
import path from "path";

let data = (await fs.readFile(path.join(path.resolve(), "./input.txt"), "utf8"))
  // let data = (await fs.readFile('./test11.txt', 'utf8'))
  .split("\n\n")
  .map((m) => {
    const re =
      /Monkey (?<index>\d+):\n  Starting items: (?<items>[\d, ]+)\n  Operation: (?<operation>[^\n]+)\n  Test: divisible by (?<divisor>\d+)\n    If true: throw to monkey (?<iftrue>\d+)\n    If false: throw to monkey (?<iffalse>\d+)/gm;
    const p = re.exec(m);
    return {
      items: p.groups.items.split(",").map((d) => parseInt(d)),
      operation: (old) => {
        let vnew;
        eval(`v${p.groups.operation}`);
        return vnew;
      },
      divisor: parseInt(p.groups.divisor),
      iftrue: parseInt(p.groups.iftrue),
      iffalse: parseInt(p.groups.iffalse),
      monkeyBusiness: 0,
    };
  });

for (let i = 0; i < 20; i++) {
  for (let j = 0; j < data.length; j++) {
    while (data[j].items.length > 0) {
      const item = data[j].items.shift();
      const newValue = Math.floor(data[j].operation(item) / 3);
      const target =
        newValue % data[j].divisor === 0 ? data[j].iftrue : data[j].iffalse;
      data[target].items.push(newValue);
      data[j].monkeyBusiness++;
    }
  }
  console.log(`Round ${i} ${data.map((x) => x.monkeyBusiness)}`);
}

const monkeyBusinesses = data
  .map((d) => d.monkeyBusiness)
  .sort((l, r) => r - l);

console.log(monkeyBusinesses[0] * monkeyBusinesses[1]);
