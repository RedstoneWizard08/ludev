import * as fs from "fs";
import * as ludic from "..";

ludic.getFabricVersions().then((v) => {
    console.log("# of Fabric versions: " + v.length);
    fs.writeFileSync("fabric-versions.json", JSON.stringify(v, null, 4));
});

ludic.getForgeVersions().then((v) => {
    console.log("# of Forge versions for MC 1.19: " + v["1.19"].length);
    fs.writeFileSync("forge-versions.json", JSON.stringify(v, null, 4));
});
