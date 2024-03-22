<script setup lang="ts">
	import Split from './Split.vue'
	import Output from './Output.vue'

	import { defineAsyncComponent } from 'vue'
	import { inBrowser } from 'vitepress'

	const Editor = inBrowser
		? defineAsyncComponent(() => import('./Editor.vue'))
		: () => null
</script>

<template>
	<div :class="$style.main">
		<header>Header</header>
		<Split :class="$style.sp">
			<template #left>
				<Editor :class="$style.editor" />
			</template>
			<template #right>
				<Output />
			</template>
		</Split>
	</div>
</template>

<style module>
	.main {
		width: 100%;
		height: calc(100vh - 64px);
		padding: 16px;
		display: flex;
		flex-direction: column;
	}

	.sp {
		flex: 1;
	}

	.editor {
		height: 100%;
	}
</style>
