const zlib = require('zlib')
const path = require('path')
const CopyWebpackPlugin = require('copy-webpack-plugin')
const CompressionPlugin = require('compression-webpack-plugin')
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
  mode: 'production',
  experiments: { syncWebAssembly: true },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [
        { from: 'obama.png' },
        { from: 'index.html' },
      ]
    }),
    new CompressionPlugin({
      filename: '[path][base].br',
      algorithm: 'brotliCompress',
      test: /\.(js|json|css|html)$/,
      compressionOptions: {
        params: {
          [zlib.constants.BROTLI_PARAM_QUALITY]: 11
        }
      },
      threshold: 10240,
      minRatio: 0.8,
      deleteOriginalAssets: false
    })

  ]
}
