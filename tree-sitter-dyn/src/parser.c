#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 20
#define LARGE_STATE_COUNT 10
#define SYMBOL_COUNT 20
#define ALIAS_COUNT 0
#define TOKEN_COUNT 12
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  anon_sym_PLUS = 1,
  anon_sym_STAR = 2,
  anon_sym_DASH = 3,
  anon_sym_LBRACE = 4,
  anon_sym_RBRACE = 5,
  anon_sym_if = 6,
  anon_sym_LBRACK = 7,
  anon_sym_RBRACK = 8,
  sym_identifier = 9,
  sym_string = 10,
  sym_number = 11,
  sym_source_file = 12,
  sym_expr = 13,
  sym_binexpr = 14,
  sym_unaryexpr = 15,
  sym_block = 16,
  sym_if = 17,
  sym__literal = 18,
  aux_sym_source_file_repeat1 = 19,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [anon_sym_PLUS] = "+",
  [anon_sym_STAR] = "*",
  [anon_sym_DASH] = "-",
  [anon_sym_LBRACE] = "{",
  [anon_sym_RBRACE] = "}",
  [anon_sym_if] = "if",
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
  [sym__literal] = "_literal",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [anon_sym_PLUS] = anon_sym_PLUS,
  [anon_sym_STAR] = anon_sym_STAR,
  [anon_sym_DASH] = anon_sym_DASH,
  [anon_sym_LBRACE] = anon_sym_LBRACE,
  [anon_sym_RBRACE] = anon_sym_RBRACE,
  [anon_sym_if] = anon_sym_if,
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
  [sym__literal] = sym__literal,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
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
  [sym__literal] = {
    .visible = false,
    .named = true,
  },
  [aux_sym_source_file_repeat1] = {
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
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(2);
      if (lookahead == '"') ADVANCE(1);
      if (lookahead == '*') ADVANCE(4);
      if (lookahead == '+') ADVANCE(3);
      if (lookahead == '-') ADVANCE(5);
      if (lookahead == '[') ADVANCE(9);
      if (lookahead == ']') ADVANCE(10);
      if (lookahead == 'i') ADVANCE(11);
      if (lookahead == '{') ADVANCE(6);
      if (lookahead == '}') ADVANCE(7);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0)
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(14);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(12);
      END_STATE();
    case 1:
      if (lookahead == '"') ADVANCE(13);
      if (lookahead != 0) ADVANCE(1);
      END_STATE();
    case 2:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 3:
      ACCEPT_TOKEN(anon_sym_PLUS);
      END_STATE();
    case 4:
      ACCEPT_TOKEN(anon_sym_STAR);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(anon_sym_DASH);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(anon_sym_LBRACE);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(anon_sym_RBRACE);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(anon_sym_if);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(12);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym_identifier);
      if (lookahead == 'f') ADVANCE(8);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(12);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym_identifier);
      if (('a' <= lookahead && lookahead <= 'z')) ADVANCE(12);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_string);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_number);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(14);
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
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [sym_identifier] = ACTIONS(1),
    [sym_string] = ACTIONS(1),
    [sym_number] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(19),
    [sym_expr] = STATE(11),
    [sym_binexpr] = STATE(10),
    [sym_unaryexpr] = STATE(10),
    [sym_block] = STATE(10),
    [sym_if] = STATE(10),
    [sym__literal] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(4),
    [ts_builtin_sym_end] = ACTIONS(3),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [sym_identifier] = ACTIONS(11),
    [sym_string] = ACTIONS(13),
    [sym_number] = ACTIONS(13),
  },
  [2] = {
    [sym_expr] = STATE(11),
    [sym_binexpr] = STATE(10),
    [sym_unaryexpr] = STATE(10),
    [sym_block] = STATE(10),
    [sym_if] = STATE(10),
    [sym__literal] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(15),
    [anon_sym_DASH] = ACTIONS(17),
    [anon_sym_LBRACE] = ACTIONS(20),
    [anon_sym_RBRACE] = ACTIONS(15),
    [anon_sym_if] = ACTIONS(23),
    [sym_identifier] = ACTIONS(26),
    [sym_string] = ACTIONS(29),
    [sym_number] = ACTIONS(29),
  },
  [3] = {
    [sym_expr] = STATE(11),
    [sym_binexpr] = STATE(10),
    [sym_unaryexpr] = STATE(10),
    [sym_block] = STATE(10),
    [sym_if] = STATE(10),
    [sym__literal] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(5),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_RBRACE] = ACTIONS(32),
    [anon_sym_if] = ACTIONS(9),
    [sym_identifier] = ACTIONS(11),
    [sym_string] = ACTIONS(13),
    [sym_number] = ACTIONS(13),
  },
  [4] = {
    [sym_expr] = STATE(11),
    [sym_binexpr] = STATE(10),
    [sym_unaryexpr] = STATE(10),
    [sym_block] = STATE(10),
    [sym_if] = STATE(10),
    [sym__literal] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(34),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [sym_identifier] = ACTIONS(11),
    [sym_string] = ACTIONS(13),
    [sym_number] = ACTIONS(13),
  },
  [5] = {
    [sym_expr] = STATE(11),
    [sym_binexpr] = STATE(10),
    [sym_unaryexpr] = STATE(10),
    [sym_block] = STATE(10),
    [sym_if] = STATE(10),
    [sym__literal] = STATE(10),
    [aux_sym_source_file_repeat1] = STATE(2),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_RBRACE] = ACTIONS(36),
    [anon_sym_if] = ACTIONS(9),
    [sym_identifier] = ACTIONS(11),
    [sym_string] = ACTIONS(13),
    [sym_number] = ACTIONS(13),
  },
  [6] = {
    [sym_expr] = STATE(12),
    [sym_binexpr] = STATE(10),
    [sym_unaryexpr] = STATE(10),
    [sym_block] = STATE(10),
    [sym_if] = STATE(10),
    [sym__literal] = STATE(10),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [sym_identifier] = ACTIONS(11),
    [sym_string] = ACTIONS(13),
    [sym_number] = ACTIONS(13),
  },
  [7] = {
    [sym_expr] = STATE(18),
    [sym_binexpr] = STATE(10),
    [sym_unaryexpr] = STATE(10),
    [sym_block] = STATE(10),
    [sym_if] = STATE(10),
    [sym__literal] = STATE(10),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [sym_identifier] = ACTIONS(11),
    [sym_string] = ACTIONS(13),
    [sym_number] = ACTIONS(13),
  },
  [8] = {
    [sym_expr] = STATE(16),
    [sym_binexpr] = STATE(10),
    [sym_unaryexpr] = STATE(10),
    [sym_block] = STATE(10),
    [sym_if] = STATE(10),
    [sym__literal] = STATE(10),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [sym_identifier] = ACTIONS(11),
    [sym_string] = ACTIONS(13),
    [sym_number] = ACTIONS(13),
  },
  [9] = {
    [sym_expr] = STATE(17),
    [sym_binexpr] = STATE(10),
    [sym_unaryexpr] = STATE(10),
    [sym_block] = STATE(10),
    [sym_if] = STATE(10),
    [sym__literal] = STATE(10),
    [anon_sym_DASH] = ACTIONS(5),
    [anon_sym_LBRACE] = ACTIONS(7),
    [anon_sym_if] = ACTIONS(9),
    [sym_identifier] = ACTIONS(11),
    [sym_string] = ACTIONS(13),
    [sym_number] = ACTIONS(13),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(40), 2,
      anon_sym_if,
      sym_identifier,
    ACTIONS(38), 8,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      sym_string,
      sym_number,
  [15] = 4,
    ACTIONS(44), 1,
      anon_sym_PLUS,
    ACTIONS(46), 1,
      anon_sym_STAR,
    ACTIONS(48), 2,
      anon_sym_if,
      sym_identifier,
    ACTIONS(42), 6,
      ts_builtin_sym_end,
      anon_sym_DASH,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      sym_string,
      sym_number,
  [34] = 4,
    ACTIONS(44), 1,
      anon_sym_PLUS,
    ACTIONS(46), 1,
      anon_sym_STAR,
    ACTIONS(52), 2,
      anon_sym_if,
      sym_identifier,
    ACTIONS(50), 6,
      ts_builtin_sym_end,
      anon_sym_DASH,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      sym_string,
      sym_number,
  [53] = 2,
    ACTIONS(56), 2,
      anon_sym_if,
      sym_identifier,
    ACTIONS(54), 8,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      sym_string,
      sym_number,
  [68] = 2,
    ACTIONS(60), 2,
      anon_sym_if,
      sym_identifier,
    ACTIONS(58), 8,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      sym_string,
      sym_number,
  [83] = 2,
    ACTIONS(64), 2,
      anon_sym_if,
      sym_identifier,
    ACTIONS(62), 8,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      sym_string,
      sym_number,
  [98] = 3,
    ACTIONS(46), 1,
      anon_sym_STAR,
    ACTIONS(68), 2,
      anon_sym_if,
      sym_identifier,
    ACTIONS(66), 7,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_DASH,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      sym_string,
      sym_number,
  [115] = 2,
    ACTIONS(68), 2,
      anon_sym_if,
      sym_identifier,
    ACTIONS(66), 8,
      ts_builtin_sym_end,
      anon_sym_PLUS,
      anon_sym_STAR,
      anon_sym_DASH,
      anon_sym_LBRACE,
      anon_sym_RBRACE,
      sym_string,
      sym_number,
  [130] = 4,
    ACTIONS(7), 1,
      anon_sym_LBRACE,
    ACTIONS(44), 1,
      anon_sym_PLUS,
    ACTIONS(46), 1,
      anon_sym_STAR,
    STATE(15), 1,
      sym_block,
  [143] = 1,
    ACTIONS(70), 1,
      ts_builtin_sym_end,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(10)] = 0,
  [SMALL_STATE(11)] = 15,
  [SMALL_STATE(12)] = 34,
  [SMALL_STATE(13)] = 53,
  [SMALL_STATE(14)] = 68,
  [SMALL_STATE(15)] = 83,
  [SMALL_STATE(16)] = 98,
  [SMALL_STATE(17)] = 115,
  [SMALL_STATE(18)] = 130,
  [SMALL_STATE(19)] = 143,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [7] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(10),
  [13] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2),
  [17] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(6),
  [20] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(3),
  [23] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(7),
  [26] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(10),
  [29] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2), SHIFT_REPEAT(10),
  [32] = {.entry = {.count = 1, .reusable = true}}, SHIFT(13),
  [34] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1),
  [36] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [38] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_expr, 1),
  [40] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_expr, 1),
  [42] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 1),
  [44] = {.entry = {.count = 1, .reusable = true}}, SHIFT(8),
  [46] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
  [48] = {.entry = {.count = 1, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 1),
  [50] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_unaryexpr, 2),
  [52] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_unaryexpr, 2),
  [54] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 2),
  [56] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 2),
  [58] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_block, 3),
  [60] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_block, 3),
  [62] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_if, 3),
  [64] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_if, 3),
  [66] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_binexpr, 3),
  [68] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_binexpr, 3),
  [70] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
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
