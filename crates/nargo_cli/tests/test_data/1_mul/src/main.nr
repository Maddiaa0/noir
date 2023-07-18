// Test unsafe integer multiplication with overflow: 12^8 = 429 981 696
// The circuit should handle properly the growth of the bit size

#[oracle(rand)]
fn rand(_a: u32) -> Field {}


fn main(mut x: u32, y: u32, z: u32) {
    x *= y;
    x *= x;  //144
    x *= x;  //20736
    x *= x;  //429 981 696
    assert(x == z);
}

unconstrained fn unsafe_safe_inverse(x: Field) ->Field {

    if x == 0//rand(4) 
    {
        0
    } else {
        1/x
    }
}

unconstrained fn unsafe(mut x: u32) ->u32 {
 
    for _i in 0..3 {
        x=x+2;
        if x ==1 {
            x=x+2;
        }
    }
    x
}