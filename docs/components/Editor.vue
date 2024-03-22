<script setup>
	import { defineAsyncComponent } from 'vue'
	import { inBrowser } from 'vitepress'

	const EditorInner = inBrowser
		? defineAsyncComponent(
				() => import('./EditorInner.vue'),
			)
		: () => null
</script>

<template>
	<EditorInner />
</template>
