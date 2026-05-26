const { describe, it } = require('node:test')
const assert = require('node:assert/strict')

const { fibonacci } = require('../index.js')

describe('fibonacci', () => {
  it('computes small values', () => {
    assert.equal(fibonacci(0), 0)
    assert.equal(fibonacci(1), 1)
    assert.equal(fibonacci(10), 55)
    assert.equal(fibonacci(20), 6765)
  })

  it('rejects overflow', () => {
    assert.throws(() => fibonacci(93), /overflows i64/)
  })
})
