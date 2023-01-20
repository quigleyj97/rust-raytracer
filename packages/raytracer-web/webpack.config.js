const webpack = require('webpack');
const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const config = {
    entry: [
        './src/index.tsx'
    ],
    output: {
        path: path.resolve(__dirname, 'public/dist'),
        filename: 'bundle.js'
    },
    module: {
        rules: [
            {
                test: /\.(js|jsx)$/,
                use: 'babel-loader',
                exclude: /node_modules/
            },
            {
                test: /\.ts(x)?$/,
                loader: 'ts-loader',
                exclude: /node_modules/
            },
            {
                test: /\.scss$/,
                use: [
                    'style-loader',
                    'css-loader',
                    'sass-loader'
                ]
            }
        ]
    },
    devServer: {
        'static': {
            directory: './public/'
        }
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve("../raytracer-core"),
            extraArgs: "-- --features wasm"
        })
    ],
    resolve: {
        extensions: [
            '.ts',
            '.tsx'
        ]
    },
    experiments: {
        asyncWebAssembly: true
    },
    devtool: 'eval'
};

module.exports = config;