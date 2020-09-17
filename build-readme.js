const fs = require('fs').promises;

const example = tag => (src, dest) => dest.replace(tag, src)
const appendHeaderAndFooter = str => {
    return "running 1 test\n\n\n\n" + str + "\n\n\n\ntest tests::suite ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"
}

const simple = example('//###SIMPLE###//')


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

