const http = require('http');

const hostname = '0.0.0.0';
const port = 8000;

const server = http.createServer((req, res) => {
  if (req.url=="/hello"){
    res.statusCode = 200;
    res.setHeader('Content-Type', 'application/json');
    res.end('{"data":"world"}');
  }
  else{
    res.statusCode = 204;
    res.setHeader('Content-Type', 'text/plain');
    res.end();
  }
});


server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});