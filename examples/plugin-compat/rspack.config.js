const BundleAnalyzerPlugin =
	require("webpack-bundle-analyzer").BundleAnalyzerPlugin;
const CopyPlugin = require("copy-webpack-plugin");
const HtmlPlugin = require("@rspack/plugin-html").default;
const { StatsWriterPlugin } = require("webpack-stats-plugin");
const csp = require("webpack-subresource-integrity").SubresourceIntegrityPlugin;
const GeneratePackageJsonPlugin = require("generate-package-json-webpack-plugin");
/**
 * @type {import('@rspack/cli').Configuration}
 */
module.exports = {
	target: "node",
	mode: "development",
	stats: { all: true },
	entry: {
		main: "./src/index.js"
	},
	builtins: {
		minify: false
	},
	plugins: [
		new csp(),
		{
			apply(compiler) {
				compiler.hooks.done.tap("xx", stats => {
					const assets = stats.toJson({ all: true });
					console.log("xxxyyy:", assets);
				});
			}
		}
	]
};

var basePackage = {
	name: "my-nodejs-module",
	version: "1.0.0",
	main: "./bundle.js",
	engines: {
		node: ">= 14"
	}
};
