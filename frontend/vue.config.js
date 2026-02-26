const { defineConfig } = require('@vue/cli-service')
const path = require('path')

module.exports = defineConfig({
  transpileDependencies: true,
  publicPath: '/', // Changed from /ServerWatch/ for custom domain
  outputDir: path.resolve(__dirname, '../dist'), // Output to repo root
})