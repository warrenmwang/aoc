const fs = require("node:fs");

fs.readFile("./7.input", "utf8", (err, data) => {
  if (err) {
    console.error(err);
    return;
  }
  partOne(data);
});

function partOne(data) {
  const lines = data.split("\n");
  for (let line of lines) {
    const parts = line.split(" ");
    console.log(parts.length)
  }
}

// honestly using javascript for puzzles like this is so disgusting lol.
