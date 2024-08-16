data MyRecord = {
  a: int
  b: str
  c: int
  d: str
}

my_record: MyRecord = MyRecord {
  a: 1
  b: "hello"
  c: 2
  d: "world"
}

a, b, c, d: int, str, int, str = my_record
