/// Determines whether the input slice contains the elements `a` and `b` adjacent to each other (in either order).
pub fn has_adjacent<T>(data: &[T], a: &T, b: &T) -> bool
where
    T: PartialEq,
{
    for pair in data.windows(2) {
        if (&pair[0] == a && &pair[1] == b) || (&pair[0] == b && &pair[1] == a) {
            return true;
        }
    }
    false
}
