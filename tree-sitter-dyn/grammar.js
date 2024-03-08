/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
	name: 'dyn',

	rules: {
		source_file: $ => repeat($.expr),

		expr: $ =>
			choice($.identifier, $._literal, $.block, $.if),

		binexpr: $ =>
			choice(
				prec.left(1, seq($.expr, '+', $.expr)),
				prec.left(2, seq($.expr, '*', $.expr)),
			),

		block: $ => seq('{', repeat($.expr), '}'),

		if: $ => seq('if', $.expr, $.block),

		identifier: $ => /[a-z]+/,

		_literal: $ => choice($.string, $.number),

		string: $ => /"[^"]*"/,

		number: $ => /\d+/,
	},
})
