declare const module: any;
import express from "express";
import path from "node:path";
import { rootHandler } from "./api/root";
async function bootstrap() {
	const app = express();
	app.use(express.static("dist"));
	app.get("/", rootHandler);

	const server = app.listen(4000, () =>
		console.log("Listening on http://localhost:4000")
	);
	if (module.hot) {
		module.hot.accept();
		module.hot.dispose(() => server.close());
	}
}
bootstrap();
