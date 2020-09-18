const fs = require('fs').promises;
const child_process = require('child_process')

const pipeAsyncFunctions = (...fns) => arg => fns.reduce((p, f) => p.then(f), Promise.resolve(arg));

const appendExample = (tagName, filePath) => readme => {
    return fs.readFile(filePath, 'utf-8').then(data => {
        return readme.replace(tagName, data)
    }).catch(() => readme)
}
const appendExampleResult = (tagName, exampleName) => readme => {

    return new Promise((resolve, reject) => {
        child_process.exec(`cargo test --example ${exampleName} -- --nocapture`, (error, stout, sterr) => {
          if (error) {
              reject(error)
          } else if (stout) {
              resolve(readme.replace(tagName, stout))
          } else if (sterr) {
              resolve(readme.replace(tagName, sterr))
          } else {
              resolve(readme)
          }
        })
    })
}

(pipeAsyncFunctions(
    () => console.log("building readme"),
    () => fs.readFile('./README.template.md', 'utf-8'),

    appendExample('//###SIMPLE###//', './examples/simple.rs'),
    appendExampleResult('//###SIMPLE-RESULT###//', 'simple'),

    appendExample('//###NESTED###//', './examples/nested-suites.rs'),
    appendExampleResult('//###NESTED-RESULT###//', 'nested-suites'),

    appendExample('//###FAILURE###//', './examples/failure.rs'),
    appendExampleResult('//###FAILURE-RESULT###//', 'failure'),

    appendExample('//###HOOKS###//', './examples/hooks.rs'),
    appendExampleResult('//###HOOKS-RESULT###//', 'hooks'),

    appendExample('//###STATE###//', './examples/state.rs'),
    appendExampleResult('//###STATE-RESULT###//', 'state'),

    appendExample('//###IMPORT###//', './examples/importing-tests.rs'),
    appendExampleResult('//###IMPORT-RESULT###//', 'importing-tests'),

    appendExample('//###REPORT###//', './examples/reporting-json-pretty.rs'),
    appendExampleResult('//###REPORT-RESULT###//', 'reporting-json-pretty'),

    str => fs.writeFile('./README.md', str)
))().catch(console.log)
