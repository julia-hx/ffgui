<div class="container">
	<div class="row">
		<h1>about</h1>
	</div>
	
	<div class="row">
		ffgui is a command runner for ffmpeg.
	</div>

	<div class="row">
		<a href="https://ffmpeg.org/" class="inline-link" target="_blank">ffmpeg</a> needs to be installed and on system PATH.
	</div>


	<div class="row">
		<p>made in Rust and JS with:</p>
	</div>

	<div class="row">
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

		<a href="https://vite.dev" target="_blank">
			<img src="/vite.svg" class="logo vite" alt="Vite Logo" />
		</a>
	</div>

	<div class="row">
		by <a href="https://linktr.ee/julia.hx" target="_blank">julia.hx</a>
	</div>
</div>

<style>
	.logo {
		will-change: filter;
		transition: 0.75s;
	}

	.logo.svelte-kit:hover {
		filter: drop-shadow(0 0 0.4em #ff3e00);
	}

	.logo.vite:hover {
		filter: drop-shadow(0 0 0.4em #747bff);
	}

	.logo.tauri:hover {
		filter: drop-shadow(0 0 0.4em #24c8db);
	}

	.logo {
		height: 3em;
		padding: 1.5em;
	}

	.inline-link {
		margin-right: 8px;
	}
</style>
