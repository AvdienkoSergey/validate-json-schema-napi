const rust_module = require('./index.node')

console.log(rust_module.hello())
try {
  const result = rust_module.validate(
    JSON.stringify({
      type: '4',
      location_id: '111',
      category_id: '',
      c_keyword: 'qwerty',
      b_keyword: 'samsung',
    })
  )
  console.log(result)
} catch (error) {
  console.log(error.message, Array.isArray(JSON.parse(error.message)))
}
