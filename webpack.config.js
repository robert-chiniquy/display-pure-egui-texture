// @ts-nocheck
const webpack = require('webpack');
const path = require('path');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyWebpackPlugin = require("copy-webpack-plugin");
const { SourceMapDevToolPlugin } = require("webpack");

module.exports = {
    entry: "./web/bootstrap.js",
    output: {
        path: path.resolve(__dirname, "web/dist"),
        filename: "bootstrap.js",
    },
    mode: "development",
    devtool: 'inline-source-map',
    plugins: [
        new CopyWebpackPlugin(['web/index.html']),
        new WasmPackPlugin({
            forceMode: "production",
            crateDirectory: __dirname,
            outDir: path.resolve(__dirname,"./web/pkg")
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        }),
        new SourceMapDevToolPlugin({
            filename: "[file].map",
            include: /\.js$/,
        }),
    ],
};
