/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

module.exports = grammar({
  name: 'peppermint',

  rules: {
    source_file: $ => repeat($._object),

    _object: $ => choice(
      $.statement,
      $._whitespace,
      $.comment,
    ),

    _whitespace: $ => /[ \t\n]+/,

    statement: $ => choice(
      $.instruction,
      $.literal,
    ),

    comment: $ => seq(choice(';', '#'), $._comment_text),
    _comment_text: $ => /[^\n]*/,

    instruction: $ => seq(
      $.opcode,
      $._whitespace,
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

    label: $ => seq($._label_name, ':'),
    label_jump: $ => seq(':', $._label_name),
    _label_name: $ => /[a-zA-Z][a-zA-Z0-9\-_]+/,
  }
});
