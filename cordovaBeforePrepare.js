const fs = require('fs');

const css_src = 'index.css';
const css_dst_dir = 'www/css/';

module.exports = function (context) {
    fs.copyFileSync(css_src, css_dst_dir + css_src);
    console.log('Copied `' + css_src + '` to `' + css_dst_dir + css_src);
}
