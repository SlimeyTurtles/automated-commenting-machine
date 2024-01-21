function multiply(a: number, b: number) {
	return a * b;
}

type Vector2 = {
	x: number;
	y: number;
};

function getDistance(p1: Vector2, p2: Vector2) {
	const { x: x1, y: y1 } = p1;
	const { x: x2, y: y2 } = p2;
	const y = x2 - x1;
	const x = y2 - y1;
	return Math.sqrt(x * x + y * y);
};