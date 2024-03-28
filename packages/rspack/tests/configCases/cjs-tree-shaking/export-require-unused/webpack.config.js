module.exports = {
	entry: "./index",
	target: "node",
	optimization: {
		usedExports: true
	},
	experiments: {
		rspackFuture: {
			newTreeshaking: true
		}
	}
};
