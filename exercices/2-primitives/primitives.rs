fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 8i32;

    let default_float = 3.0; // default=f64
    let default_integer = 4; // default=i32

    let mut inferred_type = 12;
    inferred_type = 3764765778;

    let mut mutable = 23;
    mutable = 22;

    // mutable=true; // We can't change the type of a mut variable
    let mutable = true;
    /* Compound types*/
    let my_array: [i32;5] = [1,2,3,4,5];
    let my_tuple = (5u32,-78.323f32,true,i8,'a');
}
