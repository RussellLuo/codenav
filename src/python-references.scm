(call
  function: [
      (identifier) @name.reference.call
      (attribute
        object: (identifier) @object.reference.call
        attribute: (identifier) @name.reference.call)
  ]
  arguments: (
    (argument_list (
      (identifier)* @arg.reference.call
      (keyword_argument
        value: (identifier) @kwarg.reference.call)*
    ))
  )
)

(assignment
  right: (identifier) @name.reference.assignment
)

(binary_operator
  (identifier) @name.reference.binary
)
