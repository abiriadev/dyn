/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
	name: 'dyn',

	rules: {
		source_file: $ => repeat($.expr),

		expr: $ =>
			choice(
				$.identifier,
				$._literal,
				$.func,
				$.binexpr,
				$.unaryexpr,
				$.block,
				$.let,
				$.if,
				$.iter,
			),

		binexpr: $ =>
			choice(
				prec.left(5, seq($.expr, '*', $.expr)),
				prec.left(5, seq($.expr, '/', $.expr)),
				prec.left(5, seq($.expr, '%', $.expr)),
				prec.left(4, seq($.expr, '+', $.expr)),
				prec.left(4, seq($.expr, '-', $.expr)),
				prec.left(3, seq($.expr, '==', $.expr)),
				prec.left(3, seq($.expr, '!=', $.expr)),
				prec.left(3, seq($.expr, '<', $.expr)),
				prec.left(3, seq($.expr, '<=', $.expr)),
				prec.left(3, seq($.expr, '>', $.expr)),
				prec.left(3, seq($.expr, '>=', $.expr)),
				prec.left(2, seq($.expr, '&&', $.expr)),
				prec.left(1, seq($.expr, '||', $.expr)),
			),

		unaryexpr: $ =>
			choice(seq('-', $.expr), seq('-', $.expr)),

		block: $ => seq('{', repeat($.expr), '}'),

		let: $ =>
			seq(
				'let',
				optional('!'),
				$.identifier,
				'=',
				$.expr,
			),

		if: $ =>
			seq(
				'if',
				$.expr,
				$.block,
				optional(seq('else', $.block)),
			),

		iter: $ =>
			seq(
				'iter',
				$.expr,
				'of',
				$.identifier,
				$.block,
			),

		func: $ =>
			seq(
				optional(
					seq('|', repeat($.identifier), '|'),
				),
				'->',
				$.expr,
			),

		array: $ => seq('[', repeat($.expr), ']'),

		identifier: $ => /[a-z]+/,

		_literal: $ => choice($.string, $.number),

		string: $ => /"[^"]*"/,

		number: $ => /\d+/,
	},
})
