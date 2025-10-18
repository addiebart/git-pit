const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    mode: 'development',
    entry: './src/web/tsx/index.tsx',
    output: {
    filename: 'index.js',
    path: path.resolve(__dirname, 'build/web'),
    },
    resolve: {
    extensions: ['.tsx', '.ts', '.js', '.jsx', '.json'],
    },
    module: {
    rules: [
        {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
        },
        {
        test: /\.css$/,
        use: ['style-loader', 'css-loader'],
        exclude: /node_modules/,
        }
    ],
    },
    plugins: [
        new HtmlWebpackPlugin({
            title: 'git-pit',
            template: './src/web/html/index.html'
        })
    ]
};