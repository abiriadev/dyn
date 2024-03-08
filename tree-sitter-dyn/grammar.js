/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
	name: 'dyn',

	rules: {
		source_file: $ => repeat($.expr),

		code: $ => repeat($.expr),

		expr: $ =>
			choice($.identifier, $.number, $.binexpr),

		binexpr: $ =>
			choice(prec.left(1, seq($.expr, '+', $.expr))),

		identifier: $ => /[a-z]+/,

		number: $ => /\d+/,
	},
})
