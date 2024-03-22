<script setup lang="ts">
	import { ref, onMounted } from 'vue'
	import * as monaco from 'monaco-editor'
	import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'

	self.MonacoEnvironment = {
		getWorker: () => new editorWorker(),
	}

	const editorEl = ref(null)

	onMounted(() => {
		monaco.languages.register({ id: 'dyn' })

		monaco.languages.setMonarchTokensProvider('dyn', {
			tokenizer: { root: [] },
		})

		monaco.editor.create(editorEl.value, {
			language: 'dyn',
			automaticLayout: true,
		})
	})
</script>

<template>
	<div ref="editorEl"></div>
</template>
