const fs = require('fs');
const pkg = require('../package.json');

const triplesList = Object.values(
    require('@napi-rs/triples').platformArchTriples,
)
    .flatMap((p) => Object.values(p))
    .flat();
const triples = new Map(triplesList.map((t) => [t.raw, t]));

if (fs.existsSync('./npm'))
    fs.rmSync('./npm', { recursive: true, force: true });

for (const tripleRaw of pkg.napi.triples.additional) {
    const triple = triples.get(tripleRaw);
    const pkgName = `${pkg.name}-${triple.platformArchABI}`;
    const targetPkg = {
        name: pkgName,
        version: pkg.version,
        description: pkg.description,
        license: pkg.license,
        repository: pkg.repository,
        publishConfig: pkg.publishConfig,
        os: [triple.platform],
        cpu: [triple.arch],
        main: `core.${triple.platformArchABI}.node`,
        files: [`core.${triple.platformArchABI}.node`],
        engines: { node: '>= 10' },
    };
    if (triple.abi === 'musl') targetPkg.libc = ['musl'];
    else if (triple.abi === 'gnu') targetPkg.libc = ['glibc'];

    const readMe = `# \`${pkgName}\`\n\nThis is the **${tripleRaw}** binary for \`${pkg.name}\``;

    const pkgPath = `npm/${triple.platformArchABI}`;
    fs.mkdirSync(pkgPath, { recursive: true });
    fs.writeFileSync(
        `${pkgPath}/package.json`,
        JSON.stringify(targetPkg, null, 4),
    );
    fs.writeFileSync(`${pkgPath}/README.md`, readMe);
}
