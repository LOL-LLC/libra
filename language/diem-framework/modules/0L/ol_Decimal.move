address 0x1 {
module Decimal {
    // The Move Decimal data structure is optimized for readability and compatibility.
    // In particular it is indended for compatibility with the underlying rust_decimal crate https://github.com/paupino/rust-decimal. In that library a new decimal type is initialized with Decimal::new(mantissa: i64, scale: u32)
    // Note: While the underlying Rust crate type has optimal storage characteristics, this Move decimal representation is NOT optimized for storage.

    struct Decimal has key, store, drop {
        sign: bool,
        int: u64,
        scale: u8, // max intger is number 28
    }

    // TODO: What is the largest integer rust_decimal can take?

    const MAX_RUST_U64: u64 = 18446744073709551615;
    // pair decimal ops
    const ADD: u8 = 1;
    const SUB: u8 = 2;
    const MULT: u8 = 3;
    const DIV: u8 = 4;

    // single ops
    const SQRT: u8 = 5;

    const ROUNDING_UP: u8 = 1;

    native public fun decimal_demo(sign: bool, int: u64, scale: u8): (bool, u64, u8);

    native public fun single_op(op_id: u8, sign: bool, int: u64, scale: u8): (bool, u64, u8);

    native public fun pair_op(
      op_id: u8,
      rounding_strategy_id: u8,
      // left number
      sign_1: bool,
      int_1: u64,
      scale_1: u8,
      // right number
      sign_2: bool,
      int_2: u64,
      scale_3: u8
    ): (bool, u64, u8);

    public fun new(sign: bool, int: u64, scale: u8): Decimal {
      // in Rust, the integer is downcast to u64
      // so we limit new Decimal types to that scale.
      assert(int < MAX_RUST_U64, 01);

      // check scale < 28
      assert(scale < 28, 02);

      return Decimal {
        sign: sign,
        int: int,
        scale: scale
      }
    }

    /////// SUGAR /////////
    
    public fun sqrt(d: &Decimal): Decimal {
      let (sign, int, scale) = single_op(5, *&d.sign, *&d.int, *&d.scale);
      return Decimal {
        sign: sign,
        int: int,
        scale: scale,
      }
    }

    public fun add(l: &Decimal, r: &Decimal): Decimal {
      let (sign, int, scale) = pair_op(1, 0, *&l.sign, *&l.int, *&l.scale,  *&r.sign, *&r.int, *&r.scale);
      return Decimal {
        sign: sign,
        int: int,
        scale: scale,
      }
    }

    public fun sub(l: &Decimal, r: &Decimal): Decimal {
      let (sign, int, scale) = pair_op(2, 0, *&l.sign, *&l.int, *&l.scale,  *&r.sign, *&r.int, *&r.scale);
      return Decimal {
        sign: sign,
        int: int,
        scale: scale,
      }
    }
    public fun mul(l: &Decimal, r: &Decimal): Decimal {
      let (sign, int, scale) = pair_op(3, 0, *&l.sign, *&l.int, *&l.scale,  *&r.sign, *&r.int, *&r.scale);
      return Decimal {
        sign: sign,
        int: int,
        scale: scale,
      }
    }

     public fun div(l: &Decimal, r: &Decimal): Decimal {
      let (sign, int, scale) = pair_op(4, 0, *&l.sign, *&l.int, *&l.scale,  *&r.sign, *&r.int, *&r.scale);
      return Decimal {
        sign: sign,
        int: int,
        scale: scale,
      }
    }


    public fun rescale(l: &Decimal, r: &Decimal): Decimal {
      let (sign, int, scale) = pair_op(0, 0, *&l.sign, *&l.int, *&l.scale,  *&r.sign, *&r.int, *&r.scale);
      return Decimal {
        sign: sign,
        int: int,
        scale: scale,
      }
    }

    public fun power(l: &Decimal, r: &Decimal): Decimal {
      let (sign, int, scale) = pair_op(5, 0, *&l.sign, *&l.int, *&l.scale,  *&r.sign, *&r.int, *&r.scale);
      return Decimal {
        sign: sign,
        int: int,
        scale: scale,
      }
    }

    ///// GETTERS /////

    // unwrap creates a new decimal instance
    public fun unwrap(d: &Decimal): (bool, u64, u8) {
      return (*&d.sign, *&d.int, *&d.scale)
    }

    // borrow sign
    public fun borrow_sign(d: &Decimal): &bool {
      return &d.sign
    }

    // borrows the value of the integer
    public fun borrow_int(d: &Decimal): &u64 {
      return &d.int
    }

    // borrow sign
    public fun borrow_scale(d: &Decimal): &u8 {
      return &d.scale
    }
}
}
