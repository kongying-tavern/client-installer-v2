import Path from 'node:path';
import URL from 'node:url';
import { globbySync } from 'globby';
import FsExtra from 'fs-extra';
import FontMin from 'fontmin';
import GulpRename from 'gulp-rename';

const CURFILE = URL.fileURLToPath(import.meta.url);
const __dirname = Path.resolve(CURFILE, '../../');

const STATIC_CHAR_LIST: string[] = [
    '!"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|} ',
];

const getGlob = (pattern: string, cwd: string = __dirname): string[] => {
    const patternGlob: string = Path.resolve(cwd, pattern).replace(/\\/gui, '/');
    const matchPaths = globbySync(patternGlob);
    return matchPaths;
}

const readFile = (path: string) => {
    const content: string = FsExtra.readFileSync(path, { encoding: 'utf-8' }) || '';
    return content;
};

(async function () {
    // Get translation text
    let translationPaths = getGlob('./src/locale/translations/**/*.toml');
    let translationContent = translationPaths
        .map(path => readFile(path))
        .join('');
    let translationFullContent = `${translationContent}${STATIC_CHAR_LIST.join('')}`;

    // Trim font
    let fontPaths = getGlob('./public-src/**/*.ttf');
    let fontTargetBase: string = Path.resolve(__dirname, './public/fonts/');
    for (let fontPath of fontPaths) {
        const fontName: string = Path.basename(fontPath, '.ttf');
        const fontCompressedName: string = `${fontName}-compressed.woff2`;

        new FontMin.default()
            .src(fontPath)
            .dest(fontTargetBase)
            .use(FontMin.default.glyph({
                text: translationFullContent,
            }))
            .use(FontMin.default.ttf2woff2())
            .use(GulpRename(fontCompressedName))
            .run((e?: Error) => {
                if (e) {
                    console.error(e.message);
                }
                console.log(`Sliced font: ${fontName}.ttf → ${fontCompressedName}`);
            });
    }
})();
