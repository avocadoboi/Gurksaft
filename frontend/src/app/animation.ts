/*
	CSS animations are laggy on webkitgtk for some reason, so this file contains 
	some simple animation utilities.
*/

export function reciprocalEaseOutTransferFunction(t: number, sharpness: number = 0.8) {
	return t/(t - Math.pow(1 - sharpness, 2)*(t - 1))
}

/*
	After this function is called, the passed callback function will be called
	every animation frame until the given time duration (given in milliseconds) has passed.
	The passed time passed to the callback function is normalized between 0 and 1.
	That's a funny sentence
*/
export function animate(callback: (normalizedTime: number) => void, duration: number) {
	const startTime = performance.now();
	const frame = function(timeStamp: number) {
		const t = Math.min((timeStamp - startTime)/duration, 1);
		callback(t);
		if (t < 1) {
			requestAnimationFrame(frame);
		}
	};
	frame(startTime);
}