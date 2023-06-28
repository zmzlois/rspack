/** @type {import('@rspack/cli').Configuration} */
const config = {
	context: __dirname,
	mode: "development",
	entry: {
		main: "./src/index.js"
	},
	builtins: {
		html: [
			{
				template: "./index.html"
			}
		],
		copy: ["index.html"]
	},
	plugins: [
		{
			apply(compiler) {
				compiler.hooks.compilation.tap("MyPlugin", compilation => {
					let a = "foo";
					compilation.hooks.assetPath.tap("MyPlugin", (...args) => {
						console.log(...args, a);
					});
					compilation.getAssetPath("");
					a = "bar";
					// compilation.getAssetPath("");
				});
			}
		}
	]
};
module.exports = config;
