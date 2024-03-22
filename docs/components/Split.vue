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
		:class="$style.container"
		ref="container"
		@mouseup="up"
		@mousemove="move"
	>
		<div :class="$style.left" ref="left">
			<slot name="left"></slot>
		</div>
		<div :class="$style.split" @mousedown="down">⣿</div>
		<div :class="$style.right">
			<slot name="right"></slot>
		</div>
	</div>
</template>

<style module>
	.container {
		display: flex;
	}

	.left,
	.right {
		flex: 1;
		min-width: 0;
	}

	.split {
		flex: 0 0 12px;
		display: flex;
		place-items: center;
		cursor: col-resize;
		user-select: none;
	}
</style>
