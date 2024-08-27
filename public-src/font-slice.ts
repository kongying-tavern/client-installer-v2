import Path from 'node:path';
import URL from 'node:url';
import { globbySync } from 'globby';
import FsExtra from 'fs-extra';
import SmolToml from 'smol-toml';
import FontMin from 'fontmin';
import GulpRename from 'gulp-rename';

const __filename = URL.fileURLToPath(import.meta.url);
const __dirname = Path.resolve(__filename, '../../');

interface ProfileData {
    theme: {
        font_class: string,
    },
}

interface FontTrimConfig {
    fontClass: string,
    fontFiles: string[],
    text: string,
    supportLangs: string[],
}

const STATIC_CHAR_LIST: string[] = [
    '!"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|} ',
];
const CLASS_FONT_MAP: Record<string, string[]> = {
    'font-misans': ['MiSans-Medium', 'MiSans-Semibold'],
    'font-misans-latin': ['MiSansLatin-Medium', 'MiSansLatin-Semibold'],
    'font-misans-tc': ['MiSansTC-Medium', 'MiSansTC-Semibold'],
};

const uniqueArray = <T extends string | number>(arr: T[]): T[] => Array.from(new Set(arr));

const getGlob = (pattern: string, cwd: string = __dirname): string[] => {
    const patternGlob: string = Path.resolve(cwd, pattern).replace(/\\/gui, '/');
    const matchPaths = globbySync(patternGlob);
    return matchPaths;
}

const readFile = (path: string): string => {
    const content: string = FsExtra.readFileSync(path, { encoding: 'utf-8' }) || '';
    return content;
};

const getFontNamesByClass = (className: string): string[] => CLASS_FONT_MAP[className] ?? [];

const mergeTrimConfig = (
    obj: Record<string, FontTrimConfig>,
    fontClass: string,
    profileName: string,
    translationContent: string
) => {
    let fontNames = getFontNamesByClass(fontClass);

    if (fontNames.length <= 0) return;

    let fontTrimConfig = obj[fontClass];
    if (!fontTrimConfig) {
        fontTrimConfig = {
            fontClass,
            fontFiles: fontNames,
            text: translationContent,
            supportLangs: [profileName],
        };
    } else {
        fontTrimConfig = {
            fontClass,
            fontFiles: uniqueArray([...fontTrimConfig.fontFiles, ...fontNames]),
            text: `${fontTrimConfig.text}${translationContent}`,
            supportLangs: [...fontTrimConfig.supportLangs, profileName],
        };
    }
    obj[fontClass] = fontTrimConfig;
};

const accumulateTrimConfig = (
    obj: Record<string, FontTrimConfig>,
    profilePath: string,
    _profilePathIdx: number,
    _arr: string[]
): Record<string, FontTrimConfig> => {
    let profileName = Path.basename(profilePath, '.toml');
    let profileContent = readFile(profilePath);
    let profileObj = SmolToml.parse(profileContent) as unknown as ProfileData;
    let translationPath = Path.resolve(__dirname, `./src/languages/translations/${profileName}.toml`)
    let translationContent = readFile(translationPath);
    let fontClass = profileObj['theme']['font_class'];

    mergeTrimConfig(obj, fontClass, profileName, translationContent);
    return obj;
};

const trimFont = (compressBase: string, config: FontTrimConfig) => {
    const trimFontFiles: string[] = config.fontFiles ?? [];
    const trimText: string = config.text ?? '';
    const trimTextFull: string = `${trimText}${STATIC_CHAR_LIST.join('')}`;
    const trimSupportLangs: string[] = config.supportLangs ?? [];

    for (let fontFile of trimFontFiles) {
        let fontPaths = getGlob(`./public-src/**/${fontFile}.ttf`);
        for (let fontPath of fontPaths) {
            const fontName: string = Path.basename(fontPath, '.ttf');
            const fontCompressedName: string = `${fontName}-compressed.woff2`;

            new FontMin.default()
                .src(fontPath)
                .dest(compressBase)
                .use(FontMin.default.glyph({
                    text: trimTextFull,
                }))
                .use(FontMin.default.ttf2woff2())
                .use(GulpRename(fontCompressedName))
                .run((e?: Error) => {
                    if (e) {
                        console.error(e.message);
                    }
                    console.log(`○ Sliced font: ${fontName}.ttf → ${fontCompressedName}`);
                    console.log(`→   Supported langs: ${trimSupportLangs.join(' ')}`);
                });
        }
    }
};

(async function () {
    // Build font trim config
    let profilePaths = getGlob('./src/languages/profile/*.toml');
    let fontTrimConfigMap: Record<string, FontTrimConfig> = {};
    fontTrimConfigMap = profilePaths.reduce(accumulateTrimConfig, fontTrimConfigMap);
    let fontTrimConfigList: FontTrimConfig[] = Object.values(fontTrimConfigMap);

    // Trim font
    let fontTargetBase: string = Path.resolve(__dirname, './public/fonts/');
    for (let trimConfig of fontTrimConfigList) {
        trimFont(fontTargetBase, trimConfig);
    }
})();
