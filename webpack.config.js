const path = require('path');
const stylesHandler = 'style-loader';

module.exports = {
  entry: './assets/js/index.js',
  output: {
    path: path.resolve(__dirname, 'assets/js'),
    filename: 'bundle.js',
  },
  watch: true,
  watchOptions: {
    ignored: ['/node_modules', '/src', '/target', '/templates']
  },
  module: {
    rules: [{
      test: /\.css$/i,
      use: [stylesHandler, 'css-loader', 'postcss-loader']
    },
    {
      test: /\.(eot|svg|ttf|woff|woff2|png|jpg|gif)$/i,
      type: 'asset'
    }]
  }
};
