/* 
    Miscellaneous math.
*/

extern crate cgmath;


/* Clamp value between lbound and ubound. */
pub fn clamp<T: PartialOrd>(value: T, lbound: T, ubound: T) -> T {
    if value < lbound {
        lbound
    } else if value > ubound {
        ubound
    } else {
        value
    }
}

