<script lang="ts">
	let count = $state(0);

	async function getVote(id: number) {
		let response = await fetch('http://127.0.0.1:8000/' + id + '/vote');
		let vote = await response.json();
		return vote;
	}
	let promise = getVote(1);
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<button onclick={() => count++}>
	Wupaduudle {count}
</button>

{#await promise}
	Loading Beschluss
{:then data}
	<pre>{JSON.stringify(data, null, 4)}</pre>
{:catch error}
	error
{/await}
