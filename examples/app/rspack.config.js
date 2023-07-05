/** @type {import('@rspack/cli').Configuration} */
const config = {
	context: __dirname,
	mode: "development",
	entry: {
		main: "./index.js",
	},
	builtins: {
		html: [{}],
		treeShaking: true
	},
	optimization: {

	}
};
module.exports = config;
