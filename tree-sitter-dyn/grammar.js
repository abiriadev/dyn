/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
	name: 'dyn',

	rules: {
		source_file: $ => $.code,

		code: $ => repeat($.expr),

		expr: $ => choice($.identifier, $.number),

		identifier: $ => /[a-z]+/,

		number: $ => /\d+/,
	},
})
