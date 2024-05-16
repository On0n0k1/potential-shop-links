const path = require('path')

module.exports = {
    entry: './dependencies/index.js',
    mode: 'production',
    output: {
        path: path.resolve(__dirname, 'static'),
        filename: 'bundle.js',
        //library: 'PackagedLibrary',
        //libraryTarget: 'umd',
        //libraryTarget: 'var',
        //umdNamedDefine: true
        //globalObject: 'this',
        //library: {
        //    name: "PackagedLibrary",
        //    type: "umd"
        //}
        //library: "PackagedLibrary"
        globalObject: 'this',
        library: {
            name: "PackagedLibrary",
            type: "umd"
        }
    },
    optimization: {
        minimize: false
    }
}
