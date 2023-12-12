const rspack = require("@rspack/core");
/** @type {import('@rspack/cli').Configuration} */
const config = {
	entry: "./src/index.js",
	experiments: {
		rspackFuture: {
			newTreeshaking: true
		}
	},
	optimization: {
		moduleIds: "named",
		minimize: false,
		sideEffects: false
	},
	plugins: [
		new rspack.HtmlRspackPlugin({
			template: "./index.html"
		})
	]
};
module.exports = config;
