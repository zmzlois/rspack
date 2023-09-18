const isDev = process.env.NODE_ENV === "development";
const ReactRefreshPlugin = require("@rspack/plugin-react-refresh");
/** @type {import('@rspack/cli').Configuration} */
const config = (env, ...args) => {
	return {
		entry: {
			main: "./src/index.jsx"
		},
		module: {
			rules: [
				// {
				// 	test: /\.jsx$/,
				// 	use: {
				// 		loader: "builtin:swc-loader",
				// 		options: {
				// 			// Enable source map
				// 			sourceMap: true,
				// 			jsc: {
				// 				parser: {
				// 					syntax: "ecmascript",
				// 					jsx: true
				// 				},
				// 				transform: {
				// 					react: {
				// 						pragma: "React.createElement",
				// 						pragmaFrag: "React.Fragment",
				// 						throwIfNamespace: true,
				// 						development: false,
				// 						useBuiltins: false,
				// 						runtime: "automatic"
				// 					}
				// 				}
				// 			}
				// 		}
				// 	},
				// 	type: "javascript/auto"
				// },
				{
					test: /\.(png|svg|jpg)$/,
					type: "asset/resource"
				}
			]
		},
		builtins: {
			html: [
				{
					template: "./index.html"
				}
			]
		},
		plugins: [new ReactRefreshPlugin()].filter(Boolean),
		experiments: {
			rspackFuture: {
				// disableTransformByDefault: true,
				disableReactRefreshByDefault: true
			}
		}
	};
};
module.exports = config;
