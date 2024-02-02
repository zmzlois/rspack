/** @type {import("@rspack/core").Configuration} */
module.exports = {
	mode: 'production',
	output: {
		filename: "[name].js"
	},
	experiments: {
		rspackFuture: {
			newTreeshaking: true
		}
	},
	target: "node",
	optimization: {
		minimize: false,
		chunkIds: "named",
		splitChunks: {
			chunks: "all",
			cacheGroups: {
				defaultVendors: {
					test: /[\\/]node_modules[\\/]/,
					enforce: true
				}
			}
		}
	},
	entry: {
		a: "./1",
		b: "./2",
		c: "./3"
	}
};
