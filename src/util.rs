pub fn pos_is_adjacent((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> bool {
    (ax - bx).abs() == 1 || (ay - by).abs() == 1
}
