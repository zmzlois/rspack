const { RunScriptWebpackPlugin } = require("run-script-webpack-plugin");
const path = require("path");
const clientConfig = {
	name: "client",
	context: __dirname,
	entry: {
		client: ["./src/client.tsx"]
	},
	devServer: {
		port: 5555
	},
	watchOptions: {
		aggregateTimeout: 2000
	},
	module: {
		rules: [
			{
				test: /\.(ts|tsx)$/,
				use: {
					loader: "builtin:swc-loader",
					options: {
						jsc: {
							parser: {
								syntax: "typescript",
								tsx: true,
								decorators: true
							}
						}
					}
				}
			}
		]
	},
	devServer: {
		devMiddleware: {
			writeToDisk: true
		}
	}
};
/** @type {import('@rspack/cli').Configuration} */
const serveConfig = {
	name: "server",
	context: __dirname,
	target: "node",
	entry: {
		main: ["webpack/hot/poll?100", "./src/main.ts"]
	},
	module: {
		rules: [
			{
				test: /\.(ts|tsx)$/,
				use: {
					loader: "builtin:swc-loader",
					options: {
						jsc: {
							parser: {
								syntax: "typescript",
								tsx: true,
								decorators: true
							}
						}
					}
				}
			}
		]
	},
	optimization: {
		minimize: false
	},
	externalsType: "commonjs",
	plugins: [
		!process.env.BUILD &&
			new RunScriptWebpackPlugin({
				name: "main.js",
				autoRestart: false
			})
	].filter(Boolean),
	devServer: {
		port: 5555,
		devMiddleware: {
			writeToDisk: true
		}
	}
};
module.exports = [serveConfig, clientConfig];
