const { defineConfig } = require('@vue/cli-service')
const path = require('path')

module.exports = defineConfig({
  transpileDependencies: true,
  publicPath: '/ServerWatch/', // Matches GitHub Pages URL
  outputDir: path.resolve(__dirname, '../dist'), // Output to repo root
})