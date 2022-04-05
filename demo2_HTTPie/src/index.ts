import qs from 'qs';
import chalk from 'chalk';
import fetch from 'node-fetch';
import readline from 'readline';
import type { RequestInfo, RequestInit } from 'node-fetch';

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
});

const request = async (url: RequestInfo, options?: RequestInit) => {
  return fetch(url, options).then(async response => {
    const body = await response.json();
    const headers = response.headers;
    Array.from(headers.keys()).forEach(k => {
      console.log(
        chalk.blue(k), '：', headers.get(k),
      );
    });
    console.log('\n');
    console.log(body);
  });
}

/**
 * GET https://www.httpbin.org/get a=1 b=2
 * POST https://www.httpbin.org/post a=1 b=2
 */
rl.question('> ', async answer => {
  const [_method, _url, ...args] = answer.replace(/\s+/g, '|').split('|');
  let url = _url;
  const method = _method.toUpperCase();
  const options = {
    method,
    headers: {'Content-Type': 'application/json'},
  };

  const params = Object.fromEntries(args.map(argStr => argStr.split('=')));

  if (method === 'POST') {
    Object.assign(options, {
      body: JSON.stringify(params),
    });
  } else {
    url += '?' + qs.stringify(params);
  }

  await request(url, options);
  rl.close();
});
