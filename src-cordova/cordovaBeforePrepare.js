const { copyFileSync, rmSync } = require('fs');
const { execSync } = require('child_process');

const cordova_root = 'src-cordova/';

const css_src = 'index.css';
const css_dst_dir = 'www/css/';

const wasm_dst_dir = 'www/js/';
const wasm_build_cmd = 'wasm-pack build --release \
    --target=web \
    --no-typescript \
    --features=keyboard \
    --out-dir=';

module.exports = function (context) {
    // Copy CSS file
    console.log('Copying `' + css_src + '` to `' + css_dst_dir + css_src);
    copyFileSync('../' + css_src, css_dst_dir + css_src);

    // Build WASM files
    console.log('Building WASM module...');
    execSync(wasm_build_cmd + cordova_root + wasm_dst_dir, {'cwd': '../'});
    console.log('Removing excess files...');
    rmSync(wasm_dst_dir + 'package.json');
}
