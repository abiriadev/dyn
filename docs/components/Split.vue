<script setup lang="ts">
	import { ref } from 'vue'

	const left = ref(null)
	const container = ref(null)
	const isDragging = ref(false)

	const down = () => {
		isDragging.value = true
	}

	const up = () => {
		isDragging.value = false
	}

	const move = e => {
		if (!isDragging.value) return

		const offsetX =
			container.value.getBoundingClientRect().x
		const x = e.clientX - offsetX
		const q = x / (container.value.clientWidth - x)

		left.value.style.flexGrow = q
	}
</script>

<template>
	<div
		class="container"
		ref="container"
		@mouseup="up"
		@mousemove="move"
	>
		<slot name="left" class="left" ref="left"></slot>
		<div class="split" @mousedown="down">⣿</div>
		<slot name="right" class="right"></slot>
	</div>
</template>

<style scoped>
	.container {
		display: flex;
	}

	.left,
	.right {
		flex: 1;
	}

	.split {
		flex: 0 0 12px;
		display: flex;
		place-items: center;
		cursor: col-resize;
		user-select: none;
	}
</style>
