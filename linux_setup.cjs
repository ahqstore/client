const { readFileSync, writeFileSync } = require("node:fs");
const { version } = require("./package.json");

const data = readFileSync("./install.sh").toString();

const result = data
  .replace("%%tag%%", process.env.TAG)
  .replace("%%deb%%", `AHQ.Store_${version}_amd64.deb`)
  .replace("%%rpm%%", `AHQ.Store-${version}-1.x86_64.rpm`);

writeFileSync("./install.sh", result);
