const path = require('path')
const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {
  entry: path.join(__dirname, 'src/index.js'),
  output: {
    path: path.join(__dirname, 'dist'),
    filename: '[name].bundle.js'
  },
  devServer: {
    contentBase: path.join(__dirname, 'dist'),
    watchContentBase: true,
    publicPath: '/'
  },
  module: {
    rules: [
      {
        test: /\.rs$/,
        loader: 'rust-wasm-loader',
        options: {
          path: 'dist/'
        }
      },
      {
        test: /\.wasm$/,
        loader: 'file-loader'
      }
    ]
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html',
      filename: 'index.html'
    })
  ],
  externals: {
    'fs': true,
    'path': true,
  },
}