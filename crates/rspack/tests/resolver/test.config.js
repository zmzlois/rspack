const path = require("path");

/**
 * @type {import("@rspack/cli").Configuration}
 */
module.exports = {
	context: __dirname,
	entry: {
		main: {
			import: [path.resolve(__dirname, "./src/index.js")]
		}
	},
	resolve: {
		alias: {
			"@app": path.resolve(__dirname, "src")
		}
	},
	experiments: {
		incrementalRebuild: true,
		rspackFuture: {
			newResolver: false
		}
	}
}
