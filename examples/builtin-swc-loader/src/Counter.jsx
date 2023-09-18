import { useEffect, useState } from "react";

export function Counter() {
	const [cnt, setCnt] = useState(0);
	useEffect(() => {
		setInterval(() => {
			setCnt(x => x + 1);
		}, 1000);
	}, []);
	return <div>count: {cnt}</div>;
}
