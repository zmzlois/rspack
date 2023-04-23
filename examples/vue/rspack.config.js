const { VueLoaderPlugin } = require('vue-loader')

/**
 * @type {import('@rspack/cli').Configuration}
 */
module.exports = {
  mode: 'development',
  entry: {
    main: './src/main.js'
  },
  context: __dirname,
  builtins: {
    html: [{
      template: './index.html'
    }],
    define: {
      __VUE_OPTIONS_API__: JSON.stringify(true),
      __VUE_PROD_DEVTOOLS__: JSON.stringify(false)
    },
    progress: false
  },
  devServer: {
    historyApiFallback: true
  },
  module: {
    rules: [
      // {
      //   test: /\.vue$/,
      //   use: ['./vue-loader.js']
      // },
      // {
      //   test: /\.vue$/,
      //   resourceQuery: /type=style/,
      //   use: ['./vue-loader.js'],
      //   type: 'css'
      // },
      // {
      //   test: /\.css$/,
      //   use: ['style-loader', 'css-loader'],
      //   type: 'javascript/auto'
      // },
      {
        test: /\.vue$/,
        resourceQuery: {
          and: [
            /vue/,
            /type=style/,
          ]
        },
        type: "css"
      },
      {
        test: /\.vue$/,
        loader: 'vue-loader'
      },
      {
        test: /\.svg/,
        type: 'asset/resource'
      }
    ]
  },
  plugins: [
    new VueLoaderPlugin()
  ]
}