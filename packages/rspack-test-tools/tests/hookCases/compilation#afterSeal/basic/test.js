const { createFsFromVolume, Volume } = require("memfs");

const outputFileSystem = createFsFromVolume(new Volume());

const assert = require("assert");

const PLUGIN_NAME = "test-plugin";

class Plugin {
	apply(compiler) {
		let afterProcessAssetsCalled = false;
		let afterSealSyncCalled = false;
		let afterSealAsyncCalled = false;
		compiler.hooks.compilation.tap("test", compilation => {
			assert(typeof compilation.hooks.afterSeal !== "undefined");
			compilation.hooks.afterProcessAssets.tap(PLUGIN_NAME, () => {
				afterProcessAssetsCalled = true;
			});
			compilation.hooks.afterSeal.tap(PLUGIN_NAME, () => {
				assert(afterProcessAssetsCalled);
				afterSealSyncCalled = true;
			});
			compilation.hooks.afterSeal.tapPromise(PLUGIN_NAME, async () => {
				assert(afterProcessAssetsCalled);
				afterSealAsyncCalled = true;
			});
		});

		compiler.hooks.done.tap("check", () => {
			assert(afterSealSyncCalled);
			assert(afterSealAsyncCalled);
		});
	}
}

module.exports = {
	options(context) {
		return {
			plugins: [
				new Plugin()
			]
		};
	},
	async compiler(context, compiler) {
		compiler.outputFileSystem = outputFileSystem;
	},
	async check() {}
};
