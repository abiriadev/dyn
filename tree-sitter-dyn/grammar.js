/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
	name: 'dyn',

	rules: {
		source_file: $ => repeat($.expr),

		code: $ => repeat($.expr),

		expr: $ => choice($.identifier, $._literal),

		binexpr: $ =>
			choice(prec.left(1, seq($.expr, '+', $.expr))),

		identifier: $ => /[a-z]+/,

		_literal: $ => choice($.string, $.number),

		string: $ => /"[^"]*"/,

		number: $ => /\d+/,
	},
})
