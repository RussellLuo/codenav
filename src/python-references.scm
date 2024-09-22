; Note that Tree-sitter does not support nested queries of arbitrary complexity (see https://github.com/tree-sitter/tree-sitter/issues/880),
; so the following queries only support capturing identifiers with a fixed number of nested level.

; Supports capturing identifiers as function names in three styles: x(), x.y() or x.y.z()
(call
  function: [
    (identifier) @name.reference.call
    (attribute
      object: [
        (identifier) @name.reference.object
        (attribute
          object: (identifier) @name.reference.object
          attribute: (identifier) @name.reference.object
        )
      ]
      attribute: (identifier) @name.reference.call
    )
  ]
)

; Supports capturing identifiers as function arguments in three styles: f(x), f(x.y), f(x.y.z)
(call
  arguments: (
    (argument_list [
      [
        (identifier) @name.reference.object
        (attribute
          object: [
            (identifier) @name.reference.object
            (attribute
              object: (identifier) @name.reference.object
              attribute: (identifier) @name.reference.object
            )
          ]
          attribute: (identifier) @name.reference.object
        )
      ]
    ])
  )
)

; Supports capturing identifiers as function keyword arguments in three styles: f(a=x), f(a=x.y), f(a=x.y.z)
(call
  arguments: (
    (argument_list (
      (keyword_argument
        value: [
          (identifier) @name.reference.object
          (attribute
            object: [
              (identifier) @name.reference.object
              (attribute
                object: (identifier) @name.reference.object
                attribute: (identifier) @name.reference.object
              )
            ]
            attribute: (identifier) @name.reference.object
          )
        ]
      )
    ))
  )
)

; Supports capturing identifiers as assignment values in three styles: a = x, a = x.y , a = x.y.z
(assignment
  right: [
    (identifier) @name.reference.object
    (attribute
      object: [
        (identifier) @name.reference.object
        (attribute
          object: (identifier) @name.reference.object
          attribute: (identifier) @name.reference.object
        )
      ]
      attribute: (identifier) @name.reference.object
    )
  ]
)

; Supports capturing identifiers as binary operands in three styles: a + x, a + x.y , a + x.y.z
(binary_operator
  [
    (identifier) @name.reference.object
    (attribute
      object: [
        (identifier) @name.reference.object
        (attribute
          object: (identifier) @name.reference.object
          attribute: (identifier) @name.reference.object
        )
      ]
      attribute: (identifier) @name.reference.object
    )
  ]
)

; Supports capturing identifiers as comparison operands in three styles: a > x, a > x.y , a > x.y.z
(comparison_operator
  [
    (identifier) @name.reference.object
    (attribute
      object: [
        (identifier) @name.reference.object
        (attribute
          object: (identifier) @name.reference.object
          attribute: (identifier) @name.reference.object
        )
      ]
      attribute: (identifier) @name.reference.object
    )
  ]
)