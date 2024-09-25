; Note that Tree-sitter does not support nested queries of arbitrary complexity (see https://github.com/tree-sitter/tree-sitter/issues/880),
; so the following queries only support capturing identifiers with a fixed number of nested level.

; Supports capturing identifiers as function names in three styles: x(), x.y() or x.y.z()
(call_expression
  function: [
    (identifier) @name.reference.call
    (member_expression
      object: [
        (identifier) @name.reference.object
        (member_expression
          object: (identifier) @name.reference.object
          property: (property_identifier) @name.reference.object
        )
      ]
      property: (property_identifier) @name.reference.call
    )
  ]
)

; Supports capturing identifiers as function arguments in three styles: f(x), f(x.y), f(x.y.z)
(call_expression
  arguments: (
    arguments [
      (identifier) @name.reference.object
      (member_expression
        object: [
          (identifier) @name.reference.object
          (member_expression
            object: (identifier) @name.reference.object
            property: (property_identifier) @name.reference.object
          )
        ]
        property: (property_identifier) @name.reference.object
      )
    ]
  )
)

; Supports capturing identifiers as assignment values in three styles: a = x, a = x.y , a = x.y.z
(variable_declarator
  value: [
    (identifier) @name.reference.object
    (member_expression
      object: [
        (identifier) @name.reference.object
        (member_expression
          object: (identifier) @name.reference.object
          property: (property_identifier) @name.reference.object
        )
      ]
      property: (property_identifier) @name.reference.object
    )
  ]
)

; Supports capturing identifiers as binary/comparison operands in three styles: a + x, a + x.y , a + x.y.z, a > x, a > x.y, a > x.y.z
(binary_expression
  [
    (identifier) @name.reference.object
    (member_expression
      object: [
        (identifier) @name.reference.object
        (member_expression
          object: (identifier) @name.reference.object
          property: (property_identifier) @name.reference.object
        )
      ]
      property: (property_identifier) @name.reference.object
    )
  ]
)
