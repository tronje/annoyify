function annoyify(text) {
	let result = "";
	let last_upcase = false;

	for (letter of text) {
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

function animated_button_response(e) {
	e.preventDefault;

	//reset animation
	e.target.classList.remove('animate');

	e.target.classList.add('animate');
	setTimeout(function() {
		e.target.classList.remove('animate');
	}, 700);

	run();
};

window.addEventListener('DOMContentLoaded', (event) => {
	let button = document.getElementById("animated_button");
	button.addEventListener('click', animated_button_response, false);
});
