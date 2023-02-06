const { copyFileSync } = require('fs');
const { execSync } = require('child_process');

const css_src = 'index.css';
const css_dst_dir = 'www/css/';
const wasm_dst_dir = 'node_modules/psh-gui';
const wasm_pack_cmd = 'wasm-pack build --release --target web --no-typescript';

module.exports = function (context) {
    copyFileSync(css_src, css_dst_dir + css_src);
    console.log('Copied `' + css_src + '` to `' + css_dst_dir + css_src);

    try {
        execSync(wasm_pack_cmd + ' --out-dir ' + wasm_dst_dir);
    } catch(error) {
        console.log('Error building WASM module: ' + error);
    }
}
