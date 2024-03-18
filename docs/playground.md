---
layout: page
sidebar: false
---

<script setup>
	import { defineAsyncComponent } from 'vue';
	import { inBrowser } from 'vitepress';

	const Editor = inBrowser ? defineAsyncComponent(() => import('./components/Editor.vue')) : () => null;
</script>

<Editor width="600" height="800" />
