import React from "react";
import { renderToString } from "react-dom/server";
import { App } from "./app";
export function rootHandler(req, res) {
	const result = renderToString(<App />);
	res.send(`
	<!DOCTYPE html>
	<html>
	<head>
	
	</head>
	<body>
	
	<div id="root">${result}</div>
	<script src="client.js" async defer></script>
	</body>
	</html>
	`);
}
