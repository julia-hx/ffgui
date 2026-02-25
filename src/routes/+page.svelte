<script>
	import { invoke } from "@tauri-apps/api/core";
	import DropZone from "../components/DropZone.svelte";

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

	async function runTestCommand() {
		await invoke("run_test_command");
	}
</script>

<main class="container">
	<h1>ffgui</h1>

	<!--
	<form class="row" onsubmit={alsoGreet}>
		<input
			id="greet-input"
			placeholder="Enter a name..."
			bind:value={name}
		/>
		<button type="submit">Greet</button>
	</form>
	<p>{greetMsg}</p>
	-->

	<div class="row">
		<div id="dropzone-container">
			<DropZone></DropZone>
		</div>
	</div>

	<div class="row">
		<button onclick={runTestCommand}>Test Command</button>
	</div>
</main>

<styles src="/shared.css"></styles>

<style>
	#dropzone-container {
		height: 110px;
		width: 300px;
		margin-top: 10px;
		display: flex;
		justify-content: center;
	}	
</style>