const fs = require('fs');

function lineValue(line) {
  const [lhsNumbers, rhsNumbers] = ((line.split(":"))[1])
    .split("|")
    .map(
      str => str
        .trim()
        .replace(/\s+/g, ' ')
        .split(' ')
        .map(n => parseInt(n, 10))
    )
  return rhsNumbers.reduce((sum, value, _) => sum + lhsNumbers.includes(value), 0)
}

const generateIndexes = length => [...Array(length).keys()];

const lines = fs.readFileSync('input.txt', 'utf-8').split('\n');
const cardCount = Array(lines.length).fill(1);
cardCount.forEach(
  (count, lineNumber, arr) =>
    generateIndexes(lineValue(lines[lineNumber])).forEach(
      (_1, index, _2) => arr[lineNumber + index + 1] += count));
console.log(cardCount.reduce((sum, value, _) => sum + value, 0));
