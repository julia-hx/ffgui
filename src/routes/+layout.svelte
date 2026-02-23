<script lang="ts">
	let { children } = $props();
</script>

<link rel="stylesheet" href="/styles/shared.css">

<div class="container">
	<div class="row">
		<nav>
			<a href="/">home</a>
			<a href="/about">about</a>
		</nav>
	</div>
</div>


{@render children()}
