// import { version } from "./answer";
// version;
(async () => {
	// const { version } = await import("./answer");
	// version;
	import("./answer").then(res => {
		res.version;
	});
})();
