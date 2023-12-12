// eslint-disable-next-line node/no-unpublished-require
const { SharePlugin } = require("@rspack/core").sharing;

/** @type {import("@rspack/core").Configuration} */
module.exports = {
	experiments: {
		rspackFuture: {
			newTreeshaking: true
		}
	},
	entry: {
		main: "./index.js"
	},
	optimization: {
		minimize: false,
		moduleIds: "named",
		chunkIds: "named",
		mangleExports: true
	},
	plugins: [
		new SharePlugin({
			shared: ["shared"]
		})
	]
};
