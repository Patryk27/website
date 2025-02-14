fw: {
  dateLessThat = a: b: if a.y == b.y then if a.m == b.m then a.d < b.d else a.m < b.m else a.y < b.y;
}
