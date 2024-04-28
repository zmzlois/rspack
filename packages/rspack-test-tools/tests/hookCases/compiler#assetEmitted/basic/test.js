const assert = require("assert");
const { createFsFromVolume, Volume } = require("memfs");

const outputFileSystem = createFsFromVolume(new Volume());

let hasMainJs = false;
module.exports = {
	description: "should work with assetEmitted",
	options(context) {
		return {
			plugins: [
				{
					apply(compiler) {
						compiler.hooks.assetEmitted.tap("plugin", context.snapped((filename, info) => {
							if (filename === "main.js") {
								assert(info.targetPath.includes("main.js"));
								hasMainJs = true;
							}
						}));
					}
				}
			]
		};
	},
	async compiler(context, compiler) {
		compiler.outputFileSystem = outputFileSystem;
	},
	async check() {
		expect(hasMainJs).toBeTruthy();
	}
};
