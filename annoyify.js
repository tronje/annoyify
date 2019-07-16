function annoyify(text) {
	let result = "";
	let last_upcase = false;

	for (letter of text) {
		console.log(letter);
		if (last_upcase === true) {
			result = result.concat(letter.toUpperCase());
		} else {
			result = result.concat(letter.toLowerCase());
		}

		last_upcase = !last_upcase;
	}

	return result;
}

function run() {
	let elem = document.getElementById("input_text");
	let text = elem.value;
	let annoyified = annoyify(text);

	document.getElementById("output_text").value = annoyified;
}
