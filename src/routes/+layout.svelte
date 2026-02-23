<script lang="ts">
	let { children } = $props();
</script>

<div class="container">
	<div class="row">
		<nav>
			<a href="/">home</a>
			<a href="/about">about</a>
		</nav>
	</div>
</div>


{@render children()}

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
</style>

