module.exports = grammar({
  name: 'gifdsl',
  extras: $ => [/\s/, $.comment],
  conflicts: $ => [[$.travel_block]],

  rules: {
    source_file: $ => repeat($._stmt),
    comment: $ => token(seq('#', /.*/)),

    _stmt: $ => choice(
      $.travel_block,
      $.expr_stmt,
      $.if_stmt      // Add if statements
    ),

    // Add if statement rule
    if_stmt: $ => prec.right(seq(
      'if', $.expr, ':',
      repeat($._stmt),
      optional(seq('else', ':', repeat($._stmt)))
    )),

    travel_block: $ => seq(
      'with', 'travel', $.number, ':',
      repeat($._stmt)
    ),

    expr_stmt: $ => seq(
      $.expr, repeat(seq('*', $.number))
    ),

    expr: $ => choice(
      $.call_expr,
      $.identifier,
      $.number,
      $.binary_expr  // Add binary expressions
    ),

    // Add binary expressions for conditions
    binary_expr: $ => choice(
      prec.left(1, seq($.expr, '%', $.expr)),
      prec.left(2, seq($.expr, '==', $.expr)),
      prec.left(2, seq($.expr, '!=', $.expr)),
      prec.left(2, seq($.expr, '>', $.expr)),
      prec.left(2, seq($.expr, '<', $.expr))
    ),

    call_expr: $ => seq($.identifier, '(', ')'),
    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_]*/,
    number: $ => /[0-9]+/
  }
});
