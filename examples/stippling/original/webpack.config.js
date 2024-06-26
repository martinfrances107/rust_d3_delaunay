const path = require('path')
const CopyWebpackPlugin = require('copy-webpack-plugin')
const ESLintPlugin = require('eslint-webpack-plugin')
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
  entry:{
  'index': './js/index.ts',
  'script': "./js/script.ts",
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: "[name].js",
  },
  module: {
    rules: [{
      test: '/.ts?$/',
      use: 'ts-loader',
      exclude: '/node_modules/'
    }]
  },
  performance: {
    maxEntrypointSize: 1 * 1024 * 1024,
    maxAssetSize: 1 * 1024 * 1024
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js']
  },
  mode: 'development',
  devtool: 'inline-source-map',
  experiments: { syncWebAssembly: true },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: 'obama.png' },
        { from: 'index.html' },
      ]
    }),
  ]
}
