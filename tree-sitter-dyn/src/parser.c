#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 37
#define LARGE_STATE_COUNT 18
#define SYMBOL_COUNT 25
#define ALIAS_COUNT 0
#define TOKEN_COUNT 15
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 5
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  anon_sym_PLUS = 1,
  anon_sym_STAR = 2,
  anon_sym_DASH = 3,
  anon_sym_LBRACE = 4,
  anon_sym_RBRACE = 5,
  anon_sym_if = 6,
  anon_sym_else = 7,
  anon_sym_PIPE = 8,
  anon_sym_DASH_GT = 9,
  anon_sym_LBRACK = 10,
  anon_sym_RBRACK = 11,
  sym_identifier = 12,
  sym_string = 13,
  sym_number = 14,
  sym_source_file = 15,
  sym_expr = 16,
  sym_binexpr = 17,
  sym_unaryexpr = 18,
  sym_block = 19,
  sym_if = 20,
  sym_func = 21,
  sym__literal = 22,
  aux_sym_source_file_repeat1 = 23,
  aux_sym_func_repeat1 = 24,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_PLUS] = "+",
  [anon_sym_STAR] = "*",
  [anon_sym_DASH] = "-",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_if] = "if",
  [anon_sym_else] = "else",
  [anon_sym_PIPE] = "|",
  [anon_sym_DASH_GT] = "->",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [sym_identifier] = "identifier",
  [sym_string] = "string",
  [sym_number] = "number",
  [sym_source_file] = "source_file",
  [sym_expr] = "expr",
  [sym_binexpr] = "binexpr",
  [sym_unaryexpr] = "unaryexpr",
  [sym_block] = "block",
  [sym_if] = "if",
  [sym_func] = "func",
  [sym__literal] = "_literal",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
  [aux_sym_func_repeat1] = "func_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_if] = anon_sym_if,
  [anon_sym_else] = anon_sym_else,
  [anon_sym_PIPE] = anon_sym_PIPE,
  [anon_sym_DASH_GT] = anon_sym_DASH_GT,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [sym_identifier] = sym_identifier,
  [sym_string] = sym_string,
  [sym_number] = sym_number,
  [sym_source_file] = sym_source_file,
  [sym_expr] = sym_expr,
  [sym_binexpr] = sym_binexpr,
  [sym_unaryexpr] = sym_unaryexpr,
  [sym_block] = sym_block,
  [sym_if] = sym_if,
  [sym_func] = sym_func,
  [sym__literal] = sym__literal,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
  [aux_sym_func_repeat1] = aux_sym_func_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_PLUS] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_STAR] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_if] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_else] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_PIPE] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_DASH_GT] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [sym_identifier] = {
    .visible = true,
    .named = true,
  },
  [sym_string] = {
    .visible = true,
    .named = true,
  },
  [sym_number] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_expr] = {
    .visible = true,
    .named = true,
  },
  [sym_binexpr] = {
    .visible = true,
    .named = true,
  },
  [sym_unaryexpr] = {
    .visible = true,
    .named = true,
  },
  [sym_block] = {
    .visible = true,
    .named = true,
  },
  [sym_if] = {
    .visible = true,
    .named = true,
  },
  [sym_func] = {
    .visible = true,
    .named = true,
  },
  [sym__literal] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
    .visible = false,
    .named = false,
  },
  [aux_sym_func_repeat1] = {
    .visible = false,
    .named = false,
  },
};

static const TSSymbol ts_alias_sequences[PRODUCTION_ID_COUNT][MAX_ALIAS_SEQUENCE_LENGTH] = {
  [0] = {0},
};

static const uint16_t ts_non_terminal_alias_map[] = {
  0,
};

static const TSStateId ts_primary_state_ids[STATE_COUNT] = {
  [0] = 0,
  [1] = 1,
  [2] = 2,
  [3] = 3,
  [4] = 4,
  [5] = 3,
  [6] = 6,
  [7] = 4,
  [8] = 8,
  [9] = 9,
  [10] = 10,
  [11] = 11,
  [12] = 12,
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 15,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 16,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
  [31] = 31,
  [32] = 32,
  [33] = 33,
  [34] = 34,
  [35] = 35,
  [36] = 36,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(4);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '*') ADVANCE(6);
      if (lookahead == '+') ADVANCE(5);
      if (lookahead == '-') ADVANCE(7);
      if (lookahead == '[') ADVANCE(14);
      if (lookahead == ']') ADVANCE(15);
      if (lookahead == 'e') ADVANCE(18);
      if (lookahead == 'i') ADVANCE(17);
      if (lookahead == '{') ADVANCE(8);
      if (lookahead == '|') ADVANCE(12);
      if (lookahead == '}') ADVANCE(9);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(22);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(21);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '|') ADVANCE(12);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2)
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 3:
      if (eof) ADVANCE(4);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '*') ADVANCE(6);
      if (lookahead == '+') ADVANCE(5);
      if (lookahead == '-') ADVANCE(7);
      if (lookahead == 'i') ADVANCE(17);
      if (lookahead == '{') ADVANCE(8);
      if (lookahead == '|') ADVANCE(12);
      if (lookahead == '}') ADVANCE(9);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(3)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(22);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '>') ADVANCE(13);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_if);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_else);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'e') ADVANCE(11);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'f') ADVANCE(10);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'l') ADVANCE(19);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 's') ADVANCE(16);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(sym_identifier);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(20);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(22);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 3},
  [2] = {.lex_state = 3},
  [3] = {.lex_state = 3},
  [4] = {.lex_state = 3},
  [5] = {.lex_state = 3},
  [6] = {.lex_state = 3},
  [7] = {.lex_state = 3},
  [8] = {.lex_state = 3},
  [9] = {.lex_state = 3},
  [10] = {.lex_state = 3},
  [11] = {.lex_state = 3},
  [12] = {.lex_state = 3},
  [13] = {.lex_state = 3},
  [14] = {.lex_state = 3},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 3},
  [19] = {.lex_state = 3},
  [20] = {.lex_state = 3},
  [21] = {.lex_state = 3},
  [22] = {.lex_state = 3},
  [23] = {.lex_state = 3},
  [24] = {.lex_state = 3},
  [25] = {.lex_state = 3},
  [26] = {.lex_state = 3},
  [27] = {.lex_state = 3},
  [28] = {.lex_state = 3},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 2},
  [31] = {.lex_state = 2},
  [32] = {.lex_state = 2},
  [33] = {.lex_state = 0},
  [34] = {.lex_state = 0},
  [35] = {.lex_state = 0},
  [36] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_PLUS] = ACTIONS(1),
    [anon_sym_STAR] = ACTIONS(1),
    [anon_sym_DASH] = ACTIONS(1),
    [anon_sym_LBRACE] = ACTIONS(1),
    [anon_sym_RBRACE] = ACTIONS(1),
    [anon_sym_if] = ACTIONS(1),
    [anon_sym_else] = ACTIONS(1),
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(36),
    [sym_expr] = STATE(18),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [aux_sym_source_file_repeat1] = STATE(6),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [2] = {
    [sym_expr] = STATE(18),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(19),
    [anon_sym_DASH] = ACTIONS(21),
    [anon_sym_LBRACE] = ACTIONS(24),
    [anon_sym_RBRACE] = ACTIONS(19),
    [anon_sym_if] = ACTIONS(27),
    [anon_sym_PIPE] = ACTIONS(30),
    [anon_sym_DASH_GT] = ACTIONS(33),
    [sym_identifier] = ACTIONS(36),
    [sym_string] = ACTIONS(39),
    [sym_number] = ACTIONS(39),
  },
  [3] = {
    [sym_expr] = STATE(18),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [aux_sym_source_file_repeat1] = STATE(7),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_RBRACE] = ACTIONS(42),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [4] = {
    [sym_expr] = STATE(18),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [aux_sym_source_file_repeat1] = STATE(2),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_RBRACE] = ACTIONS(44),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [5] = {
    [sym_expr] = STATE(18),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [aux_sym_source_file_repeat1] = STATE(4),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_RBRACE] = ACTIONS(46),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [6] = {
    [sym_expr] = STATE(18),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(48),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [7] = {
    [sym_expr] = STATE(18),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [aux_sym_source_file_repeat1] = STATE(2),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_RBRACE] = ACTIONS(50),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [8] = {
    [sym_expr] = STATE(21),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [9] = {
    [sym_expr] = STATE(27),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [10] = {
    [sym_expr] = STATE(29),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [11] = {
    [sym_expr] = STATE(19),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [12] = {
    [sym_expr] = STATE(25),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [13] = {
    [sym_expr] = STATE(23),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [14] = {
    [sym_expr] = STATE(22),
    [sym_binexpr] = STATE(28),
    [sym_unaryexpr] = STATE(28),
    [sym_block] = STATE(28),
    [sym_if] = STATE(28),
    [sym_func] = STATE(28),
    [sym__literal] = STATE(28),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [15] = {
    [ts_builtin_sym_end] = ACTIONS(52),
    [anon_sym_PLUS] = ACTIONS(52),
    [anon_sym_STAR] = ACTIONS(52),
    [anon_sym_DASH] = ACTIONS(54),
    [anon_sym_LBRACE] = ACTIONS(52),
    [anon_sym_RBRACE] = ACTIONS(52),
    [anon_sym_if] = ACTIONS(54),
    [anon_sym_else] = ACTIONS(54),
    [anon_sym_PIPE] = ACTIONS(52),
    [anon_sym_DASH_GT] = ACTIONS(52),
    [sym_identifier] = ACTIONS(54),
    [sym_string] = ACTIONS(52),
    [sym_number] = ACTIONS(52),
  },
  [16] = {
    [ts_builtin_sym_end] = ACTIONS(56),
    [anon_sym_PLUS] = ACTIONS(56),
    [anon_sym_STAR] = ACTIONS(56),
    [anon_sym_DASH] = ACTIONS(58),
    [anon_sym_LBRACE] = ACTIONS(56),
    [anon_sym_RBRACE] = ACTIONS(56),
    [anon_sym_if] = ACTIONS(58),
    [anon_sym_else] = ACTIONS(58),
    [anon_sym_PIPE] = ACTIONS(56),
    [anon_sym_DASH_GT] = ACTIONS(56),
    [sym_identifier] = ACTIONS(58),
    [sym_string] = ACTIONS(56),
    [sym_number] = ACTIONS(56),
  },
  [17] = {
    [ts_builtin_sym_end] = ACTIONS(60),
    [anon_sym_PLUS] = ACTIONS(60),
    [anon_sym_STAR] = ACTIONS(60),
    [anon_sym_DASH] = ACTIONS(62),
    [anon_sym_LBRACE] = ACTIONS(60),
    [anon_sym_RBRACE] = ACTIONS(60),
    [anon_sym_if] = ACTIONS(62),
    [anon_sym_else] = ACTIONS(64),
    [anon_sym_PIPE] = ACTIONS(60),
    [anon_sym_DASH_GT] = ACTIONS(60),
    [sym_identifier] = ACTIONS(62),
    [sym_string] = ACTIONS(60),
    [sym_number] = ACTIONS(60),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 4,
    ACTIONS(68), 1,
      anon_sym_PLUS,
    ACTIONS(70), 1,
      anon_sym_STAR,
    ACTIONS(72), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(66), 7,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [21] = 4,
    ACTIONS(68), 1,
      anon_sym_PLUS,
    ACTIONS(70), 1,
      anon_sym_STAR,
    ACTIONS(76), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(74), 7,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [42] = 2,
    ACTIONS(54), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(52), 9,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [59] = 3,
    ACTIONS(70), 1,
      anon_sym_STAR,
    ACTIONS(80), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(78), 8,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [78] = 2,
    ACTIONS(80), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(78), 9,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [95] = 4,
    ACTIONS(68), 1,
      anon_sym_PLUS,
    ACTIONS(70), 1,
      anon_sym_STAR,
    ACTIONS(84), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(82), 7,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [116] = 2,
    ACTIONS(88), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(86), 9,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [133] = 4,
    ACTIONS(68), 1,
      anon_sym_PLUS,
    ACTIONS(70), 1,
      anon_sym_STAR,
    ACTIONS(92), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(90), 7,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [154] = 2,
    ACTIONS(58), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(56), 9,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [171] = 4,
    ACTIONS(68), 1,
      anon_sym_PLUS,
    ACTIONS(70), 1,
      anon_sym_STAR,
    ACTIONS(96), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(94), 7,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [192] = 2,
    ACTIONS(100), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(98), 9,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [209] = 4,
    ACTIONS(68), 1,
      anon_sym_PLUS,
    ACTIONS(70), 1,
      anon_sym_STAR,
    ACTIONS(102), 1,
      anon_sym_LBRACE,
    STATE(17), 1,
      sym_block,
  [222] = 3,
    ACTIONS(104), 1,
      anon_sym_PIPE,
    ACTIONS(106), 1,
      sym_identifier,
    STATE(32), 1,
      aux_sym_func_repeat1,
  [232] = 3,
    ACTIONS(108), 1,
      anon_sym_PIPE,
    ACTIONS(110), 1,
      sym_identifier,
    STATE(30), 1,
      aux_sym_func_repeat1,
  [242] = 3,
    ACTIONS(112), 1,
      anon_sym_PIPE,
    ACTIONS(114), 1,
      sym_identifier,
    STATE(32), 1,
      aux_sym_func_repeat1,
  [252] = 2,
    ACTIONS(7), 1,
      anon_sym_LBRACE,
    STATE(24), 1,
      sym_block,
  [259] = 1,
    ACTIONS(117), 1,
      anon_sym_DASH_GT,
  [263] = 1,
    ACTIONS(119), 1,
      anon_sym_DASH_GT,
  [267] = 1,
    ACTIONS(121), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(18)] = 0,
  [SMALL_STATE(19)] = 21,
  [SMALL_STATE(20)] = 42,
  [SMALL_STATE(21)] = 59,
  [SMALL_STATE(22)] = 78,
  [SMALL_STATE(23)] = 95,
  [SMALL_STATE(24)] = 116,
  [SMALL_STATE(25)] = 133,
  [SMALL_STATE(26)] = 154,
  [SMALL_STATE(27)] = 171,
  [SMALL_STATE(28)] = 192,
  [SMALL_STATE(29)] = 209,
  [SMALL_STATE(30)] = 222,
  [SMALL_STATE(31)] = 232,
  [SMALL_STATE(32)] = 242,
  [SMALL_STATE(33)] = 252,
  [SMALL_STATE(34)] = 259,
  [SMALL_STATE(35)] = 263,
  [SMALL_STATE(36)] = 267,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(9),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(31),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(28),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [21] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(9),
  [24] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(3),
  [27] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(10),
  [30] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(31),
  [33] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(11),
  [36] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(28),
  [39] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(28),
  [42] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [50] = {.entry = {.count = 1, .reusable = true}}, SHIFT(20),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [54] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3),
  [56] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [58] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2),
  [60] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if, 3),
  [62] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if, 3),
  [64] = {.entry = {.count = 1, .reusable = false}}, SHIFT(33),
  [66] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 1),
  [68] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [70] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [72] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 1),
  [74] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func, 2),
  [76] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func, 2),
  [78] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binexpr, 3),
  [80] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binexpr, 3),
  [82] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func, 4),
  [84] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func, 4),
  [86] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if, 5),
  [88] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if, 5),
  [90] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func, 5),
  [92] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func, 5),
  [94] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unaryexpr, 2),
  [96] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unaryexpr, 2),
  [98] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr, 1),
  [100] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expr, 1),
  [102] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [104] = {.entry = {.count = 1, .reusable = true}}, SHIFT(35),
  [106] = {.entry = {.count = 1, .reusable = true}}, SHIFT(32),
  [108] = {.entry = {.count = 1, .reusable = true}}, SHIFT(34),
  [110] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [112] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_repeat1, 2),
  [114] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_func_repeat1, 2), SHIFT_REPEAT(32),
  [117] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [119] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [121] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef _WIN32
#define extern __declspec(dllexport)
#endif

extern const TSLanguage *tree_sitter_dyn(void) {
  static const TSLanguage language = {
    .version = LANGUAGE_VERSION,
    .symbol_count = SYMBOL_COUNT,
    .alias_count = ALIAS_COUNT,
    .token_count = TOKEN_COUNT,
    .external_token_count = EXTERNAL_TOKEN_COUNT,
    .state_count = STATE_COUNT,
    .large_state_count = LARGE_STATE_COUNT,
    .production_id_count = PRODUCTION_ID_COUNT,
    .field_count = FIELD_COUNT,
    .max_alias_sequence_length = MAX_ALIAS_SEQUENCE_LENGTH,
    .parse_table = &ts_parse_table[0][0],
    .small_parse_table = ts_small_parse_table,
    .small_parse_table_map = ts_small_parse_table_map,
    .parse_actions = ts_parse_actions,
    .symbol_names = ts_symbol_names,
    .symbol_metadata = ts_symbol_metadata,
    .public_symbol_map = ts_symbol_map,
    .alias_map = ts_non_terminal_alias_map,
    .alias_sequences = &ts_alias_sequences[0][0],
    .lex_modes = ts_lex_modes,
    .lex_fn = ts_lex,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
