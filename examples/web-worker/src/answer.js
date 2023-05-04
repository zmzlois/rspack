// import { secret } from "./secret"; not supported yet
self.onmessage = ({ data: { question } }) => {
	self.postMessage({
		answer: 42
	});
};
