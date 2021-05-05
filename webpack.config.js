'use strict';
const path = require('path');

module.exports = {
    devtool: 'inline-source-map',
	entry: './src/ts/index.ts',
	output: {
		filename: 'dist.js',
        path: path.resolve(__dirname, 'dist'),
        libraryTarget: 'var',
        library: 'CRMQuery'
	},
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                loader: 'awesome-typescript-loader'
            },
            {
                test: /\.scss$/,
                use: [
                    'style-loader',
                    'css-loader',
                    'sass-loader',
                ]
            }
        ]
    },
    resolve: {
        extensions: [ '.ts', '.tsx', '.js' ]
    },
    externals: {
        // require("jquery") is external and available
        //  on the global var jQuery
        "jquery": "jQuery"
    },
    mode: 'production'
};