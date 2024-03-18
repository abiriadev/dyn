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
				link: '/guide/intro',
			},
			{
				text: 'Reference',
				link: '/reference/syntax',
			},
			{
				text: 'Examples',
				link: '/examples',
			},
			{
				text: 'Playground',
				link: '/playground',
			},
		],

		sidebar: {
			'/guide/': {
				base: '/guide/',
				items: [
					{
						text: 'Guide',
						items: [
							{
								text: 'What is Dyn',
								link: 'intro',
							},
							{
								text: 'Getting Started',
								items: [
									{
										text: 'Quickstart',
										link: 'getting-started/quickstart',
									},
									{
										text: 'Installation',
										link: 'getting-started/installation',
									},
									{
										text: 'Build from source',
										link: 'getting-started/build-from-source',
									},
								],
							},
							{
								text: 'Syntax Overview',
								items: [
									{
										text: 'Basic Syntax',
										link: 'syntax-overview/basic-syntax',
									},
									{
										text: 'Array',
										link: 'syntax-overview/array',
									},
									{
										text: 'Record',
										link: 'syntax-overview/record',
									},
									{
										text: 'Function',
										link: 'syntax-overview/function',
									},
								],
							},
						],
					},
				],
			},
			'/reference/': {
				base: '/reference/',
				items: [
					{
						text: 'Reference',
						items: [
							{
								text: 'Syntax',
								link: 'syntax',
							},
						],
					},
				],
			},
		},
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
		math: true,
	},
	vite: {
		ssr: {
			noExternal: ['monaco-editor'],
		},
	},
})
