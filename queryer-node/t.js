'use strict';

const lib = require('./');

const url = 'https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv';

async function main() {
  const r = await lib.query(`
SELECT location name, total_cases, new_cases, total_deaths, new_deaths
FROM ${url} where new_deaths >= 100 ORDER BY new_cases DESC, new_deaths DESC`);

  console.log(r);
}

main();
