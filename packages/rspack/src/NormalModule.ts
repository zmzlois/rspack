import { SyncHook } from "tapable";
import { Compilation } from "./compilation";

interface NormalModuleCompilationHooks {
	loader: SyncHook<[object, NormalModule]>;
}

const compilationHooksMap = new WeakMap<
	Compilation,
	NormalModuleCompilationHooks
>();

class NormalModule {
	static getCompilationHooks(compilation: Compilation) {
		if (!(compilation instanceof Compilation)) {
			throw new TypeError(
				"The 'compilation' argument must be an instance of Compilation"
			);
		}
		let hooks = compilationHooksMap.get(compilation);
		if (hooks === undefined) {
			hooks = {
				loader: new SyncHook<[object, NormalModule]>([
					"loaderContext",
					"module"
				])
			};
			compilationHooksMap.set(compilation, hooks);
		}
		return hooks;
	}
}

export { NormalModule };
