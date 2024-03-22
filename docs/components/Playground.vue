<script setup lang="ts">
	import Split from './Split.vue'

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

	.editor {
		height: 100%;
	}
</style>
