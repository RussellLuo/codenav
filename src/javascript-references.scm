(call_expression
  function: [
    (identifier) @name.reference.call
    (member_expression
      object: (identifier) @object.reference.call
      property: (property_identifier) @name.reference.call
    )
  ]
  arguments: (
    (identifier)* @arg.reference.call
  )
)

(variable_declarator
  name: (identifier) @name.reference.assignment
)

(binary_expression
  (identifier) @name.reference.binary
)