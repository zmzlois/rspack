import {
	BuiltinPlugin,
	BuiltinPluginName,
	JsChunk,
	JsModule,
	RawCacheGroupOptions,
	RawSplitChunksOptions,
	RawSplitChunksSizesOptions
} from "@rspack/binding";
import assert from "assert";

import { Chunk } from "../Chunk";
import { Compiler } from "../Compiler";
import {
	OptimizationSplitChunksSizes,
	type OptimizationSplitChunksOptions
} from "../config/zod";
import { Module } from "../Module";
import { createBuiltinPlugin, RspackBuiltinPlugin } from "./base";

export class SplitChunksPlugin extends RspackBuiltinPlugin {
	name = BuiltinPluginName.SplitChunksPlugin;
	affectedHooks = "thisCompilation" as const;

	constructor(private options: OptimizationSplitChunksOptions) {
		super();
	}

	raw(compiler: Compiler): BuiltinPlugin {
		const rawOptions = toRawSplitChunksOptions(this.options, compiler);
		assert(typeof rawOptions !== "undefined");
		return createBuiltinPlugin(this.name, rawOptions);
	}
}

function toRawSplitChunksOptions(
	sc: false | OptimizationSplitChunksOptions,
	compiler: Compiler
): RawSplitChunksOptions | undefined {
	if (!sc) {
		return;
	}

	function getRawSplitChunksSizes(
		sizes?: OptimizationSplitChunksSizes
	): RawSplitChunksSizesOptions | undefined {
		if (typeof sizes === "number") {
			return {
				javascript: sizes
			};
		}
		return sizes;
	}

	function getName(name: any) {
		interface Context {
			module: JsModule;
		}

		if (typeof name === "function") {
			return (ctx: Context) => {
				if (typeof ctx.module === "undefined") {
					return name(undefined);
				} else {
					return name(Module.__from_binding(ctx.module));
				}
			};
		} else {
			return name;
		}
	}

	function getTest(test: any) {
		interface Context {
			module: JsModule;
		}

		if (typeof test === "function") {
			return (ctx: Context) => {
				if (typeof ctx.module === "undefined") {
					return test(undefined);
				} else {
					return test(Module.__from_binding(ctx.module));
				}
			};
		} else {
			return test;
		}
	}

	function getChunks(chunks: any) {
		if (typeof chunks === "function") {
			return (chunk: JsChunk) =>
				chunks(
					Chunk.__from_binding(
						chunk,
						compiler._lastCompilation!.__internal_getInner()
					)
				);
		} else {
			return chunks;
		}
	}

	const {
		name,
		chunks,
		defaultSizeTypes,
		cacheGroups = {},
		fallbackCacheGroup,
		minSize,
		...passThrough
	} = sc;

	return {
		name: getName(name),
		chunks: getChunks(chunks),
		minSize: getRawSplitChunksSizes(minSize),
		defaultSizeTypes: defaultSizeTypes || ["javascript", "unknown"],
		cacheGroups: Object.entries(cacheGroups)
			.filter(([_key, group]) => group !== false)
			.map(([key, group]) => {
				group = group as Exclude<typeof group, false>;

				const { test, name, chunks, minSize, ...passThrough } = group;
				const rawGroup: RawCacheGroupOptions = {
					key,
					test: getTest(test),
					name: getName(name),
					chunks: getChunks(chunks),
					minSize: getRawSplitChunksSizes(minSize),
					...passThrough
				};
				return rawGroup;
			}),
		fallbackCacheGroup: {
			chunks: getChunks(chunks),
			...fallbackCacheGroup,
			minSize: getRawSplitChunksSizes(fallbackCacheGroup?.minSize)
		},
		...passThrough
	};
}
