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
				text: 'Guide',
				link: '/markdown-examples',
			},
			{
				text: 'Reference',
				link: '/markdown-examples',
			},
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
				text: 'Guide',
				items: [
					{
						text: 'What is Dyn',
						link: '/guide/intro',
					},
					{
						text: 'Getting Started',
						items: [
							{
								text: 'Quickstart',
								link: '/guide/getting-started/quickstart',
							},
							{
								text: 'Installation',
								link: '/guide/getting-started/installation',
							},
							{
								text: 'Build from source',
								link: '/guide/getting-started/build-from-source',
							},
						],
					},
				],
			},
		],

		socialLinks: [
			{
				icon: 'github',
				link: 'https://github.com/abiriadev/dyn',
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
