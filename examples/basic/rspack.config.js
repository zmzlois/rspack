const rspack = require("@rspack/core");
/** @type {import('@rspack/cli').Configuration} */
const config = {
	entry: "./src/index.js",
	plugins: [
		new rspack.HtmlRspackPlugin({
			template: "./index.html"
		})
	],
	module: {
		rules: [
			{
				test: /index\.js/,
				loader: "builtin:mini-css-extract-rspack-plugin"
			}
		]
	}
};
module.exports = config;
