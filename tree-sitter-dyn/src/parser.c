#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 31
#define LARGE_STATE_COUNT 13
#define SYMBOL_COUNT 24
#define ALIAS_COUNT 0
#define TOKEN_COUNT 14
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
  anon_sym_PIPE = 7,
  anon_sym_DASH_GT = 8,
  anon_sym_LBRACK = 9,
  anon_sym_RBRACK = 10,
  sym_identifier = 11,
  sym_string = 12,
  sym_number = 13,
  sym_source_file = 14,
  sym_expr = 15,
  sym_binexpr = 16,
  sym_unaryexpr = 17,
  sym_block = 18,
  sym_if = 19,
  sym_func = 20,
  sym__literal = 21,
  aux_sym_source_file_repeat1 = 22,
  aux_sym_func_repeat1 = 23,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_PLUS] = "+",
  [anon_sym_STAR] = "*",
  [anon_sym_DASH] = "-",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_if] = "if",
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
  [5] = 5,
  [6] = 6,
  [7] = 7,
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
  [20] = 20,
  [21] = 21,
  [22] = 22,
  [23] = 23,
  [24] = 24,
  [25] = 25,
  [26] = 26,
  [27] = 27,
  [28] = 28,
  [29] = 29,
  [30] = 30,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(3);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '*') ADVANCE(5);
      if (lookahead == '+') ADVANCE(4);
      if (lookahead == '-') ADVANCE(6);
      if (lookahead == '[') ADVANCE(12);
      if (lookahead == ']') ADVANCE(13);
      if (lookahead == 'i') ADVANCE(14);
      if (lookahead == '{') ADVANCE(7);
      if (lookahead == '|') ADVANCE(10);
      if (lookahead == '}') ADVANCE(8);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(17);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(15);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(16);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      if (lookahead == '|') ADVANCE(10);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2)
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(15);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_DASH);
      if (lookahead == '>') ADVANCE(11);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_if);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(15);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_PIPE);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_DASH_GT);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'f') ADVANCE(9);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(15);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym_identifier);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(15);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(17);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 0},
  [2] = {.lex_state = 0},
  [3] = {.lex_state = 0},
  [4] = {.lex_state = 0},
  [5] = {.lex_state = 0},
  [6] = {.lex_state = 0},
  [7] = {.lex_state = 0},
  [8] = {.lex_state = 0},
  [9] = {.lex_state = 0},
  [10] = {.lex_state = 0},
  [11] = {.lex_state = 0},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 0},
  [14] = {.lex_state = 0},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 0},
  [17] = {.lex_state = 0},
  [18] = {.lex_state = 0},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 0},
  [22] = {.lex_state = 0},
  [23] = {.lex_state = 0},
  [24] = {.lex_state = 0},
  [25] = {.lex_state = 2},
  [26] = {.lex_state = 2},
  [27] = {.lex_state = 2},
  [28] = {.lex_state = 0},
  [29] = {.lex_state = 0},
  [30] = {.lex_state = 0},
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
    [anon_sym_PIPE] = ACTIONS(1),
    [anon_sym_DASH_GT] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(29),
    [sym_expr] = STATE(17),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
    [aux_sym_source_file_repeat1] = STATE(5),
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
    [sym_expr] = STATE(17),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
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
    [sym_expr] = STATE(17),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
    [aux_sym_source_file_repeat1] = STATE(2),
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
    [sym_expr] = STATE(17),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
    [aux_sym_source_file_repeat1] = STATE(3),
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
    [sym_expr] = STATE(17),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(46),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [6] = {
    [sym_expr] = STATE(16),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
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
    [sym_expr] = STATE(24),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
  [8] = {
    [sym_expr] = STATE(23),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
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
    [sym_expr] = STATE(14),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
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
    [sym_expr] = STATE(21),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
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
    [sym_expr] = STATE(20),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
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
    [sym_expr] = STATE(19),
    [sym_binexpr] = STATE(22),
    [sym_unaryexpr] = STATE(22),
    [sym_block] = STATE(22),
    [sym_if] = STATE(22),
    [sym_func] = STATE(22),
    [sym__literal] = STATE(22),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [anon_sym_PIPE] = ACTIONS(11),
    [anon_sym_DASH_GT] = ACTIONS(13),
    [sym_identifier] = ACTIONS(15),
    [sym_string] = ACTIONS(17),
    [sym_number] = ACTIONS(17),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(50), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(48), 9,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [17] = 4,
    ACTIONS(54), 1,
      anon_sym_PLUS,
    ACTIONS(56), 1,
      anon_sym_STAR,
    ACTIONS(58), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(52), 7,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [38] = 2,
    ACTIONS(62), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(60), 9,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [55] = 4,
    ACTIONS(54), 1,
      anon_sym_PLUS,
    ACTIONS(56), 1,
      anon_sym_STAR,
    ACTIONS(66), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(64), 7,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [76] = 4,
    ACTIONS(54), 1,
      anon_sym_PLUS,
    ACTIONS(56), 1,
      anon_sym_STAR,
    ACTIONS(70), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(68), 7,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [97] = 2,
    ACTIONS(74), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(72), 9,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [114] = 3,
    ACTIONS(56), 1,
      anon_sym_STAR,
    ACTIONS(78), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(76), 8,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [133] = 2,
    ACTIONS(78), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(76), 9,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [150] = 4,
    ACTIONS(54), 1,
      anon_sym_PLUS,
    ACTIONS(56), 1,
      anon_sym_STAR,
    ACTIONS(82), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(80), 7,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [171] = 2,
    ACTIONS(86), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(84), 9,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [188] = 4,
    ACTIONS(54), 1,
      anon_sym_PLUS,
    ACTIONS(56), 1,
      anon_sym_STAR,
    ACTIONS(90), 3,
      anon_sym_DASH,
      anon_sym_if,
      sym_identifier,
    ACTIONS(88), 7,
      ts_builtin_sym_end,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      anon_sym_PIPE,
      anon_sym_DASH_GT,
      sym_string,
      sym_number,
  [209] = 4,
    ACTIONS(7), 1,
      anon_sym_LBRACE,
    ACTIONS(54), 1,
      anon_sym_PLUS,
    ACTIONS(56), 1,
      anon_sym_STAR,
    STATE(18), 1,
      sym_block,
  [222] = 3,
    ACTIONS(92), 1,
      anon_sym_PIPE,
    ACTIONS(94), 1,
      sym_identifier,
    STATE(27), 1,
      aux_sym_func_repeat1,
  [232] = 3,
    ACTIONS(96), 1,
      anon_sym_PIPE,
    ACTIONS(98), 1,
      sym_identifier,
    STATE(25), 1,
      aux_sym_func_repeat1,
  [242] = 3,
    ACTIONS(100), 1,
      anon_sym_PIPE,
    ACTIONS(102), 1,
      sym_identifier,
    STATE(27), 1,
      aux_sym_func_repeat1,
  [252] = 1,
    ACTIONS(105), 1,
      anon_sym_DASH_GT,
  [256] = 1,
    ACTIONS(107), 1,
      ts_builtin_sym_end,
  [260] = 1,
    ACTIONS(109), 1,
      anon_sym_DASH_GT,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(13)] = 0,
  [SMALL_STATE(14)] = 17,
  [SMALL_STATE(15)] = 38,
  [SMALL_STATE(16)] = 55,
  [SMALL_STATE(17)] = 76,
  [SMALL_STATE(18)] = 97,
  [SMALL_STATE(19)] = 114,
  [SMALL_STATE(20)] = 133,
  [SMALL_STATE(21)] = 150,
  [SMALL_STATE(22)] = 171,
  [SMALL_STATE(23)] = 188,
  [SMALL_STATE(24)] = 209,
  [SMALL_STATE(25)] = 222,
  [SMALL_STATE(26)] = 232,
  [SMALL_STATE(27)] = 242,
  [SMALL_STATE(28)] = 252,
  [SMALL_STATE(29)] = 256,
  [SMALL_STATE(30)] = 260,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = false}}, SHIFT(6),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(4),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(26),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [15] = {.entry = {.count = 1, .reusable = false}}, SHIFT(22),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(22),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [21] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(6),
  [24] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(4),
  [27] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(7),
  [30] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(26),
  [33] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(9),
  [36] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(22),
  [39] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(22),
  [42] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(15),
  [46] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [50] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func, 2),
  [54] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [56] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [58] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func, 2),
  [60] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [62] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2),
  [64] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unaryexpr, 2),
  [66] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unaryexpr, 2),
  [68] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 1),
  [70] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 1),
  [72] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if, 3),
  [74] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if, 3),
  [76] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binexpr, 3),
  [78] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binexpr, 3),
  [80] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func, 4),
  [82] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func, 4),
  [84] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr, 1),
  [86] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expr, 1),
  [88] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_func, 5),
  [90] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_func, 5),
  [92] = {.entry = {.count = 1, .reusable = true}}, SHIFT(30),
  [94] = {.entry = {.count = 1, .reusable = true}}, SHIFT(27),
  [96] = {.entry = {.count = 1, .reusable = true}}, SHIFT(28),
  [98] = {.entry = {.count = 1, .reusable = true}}, SHIFT(25),
  [100] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_func_repeat1, 2),
  [102] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_func_repeat1, 2), SHIFT_REPEAT(27),
  [105] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [107] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [109] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
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
