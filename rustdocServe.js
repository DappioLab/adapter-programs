const http = require("http");
const fs = require("fs");
const path = require("path");

const hostname = "0.0.0.0";
const port = 3002;

const server = http.createServer((req, res) => {
  console.log("Request for " + req.url + " by method " + req.method);

  if (req.method == "GET") {
    var fileUrl;
    if (req.url == "/") fileUrl = "/index.html";
    else fileUrl = req.url;
    var filePath = path.resolve("./target/doc" + fileUrl);

    res.statusCode = 200;
    fs.createReadStream(filePath).pipe(res);
  }
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
