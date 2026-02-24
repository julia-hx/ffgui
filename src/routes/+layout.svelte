<script lang="ts">
	let { children } = $props();
</script>

<link rel="stylesheet" href="/styles/shared.css">

<div class="container">
	<nav class="row">
		<a href="/">home</a>
		<a href="/about">about</a>
	</nav>
</div>

{@render children()}
