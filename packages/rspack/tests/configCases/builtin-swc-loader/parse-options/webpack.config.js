module.exports = {
	module: {
		rules: [
			{
				test: /a\.js/,
				loader: "builtin:swc-loader",
				options: {
					jsc: {
						__: ""
					}
				}
			}
		]
	}
}
