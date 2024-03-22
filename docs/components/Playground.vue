<script setup lang="ts">
	import Split from './Split.vue'
</script>

<template>
	<div :class="$style.main">
		<header>Header</header>
		<Split :class="$style.sp">
			<template #left>
				<div :class="$style.a">A</div>
			</template>
			<template #right>
				<div :class="$style.b">B</div>
			</template>
		</Split>
	</div>
</template>

<style module>
	.main {
		width: 100%;
		height: calc(100vh - 64px);
		display: flex;
		flex-direction: column;
	}

	.sp {
		outline: 1px solid red;
		flex: 1;
	}

	.a,
	.b {
		height: 100%;
	}
</style>
