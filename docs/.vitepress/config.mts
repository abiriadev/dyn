import { defineConfig } from 'vitepress'
import { readFileSync } from 'node:fs'

// https://vitepress.dev/reference/site-config
export default defineConfig({
	title: 'Dyn',
	description: 'Highly dynamic programming language',
	themeConfig: {
		// https://vitepress.dev/reference/default-theme-config
		nav: [
			{ text: 'Home', link: '/' },
			{
				text: 'Examples',
				link: '/markdown-examples',
			},
			{
				text: 'Playground',
				link: '/playground',
			},
		],

		sidebar: [
			{
				text: 'Examples',
				items: [
					{
						text: 'Markdown Examples',
						link: '/markdown-examples',
					},
					{
						text: 'Runtime API Examples',
						link: '/api-examples',
					},
				],
			},
		],

		socialLinks: [
			{
				icon: 'github',
				link: 'https://github.com/vuejs/vitepress',
			},
		],
	},
	markdown: {
		languages: [
			{
				id: 'dyn',
				scopeName: 'source.dyn',
				grammar: JSON.parse(
					readFileSync(
						'../dyn-vscode/syntaxes/dyn.tmLanguage.json',
					).toString(),
				),
			},
		],
	},
})
