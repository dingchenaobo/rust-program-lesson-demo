const fs = require('fs');
const path = require('path');
const https = require('https');
const TurndownService = require('turndown');

const turndownService = new TurndownService();
const cwd = process.cwd();

https.get('https://www.rust-lang.org/', res => {
  const responseBufferList = [];

  res.on('data', chunk => responseBufferList.push(chunk));

  res.on('end', () => {
    const responseTxt = Buffer.concat(responseBufferList).toString();
    const markdown = turndownService.turndown(responseTxt);
    fs.writeFileSync(path.join(cwd, 'rust-lang.js.md'), markdown, {
      encoding: 'utf-8',
    })
  });
});
