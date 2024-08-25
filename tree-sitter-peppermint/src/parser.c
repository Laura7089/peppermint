#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 13
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 16
#define ALIAS_COUNT 0
#define TOKEN_COUNT 9
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym_opcode = 1,
  sym__whitespace = 2,
  sym_comment = 3,
  anon_sym_LBRACK = 4,
  anon_sym_RBRACK = 5,
  sym__number = 6,
  sym_label = 7,
  sym_label_jump = 8,
  sym_source_file = 9,
  sym_statement = 10,
  sym_instruction = 11,
  sym_operand = 12,
  sym_literal = 13,
  sym_address = 14,
  aux_sym_source_file_repeat1 = 15,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym_opcode] = "opcode",
  [sym__whitespace] = "_whitespace",
  [sym_comment] = "comment",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [sym__number] = "_number",
  [sym_label] = "label",
  [sym_label_jump] = "label_jump",
  [sym_source_file] = "source_file",
  [sym_statement] = "statement",
  [sym_instruction] = "instruction",
  [sym_operand] = "operand",
  [sym_literal] = "literal",
  [sym_address] = "address",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym_opcode] = sym_opcode,
  [sym__whitespace] = sym__whitespace,
  [sym_comment] = sym_comment,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [sym__number] = sym__number,
  [sym_label] = sym_label,
  [sym_label_jump] = sym_label_jump,
  [sym_source_file] = sym_source_file,
  [sym_statement] = sym_statement,
  [sym_instruction] = sym_instruction,
  [sym_operand] = sym_operand,
  [sym_literal] = sym_literal,
  [sym_address] = sym_address,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym_opcode] = {
    .visible = true,
    .named = true,
  },
  [sym__whitespace] = {
    .visible = false,
    .named = true,
  },
  [sym_comment] = {
    .visible = true,
    .named = true,
  },
  [anon_sym_LBRACK] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_RBRACK] = {
    .visible = true,
    .named = false,
  },
  [sym__number] = {
    .visible = false,
    .named = true,
  },
  [sym_label] = {
    .visible = true,
    .named = true,
  },
  [sym_label_jump] = {
    .visible = true,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_instruction] = {
    .visible = true,
    .named = true,
  },
  [sym_operand] = {
    .visible = true,
    .named = true,
  },
  [sym_literal] = {
    .visible = true,
    .named = true,
  },
  [sym_address] = {
    .visible = true,
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
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(5);
      ADVANCE_MAP(
        '0', 13,
        ':', 4,
        '[', 10,
        ']', 11,
        '#', 7,
        ';', 7,
        '\t', 6,
        '\n', 6,
        ' ', 6,
      );
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(14);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(8);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(9);
      END_STATE();
    case 1:
      if (lookahead == '0') ADVANCE(13);
      if (lookahead == '#' ||
          lookahead == ';') ADVANCE(7);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == ' ') ADVANCE(6);
      if (('1' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(14);
      END_STATE();
    case 2:
      if (lookahead == ':') ADVANCE(15);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(2);
      END_STATE();
    case 3:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(14);
      END_STATE();
    case 4:
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(16);
      END_STATE();
    case 5:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 6:
      ACCEPT_TOKEN(sym__whitespace);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == ' ') ADVANCE(6);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(sym_comment);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(7);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(sym_opcode);
      if (lookahead == ':') ADVANCE(15);
      if (lookahead == '-' ||
          lookahead == '_') ADVANCE(2);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(12);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(8);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(9);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(sym_opcode);
      if (lookahead == ':') ADVANCE(15);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(2);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(9);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym__number);
      if (lookahead == ':') ADVANCE(15);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(12);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(2);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym__number);
      if (lookahead == 'b') ADVANCE(14);
      if (lookahead == 'x') ADVANCE(3);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(14);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym__number);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(14);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym_label);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym_label_jump);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(16);
      END_STATE();
    default:
      return false;
  }
}

static bool ts_lex_keywords(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      ACCEPT_TOKEN(ts_builtin_sym_end);
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
  [11] = {.lex_state = 1},
  [12] = {.lex_state = 0},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [sym_opcode] = ACTIONS(1),
    [sym__whitespace] = ACTIONS(3),
    [sym_comment] = ACTIONS(3),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [sym__number] = ACTIONS(1),
    [sym_label] = ACTIONS(1),
    [sym_label_jump] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(10),
    [sym_statement] = STATE(2),
    [sym_instruction] = STATE(6),
    [sym_literal] = STATE(6),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(5),
    [sym_opcode] = ACTIONS(7),
    [sym__whitespace] = ACTIONS(3),
    [sym_comment] = ACTIONS(3),
    [sym__number] = ACTIONS(9),
    [sym_label] = ACTIONS(11),
  },
  [2] = {
    [sym_statement] = STATE(3),
    [sym_instruction] = STATE(6),
    [sym_literal] = STATE(6),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(13),
    [sym_opcode] = ACTIONS(7),
    [sym__whitespace] = ACTIONS(3),
    [sym_comment] = ACTIONS(3),
    [sym__number] = ACTIONS(9),
    [sym_label] = ACTIONS(15),
  },
  [3] = {
    [sym_statement] = STATE(3),
    [sym_instruction] = STATE(6),
    [sym_literal] = STATE(6),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(17),
    [sym_opcode] = ACTIONS(19),
    [sym__whitespace] = ACTIONS(3),
    [sym_comment] = ACTIONS(3),
    [sym__number] = ACTIONS(22),
    [sym_label] = ACTIONS(25),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 5,
    ACTIONS(28), 1,
      anon_sym_LBRACK,
    ACTIONS(30), 1,
      sym_label_jump,
    STATE(7), 1,
      sym_address,
    STATE(8), 1,
      sym_operand,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [17] = 3,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(32), 2,
      ts_builtin_sym_end,
      sym_label,
    ACTIONS(34), 2,
      sym_opcode,
      sym__number,
  [30] = 3,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(36), 2,
      ts_builtin_sym_end,
      sym_label,
    ACTIONS(38), 2,
      sym_opcode,
      sym__number,
  [43] = 3,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(40), 2,
      ts_builtin_sym_end,
      sym_label,
    ACTIONS(42), 2,
      sym_opcode,
      sym__number,
  [56] = 3,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(44), 2,
      ts_builtin_sym_end,
      sym_label,
    ACTIONS(46), 2,
      sym_opcode,
      sym__number,
  [69] = 3,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
    ACTIONS(48), 2,
      ts_builtin_sym_end,
      sym_label,
    ACTIONS(50), 2,
      sym_opcode,
      sym__number,
  [82] = 2,
    ACTIONS(52), 1,
      ts_builtin_sym_end,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [90] = 2,
    ACTIONS(54), 1,
      sym__number,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
  [98] = 2,
    ACTIONS(56), 1,
      anon_sym_RBRACK,
    ACTIONS(3), 2,
      sym__whitespace,
      sym_comment,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 17,
  [SMALL_STATE(6)] = 30,
  [SMALL_STATE(7)] = 43,
  [SMALL_STATE(8)] = 56,
  [SMALL_STATE(9)] = 69,
  [SMALL_STATE(10)] = 82,
  [SMALL_STATE(11)] = 90,
  [SMALL_STATE(12)] = 98,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, SHIFT_EXTRA(),
  [5] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(4),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(5),
  [11] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [13] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [15] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [17] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [19] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(4),
  [22] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(5),
  [25] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(3),
  [28] = {.entry = {.count = 1, .reusable = true}}, SHIFT(11),
  [30] = {.entry = {.count = 1, .reusable = true}}, SHIFT(7),
  [32] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_literal, 1, 0, 0),
  [34] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_literal, 1, 0, 0),
  [36] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 1, 0, 0),
  [38] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_statement, 1, 0, 0),
  [40] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_operand, 1, 0, 0),
  [42] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_operand, 1, 0, 0),
  [44] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 2, 0, 0),
  [46] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 2, 0, 0),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_address, 3, 0, 0),
  [50] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_address, 3, 0, 0),
  [52] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [54] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
  [56] = {.entry = {.count = 1, .reusable = true}}, SHIFT(9),
};

#ifdef __cplusplus
extern "C" {
#endif
#ifdef TREE_SITTER_HIDE_SYMBOLS
#define TS_PUBLIC
#elif defined(_WIN32)
#define TS_PUBLIC __declspec(dllexport)
#else
#define TS_PUBLIC __attribute__((visibility("default")))
#endif

TS_PUBLIC const TSLanguage *tree_sitter_peppermint(void) {
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
    .keyword_lex_fn = ts_lex_keywords,
    .keyword_capture_token = sym_opcode,
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
