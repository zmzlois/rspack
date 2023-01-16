/**
 * @type {import('webpack').Configuration}
 */
module.exports = {
	mode: "development",
	entry: {
		main: {
			import: ["./index.js"]
		}
	},
	define: {
		"process.env.NODE_ENV": "'development'"
	},
	module: {
    rules: [
      {
        test: {
          type: "regexp",
          matcher: "a.js$",
        },
        sideEffects: true
      }
    ],
  },
	infrastructureLogging: {
		debug: true
	},
	builtins: {
		treeShaking: true,
		sideEffects: true
	}
};
