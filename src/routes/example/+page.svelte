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
</script>

<main class="container">
	<h1>Welcome to Tauri + Svelte</h1>

	<div class="row">
		<a href="https://vite.dev" target="_blank">
			<img src="/vite.svg" class="logo vite" alt="Vite Logo" />
		</a>
		<a href="https://tauri.app" target="_blank">
			<img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
		</a>
		<a href="https://svelte.dev" target="_blank">
			<img
				src="/svelte.svg"
				class="logo svelte-kit"
				alt="SvelteKit Logo"
			/>
		</a>
	</div>
	<p>Click on the Tauri, Vite, and SvelteKit logos to learn more.</p>

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

<style>
	:root {
		font-family: monospace;
		font-size: 16px;
		line-height: 24px;
		font-weight: 400;

		color: #f6f6f6;
		background-color: #0f0f0f;

		font-synthesis: none;
		text-rendering: optimizeLegibility;
		-webkit-font-smoothing: antialiased;
		-moz-osx-font-smoothing: grayscale;
		-webkit-text-size-adjust: 100%;
	}

	:global(.container) {
		margin: 0;
		padding-top: 4vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
		text-align: center;
	}

	:global(.row) {
		display: flex;
		justify-content: center;
	}
	
	:global(a) {
		font-weight: 500;
		color: #646cff;
		text-decoration: inherit;
	}

	:global(a:hover) {
		color: #535bf2;
	}

	.logo.vite:hover {
		filter: drop-shadow(0 0 0.2em #747bff);
	}

	.logo.svelte-kit:hover {
		filter: drop-shadow(0 0 0.2em #ff3e00);
	}

	.logo.tauri:hover {
		filter: drop-shadow(0 0 0.2em #24c8db);
	}

	.logo {
		height: 6em;
		padding: 1.5em;
		will-change: filter;
		transition: 0.75s;
	}

	h1 {
		text-align: center;
	}

	input,
	button {
		border-radius: 8px;
		border: 1px solid transparent;
		padding: 0.6em 1.2em;
		font-size: 1em;
		font-weight: 500;
		font-family: inherit;
		color: #0f0f0f;
		background-color: #f0f0f0;
		transition: border-color 0.25s;
		box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
	}

	button {
		cursor: pointer;
		color: #f0f0f0;
		background-color: #396cd8;
	}

	button:hover {
		border-color: #f0f0f0;
	}
	button:active {
		background-color: #0f0f0f;
	}

	input,
	button {
		outline: none;
	}

	#greet-input {
		margin-right: 5px;
	}
</style>
