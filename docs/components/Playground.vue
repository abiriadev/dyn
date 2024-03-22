<script setup lang="ts">
	import Split from './Split.vue'
	import Output from './Output.vue'
	import Buttons from './Buttons.vue'
	import Editor from './Editor.vue'
</script>

<template>
	<div :class="$style.main">
		<Buttons />
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
		gap: 16px;
	}

	.sp {
		flex: 1;
	}

	.editor {
		height: 100%;
	}
</style>
