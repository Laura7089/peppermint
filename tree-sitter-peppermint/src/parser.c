#include "tree_sitter/parser.h"

#if defined(__GNUC__) || defined(__clang__)
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif

#define LANGUAGE_VERSION 14
#define STATE_COUNT 22
#define LARGE_STATE_COUNT 4
#define SYMBOL_COUNT 22
#define ALIAS_COUNT 0
#define TOKEN_COUNT 11
#define EXTERNAL_TOKEN_COUNT 0
#define FIELD_COUNT 0
#define MAX_ALIAS_SEQUENCE_LENGTH 3
#define PRODUCTION_ID_COUNT 1

enum ts_symbol_identifiers {
  sym__whitespace = 1,
  anon_sym_SEMI = 2,
  anon_sym_POUND = 3,
  sym__comment_text = 4,
  sym_opcode = 5,
  anon_sym_LBRACK = 6,
  anon_sym_RBRACK = 7,
  sym__number = 8,
  anon_sym_COLON = 9,
  sym__label_name = 10,
  sym_source_file = 11,
  sym__object = 12,
  sym_statement = 13,
  sym_comment = 14,
  sym_instruction = 15,
  sym_operand = 16,
  sym_literal = 17,
  sym_address = 18,
  sym_label = 19,
  sym_label_jump = 20,
  aux_sym_source_file_repeat1 = 21,
};

static const char * const ts_symbol_names[] = {
  [ts_builtin_sym_end] = "end",
  [sym__whitespace] = "_whitespace",
  [anon_sym_SEMI] = ";",
  [anon_sym_POUND] = "#",
  [sym__comment_text] = "_comment_text",
  [sym_opcode] = "opcode",
  [anon_sym_LBRACK] = "[",
  [anon_sym_RBRACK] = "]",
  [sym__number] = "_number",
  [anon_sym_COLON] = ":",
  [sym__label_name] = "_label_name",
  [sym_source_file] = "source_file",
  [sym__object] = "_object",
  [sym_statement] = "statement",
  [sym_comment] = "comment",
  [sym_instruction] = "instruction",
  [sym_operand] = "operand",
  [sym_literal] = "literal",
  [sym_address] = "address",
  [sym_label] = "label",
  [sym_label_jump] = "label_jump",
  [aux_sym_source_file_repeat1] = "source_file_repeat1",
};

static const TSSymbol ts_symbol_map[] = {
  [ts_builtin_sym_end] = ts_builtin_sym_end,
  [sym__whitespace] = sym__whitespace,
  [anon_sym_SEMI] = anon_sym_SEMI,
  [anon_sym_POUND] = anon_sym_POUND,
  [sym__comment_text] = sym__comment_text,
  [sym_opcode] = sym_opcode,
  [anon_sym_LBRACK] = anon_sym_LBRACK,
  [anon_sym_RBRACK] = anon_sym_RBRACK,
  [sym__number] = sym__number,
  [anon_sym_COLON] = anon_sym_COLON,
  [sym__label_name] = sym__label_name,
  [sym_source_file] = sym_source_file,
  [sym__object] = sym__object,
  [sym_statement] = sym_statement,
  [sym_comment] = sym_comment,
  [sym_instruction] = sym_instruction,
  [sym_operand] = sym_operand,
  [sym_literal] = sym_literal,
  [sym_address] = sym_address,
  [sym_label] = sym_label,
  [sym_label_jump] = sym_label_jump,
  [aux_sym_source_file_repeat1] = aux_sym_source_file_repeat1,
};

static const TSSymbolMetadata ts_symbol_metadata[] = {
  [ts_builtin_sym_end] = {
    .visible = false,
    .named = true,
  },
  [sym__whitespace] = {
    .visible = false,
    .named = true,
  },
  [anon_sym_SEMI] = {
    .visible = true,
    .named = false,
  },
  [anon_sym_POUND] = {
    .visible = true,
    .named = false,
  },
  [sym__comment_text] = {
    .visible = false,
    .named = true,
  },
  [sym_opcode] = {
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
  [anon_sym_COLON] = {
    .visible = true,
    .named = false,
  },
  [sym__label_name] = {
    .visible = false,
    .named = true,
  },
  [sym_source_file] = {
    .visible = true,
    .named = true,
  },
  [sym__object] = {
    .visible = false,
    .named = true,
  },
  [sym_statement] = {
    .visible = true,
    .named = true,
  },
  [sym_comment] = {
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
  [sym_label] = {
    .visible = true,
    .named = true,
  },
  [sym_label_jump] = {
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
  [13] = 13,
  [14] = 14,
  [15] = 15,
  [16] = 16,
  [17] = 17,
  [18] = 18,
  [19] = 19,
  [20] = 20,
  [21] = 21,
};

static bool ts_lex(TSLexer *lexer, TSStateId state) {
  START_LEXER();
  eof = lexer->eof(lexer);
  switch (state) {
    case 0:
      if (eof) ADVANCE(7);
      if (lookahead == '#') ADVANCE(10);
      if (lookahead == '0') ADVANCE(19);
      if (lookahead == ':') ADVANCE(22);
      if (lookahead == ';') ADVANCE(9);
      if (lookahead == '[') ADVANCE(17);
      if (lookahead == ']') ADVANCE(18);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(0);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(20);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(13);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(15);
      END_STATE();
    case 1:
      if (lookahead == '0') ADVANCE(19);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(1);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(20);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(14);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(16);
      END_STATE();
    case 2:
      if (lookahead == '0') ADVANCE(19);
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(2);
      if (('1' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(20);
      END_STATE();
    case 3:
      if (('\t' <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') SKIP(3);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(5);
      END_STATE();
    case 4:
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(20);
      END_STATE();
    case 5:
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(23);
      END_STATE();
    case 6:
      if (eof) ADVANCE(7);
      if (lookahead == '#') ADVANCE(10);
      if (lookahead == '0') ADVANCE(19);
      if (lookahead == ';') ADVANCE(9);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == ' ') ADVANCE(8);
      if ((0x0b <= lookahead && lookahead <= '\r')) SKIP(6);
      if (('1' <= lookahead && lookahead <= '9')) ADVANCE(20);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(13);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(15);
      END_STATE();
    case 7:
      ACCEPT_TOKEN(ts_builtin_sym_end);
      END_STATE();
    case 8:
      ACCEPT_TOKEN(sym__whitespace);
      if (lookahead == '\t' ||
          lookahead == '\n' ||
          lookahead == ' ') ADVANCE(8);
      END_STATE();
    case 9:
      ACCEPT_TOKEN(anon_sym_SEMI);
      END_STATE();
    case 10:
      ACCEPT_TOKEN(anon_sym_POUND);
      END_STATE();
    case 11:
      ACCEPT_TOKEN(sym__comment_text);
      if (lookahead == '\t' ||
          (0x0b <= lookahead && lookahead <= '\r') ||
          lookahead == ' ') ADVANCE(11);
      if (lookahead != 0 &&
          (lookahead < '\t' || '\r' < lookahead)) ADVANCE(12);
      END_STATE();
    case 12:
      ACCEPT_TOKEN(sym__comment_text);
      if (lookahead != 0 &&
          lookahead != '\n') ADVANCE(12);
      END_STATE();
    case 13:
      ACCEPT_TOKEN(sym_opcode);
      if (lookahead == '-' ||
          lookahead == '_') ADVANCE(23);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(21);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(13);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(15);
      END_STATE();
    case 14:
      ACCEPT_TOKEN(sym_opcode);
      if (('0' <= lookahead && lookahead <= '9')) ADVANCE(20);
      if (('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(14);
      if (('G' <= lookahead && lookahead <= 'Z') ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(16);
      END_STATE();
    case 15:
      ACCEPT_TOKEN(sym_opcode);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          lookahead == '_') ADVANCE(23);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(15);
      END_STATE();
    case 16:
      ACCEPT_TOKEN(sym_opcode);
      if (('A' <= lookahead && lookahead <= 'Z') ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(16);
      END_STATE();
    case 17:
      ACCEPT_TOKEN(anon_sym_LBRACK);
      END_STATE();
    case 18:
      ACCEPT_TOKEN(anon_sym_RBRACK);
      END_STATE();
    case 19:
      ACCEPT_TOKEN(sym__number);
      if (lookahead == 'b') ADVANCE(20);
      if (lookahead == 'x') ADVANCE(4);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(20);
      END_STATE();
    case 20:
      ACCEPT_TOKEN(sym__number);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(20);
      END_STATE();
    case 21:
      ACCEPT_TOKEN(sym__number);
      if (('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'F') ||
          ('a' <= lookahead && lookahead <= 'f')) ADVANCE(21);
      if (lookahead == '-' ||
          ('G' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('g' <= lookahead && lookahead <= 'z')) ADVANCE(23);
      END_STATE();
    case 22:
      ACCEPT_TOKEN(anon_sym_COLON);
      END_STATE();
    case 23:
      ACCEPT_TOKEN(sym__label_name);
      if (lookahead == '-' ||
          ('0' <= lookahead && lookahead <= '9') ||
          ('A' <= lookahead && lookahead <= 'Z') ||
          lookahead == '_' ||
          ('a' <= lookahead && lookahead <= 'z')) ADVANCE(23);
      END_STATE();
    default:
      return false;
  }
}

static const TSLexMode ts_lex_modes[STATE_COUNT] = {
  [0] = {.lex_state = 0},
  [1] = {.lex_state = 6},
  [2] = {.lex_state = 6},
  [3] = {.lex_state = 6},
  [4] = {.lex_state = 6},
  [5] = {.lex_state = 6},
  [6] = {.lex_state = 6},
  [7] = {.lex_state = 6},
  [8] = {.lex_state = 6},
  [9] = {.lex_state = 6},
  [10] = {.lex_state = 6},
  [11] = {.lex_state = 6},
  [12] = {.lex_state = 0},
  [13] = {.lex_state = 1},
  [14] = {.lex_state = 1},
  [15] = {.lex_state = 0},
  [16] = {.lex_state = 3},
  [17] = {.lex_state = 2},
  [18] = {.lex_state = 11},
  [19] = {.lex_state = 0},
  [20] = {.lex_state = 0},
  [21] = {.lex_state = 6},
};

static const uint16_t ts_parse_table[LARGE_STATE_COUNT][SYMBOL_COUNT] = {
  [0] = {
    [ts_builtin_sym_end] = ACTIONS(1),
    [anon_sym_SEMI] = ACTIONS(1),
    [anon_sym_POUND] = ACTIONS(1),
    [sym_opcode] = ACTIONS(1),
    [anon_sym_LBRACK] = ACTIONS(1),
    [anon_sym_RBRACK] = ACTIONS(1),
    [sym__number] = ACTIONS(1),
    [anon_sym_COLON] = ACTIONS(1),
    [sym__label_name] = ACTIONS(1),
  },
  [1] = {
    [sym_source_file] = STATE(20),
    [sym__object] = STATE(2),
    [sym_statement] = STATE(2),
    [sym_comment] = STATE(2),
    [sym_instruction] = STATE(9),
    [sym_literal] = STATE(9),
    [sym_label] = STATE(13),
    [aux_sym_source_file_repeat1] = STATE(2),
    [ts_builtin_sym_end] = ACTIONS(3),
    [sym__whitespace] = ACTIONS(5),
    [anon_sym_SEMI] = ACTIONS(7),
    [anon_sym_POUND] = ACTIONS(7),
    [sym_opcode] = ACTIONS(9),
    [sym__number] = ACTIONS(11),
    [sym__label_name] = ACTIONS(13),
  },
  [2] = {
    [sym__object] = STATE(3),
    [sym_statement] = STATE(3),
    [sym_comment] = STATE(3),
    [sym_instruction] = STATE(9),
    [sym_literal] = STATE(9),
    [sym_label] = STATE(13),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(15),
    [sym__whitespace] = ACTIONS(17),
    [anon_sym_SEMI] = ACTIONS(7),
    [anon_sym_POUND] = ACTIONS(7),
    [sym_opcode] = ACTIONS(9),
    [sym__number] = ACTIONS(11),
    [sym__label_name] = ACTIONS(13),
  },
  [3] = {
    [sym__object] = STATE(3),
    [sym_statement] = STATE(3),
    [sym_comment] = STATE(3),
    [sym_instruction] = STATE(9),
    [sym_literal] = STATE(9),
    [sym_label] = STATE(13),
    [aux_sym_source_file_repeat1] = STATE(3),
    [ts_builtin_sym_end] = ACTIONS(19),
    [sym__whitespace] = ACTIONS(21),
    [anon_sym_SEMI] = ACTIONS(24),
    [anon_sym_POUND] = ACTIONS(24),
    [sym_opcode] = ACTIONS(27),
    [sym__number] = ACTIONS(30),
    [sym__label_name] = ACTIONS(33),
  },
};

static const uint16_t ts_small_parse_table[] = {
  [0] = 2,
    ACTIONS(36), 2,
      ts_builtin_sym_end,
      sym__whitespace,
    ACTIONS(38), 5,
      anon_sym_SEMI,
      anon_sym_POUND,
      sym_opcode,
      sym__number,
      sym__label_name,
  [12] = 2,
    ACTIONS(40), 2,
      ts_builtin_sym_end,
      sym__whitespace,
    ACTIONS(42), 5,
      anon_sym_SEMI,
      anon_sym_POUND,
      sym_opcode,
      sym__number,
      sym__label_name,
  [24] = 2,
    ACTIONS(44), 2,
      ts_builtin_sym_end,
      sym__whitespace,
    ACTIONS(46), 5,
      anon_sym_SEMI,
      anon_sym_POUND,
      sym_opcode,
      sym__number,
      sym__label_name,
  [36] = 2,
    ACTIONS(48), 2,
      ts_builtin_sym_end,
      sym__whitespace,
    ACTIONS(50), 5,
      anon_sym_SEMI,
      anon_sym_POUND,
      sym_opcode,
      sym__number,
      sym__label_name,
  [48] = 2,
    ACTIONS(52), 2,
      ts_builtin_sym_end,
      sym__whitespace,
    ACTIONS(54), 5,
      anon_sym_SEMI,
      anon_sym_POUND,
      sym_opcode,
      sym__number,
      sym__label_name,
  [60] = 2,
    ACTIONS(56), 2,
      ts_builtin_sym_end,
      sym__whitespace,
    ACTIONS(58), 5,
      anon_sym_SEMI,
      anon_sym_POUND,
      sym_opcode,
      sym__number,
      sym__label_name,
  [72] = 2,
    ACTIONS(60), 2,
      ts_builtin_sym_end,
      sym__whitespace,
    ACTIONS(62), 5,
      anon_sym_SEMI,
      anon_sym_POUND,
      sym_opcode,
      sym__number,
      sym__label_name,
  [84] = 2,
    ACTIONS(64), 2,
      ts_builtin_sym_end,
      sym__whitespace,
    ACTIONS(66), 5,
      anon_sym_SEMI,
      anon_sym_POUND,
      sym_opcode,
      sym__number,
      sym__label_name,
  [96] = 4,
    ACTIONS(68), 1,
      anon_sym_LBRACK,
    ACTIONS(70), 1,
      anon_sym_COLON,
    STATE(4), 1,
      sym_operand,
    STATE(8), 2,
      sym_address,
      sym_label_jump,
  [110] = 3,
    ACTIONS(9), 1,
      sym_opcode,
    ACTIONS(11), 1,
      sym__number,
    STATE(11), 2,
      sym_instruction,
      sym_literal,
  [121] = 1,
    ACTIONS(72), 2,
      sym_opcode,
      sym__number,
  [126] = 1,
    ACTIONS(74), 1,
      anon_sym_COLON,
  [130] = 1,
    ACTIONS(76), 1,
      sym__label_name,
  [134] = 1,
    ACTIONS(78), 1,
      sym__number,
  [138] = 1,
    ACTIONS(80), 1,
      sym__comment_text,
  [142] = 1,
    ACTIONS(82), 1,
      anon_sym_RBRACK,
  [146] = 1,
    ACTIONS(84), 1,
      ts_builtin_sym_end,
  [150] = 1,
    ACTIONS(86), 1,
      sym__whitespace,
};

static const uint32_t ts_small_parse_table_map[] = {
  [SMALL_STATE(4)] = 0,
  [SMALL_STATE(5)] = 12,
  [SMALL_STATE(6)] = 24,
  [SMALL_STATE(7)] = 36,
  [SMALL_STATE(8)] = 48,
  [SMALL_STATE(9)] = 60,
  [SMALL_STATE(10)] = 72,
  [SMALL_STATE(11)] = 84,
  [SMALL_STATE(12)] = 96,
  [SMALL_STATE(13)] = 110,
  [SMALL_STATE(14)] = 121,
  [SMALL_STATE(15)] = 126,
  [SMALL_STATE(16)] = 130,
  [SMALL_STATE(17)] = 134,
  [SMALL_STATE(18)] = 138,
  [SMALL_STATE(19)] = 142,
  [SMALL_STATE(20)] = 146,
  [SMALL_STATE(21)] = 150,
};

static const TSParseActionEntry ts_parse_actions[] = {
  [0] = {.entry = {.count = 0, .reusable = false}},
  [1] = {.entry = {.count = 1, .reusable = false}}, RECOVER(),
  [3] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 0, 0, 0),
  [5] = {.entry = {.count = 1, .reusable = true}}, SHIFT(2),
  [7] = {.entry = {.count = 1, .reusable = false}}, SHIFT(18),
  [9] = {.entry = {.count = 1, .reusable = false}}, SHIFT(21),
  [11] = {.entry = {.count = 1, .reusable = false}}, SHIFT(7),
  [13] = {.entry = {.count = 1, .reusable = false}}, SHIFT(15),
  [15] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_source_file, 1, 0, 0),
  [17] = {.entry = {.count = 1, .reusable = true}}, SHIFT(3),
  [19] = {.entry = {.count = 1, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0),
  [21] = {.entry = {.count = 2, .reusable = true}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(3),
  [24] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(18),
  [27] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(21),
  [30] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(7),
  [33] = {.entry = {.count = 2, .reusable = false}}, REDUCE(aux_sym_source_file_repeat1, 2, 0, 0), SHIFT_REPEAT(15),
  [36] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_instruction, 3, 0, 0),
  [38] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_instruction, 3, 0, 0),
  [40] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_address, 3, 0, 0),
  [42] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_address, 3, 0, 0),
  [44] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_label_jump, 2, 0, 0),
  [46] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_label_jump, 2, 0, 0),
  [48] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_literal, 1, 0, 0),
  [50] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_literal, 1, 0, 0),
  [52] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_operand, 1, 0, 0),
  [54] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_operand, 1, 0, 0),
  [56] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 1, 0, 0),
  [58] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_statement, 1, 0, 0),
  [60] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_comment, 2, 0, 0),
  [62] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_comment, 2, 0, 0),
  [64] = {.entry = {.count = 1, .reusable = true}}, REDUCE(sym_statement, 2, 0, 0),
  [66] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_statement, 2, 0, 0),
  [68] = {.entry = {.count = 1, .reusable = true}}, SHIFT(17),
  [70] = {.entry = {.count = 1, .reusable = true}}, SHIFT(16),
  [72] = {.entry = {.count = 1, .reusable = false}}, REDUCE(sym_label, 2, 0, 0),
  [74] = {.entry = {.count = 1, .reusable = true}}, SHIFT(14),
  [76] = {.entry = {.count = 1, .reusable = true}}, SHIFT(6),
  [78] = {.entry = {.count = 1, .reusable = true}}, SHIFT(19),
  [80] = {.entry = {.count = 1, .reusable = true}}, SHIFT(10),
  [82] = {.entry = {.count = 1, .reusable = true}}, SHIFT(5),
  [84] = {.entry = {.count = 1, .reusable = true}},  ACCEPT_INPUT(),
  [86] = {.entry = {.count = 1, .reusable = true}}, SHIFT(12),
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
    .primary_state_ids = ts_primary_state_ids,
  };
  return &language;
}
#ifdef __cplusplus
}
#endif
