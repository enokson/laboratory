const fs = require('fs').promises;

const example = tag => (src, dest) => dest.replace(tag, src)

const simple = example('//###SIMPLE###//')
const simpleResult = example('//###SIMPLE-RESULT###//')

const failure = example('//###FAILURE###//')
const nested = example('//###NESTED###//')

const readFile = (name, path) => obj => {
    return fs.readFile(path, 'utf8').then(data => {
        return Object.assign(obj, { [name]: data })
    })
}

readFile('readme', 'README.template.md')({})
    .then(readFile('simple', './examples/simple.rs'))
    .then(readFile('nested', './examples/nested-suites.rs'))
    .then(readFile('failure', './examples/failure.rs'))
    .then(obj => fs.writeFile('README.md',
        nested(obj.nested,
            failure(obj.failure,
                simple(obj.simple, obj.readme)))))

