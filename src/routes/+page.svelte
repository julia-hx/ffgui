<script>
	import { invoke } from "@tauri-apps/api/core";

	let name = $state("");
	let greetMsg = $state("");

	$effect(() => {
		console.log(name);
	});

	/**
	 * @param {{ preventDefault: () => void; }} event
	 */
	async function greet(event) {
		event.preventDefault();
		// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
		greetMsg = await invoke("greet", { name });
	}

	async function alsoGreet() {
		greetMsg = await invoke("greet", { name });
	}

	async function runCommand() {

	}
</script>

<main class="container">
	<h1>ffgui</h1>

	<div class="row">
		<p>ffmpeg command runner</p>		
	</div>

	<form class="row" onsubmit={alsoGreet}>
		<input
			id="greet-input"
			placeholder="Enter a name..."
			bind:value={name}
		/>
		<button type="submit">Greet</button>
	</form>
	<p>{greetMsg}</p>

	<div class="row">
		<a href="https://linktr.ee/julia.hx" target="_blank">julia.hx</a>
	</div>
</main>

<styles src="/shared.css">
</styles>
