/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: 'peppermint',

  rules: {
    source_file: $ => repeat(choice(
      $.statement,
      $.label,
    )),

    _whitespace: $ => /[ \t\n]+/,

    statement: $ => seq(
      choice(
        $.instruction,
        $.literal,
      ),
    ),

    comment: $ => /[;#][^\n]*/,

    instruction: $ => seq(
      $.opcode,
      $.operand,
    ),

    opcode: $ => /[a-zA-Z]+/,
    operand: $ => choice(
      $.label_jump,
      $.address,
    ),

    literal: $ => $._number,
    address: $ => seq('[', $._number, ']'),
    _number: $ => /(0[xb])?[0-9a-fA-F]+/,

    // TODO: this is pretty horrible repetition
    label: $ => /[a-zA-Z][a-zA-Z0-9\-_]*:/,
    label_jump: $ => /:[a-zA-Z][a-zA-Z0-9\-_]*/,
  },

  word: $ => $.opcode,

  extras: $ => [
    $.comment,
    $._whitespace,
  ],
});
