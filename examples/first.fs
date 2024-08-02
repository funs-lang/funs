increment = (x) -> x + 1

test_multiline = () ->
    assert.equal increment(1), 2
    assert.equal increment(2), 3

test_multiline()
