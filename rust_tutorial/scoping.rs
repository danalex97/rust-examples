fn destroy_box(_c : Box<i32>) {
    // c is destroyed since the function takes ownership of the memory
}

fn borrow_box(c : &i32) {
    println!("borrowed {}", c);
}

fn borrow_mut_box(c : &mut i32) {
    *c = 32;
}

struct Point { x: i32, y: i32, z: i32 }

#[derive(Clone, Copy)]
#[allow(dead_code)]
struct Point2 { x: i32, y: i32 }

struct ComplexRef<'a> {
    int_ref: &'a i32,
}

fn main() {
    let x = 5u32;
    let y = x;
    println!("x is {}, and y is {}", x, y);

    let a = Box::new(5i32);
    println!("a contains {}", a);

    // Move a into b
    let b = a;

    // Error -> move occurs because `a` has type `std::boxed::Box<i32>`, which does not implement the `Copy` trait
    // println!("a contains {}", a);

    destroy_box(b);

    // Error -> in this case the move occurs when calling destroy_box
    // println!("b contains {}", b);

    // mutability can be changed at ownership transfer
    let c = Box::new(5i32);
    let mut mc = c;
    *mc = 4;
    println!("mc contains {}", mc);

    // borrowing the contents of the box
    borrow_box(&mc);

    {
        // borrow box
        let _mc_ref: &i32 = &mc;
        // note we can borrow to multiple variables
        let _mc_ref2: &i32 = &mc;

        // can't move while borrowed
        // destroy_box(mc);
    }

    // borrwing mutable
    borrow_mut_box(&mut mc);
    println!("New value {}", mc);

    // can't borrow mutably an immutable object
    let _uc = Box::new(5i32);
    // borrow_mut_box(&mut uc);

    // when data is immutably borrowed, it also freezes - i.e. can't be modified
    {
        let _mc_ref: &i32 = &mc;

        // assignment to borrowed mc can't be done
        // *mc = 40;
    }

    // aliasing
    let mut point = Point { x: 0, y: 0, z: 0 };

    {
        let mut_borrow = &mut point;

        mut_borrow.x = 5;
        mut_borrow.y = 2;
        mut_borrow.z = 1;

        // can't borrow `point.y` as immutable as `point` was borrowed as mutable
        // let y = &point.y;

        // can't borrow `point.z` as immutable as `point` was borrowed as mutable
        // println!("Point Z coordinate is {}", point.z);

        // here we don't borrow, but just pass a mutable reference as an immutable one
        println!("Point Z coordinate is {}", mut_borrow.z);
    }

    {
        let _b1 = &point;
        let _b2 = &point;

        // can't borrow mutable and also immutable
        // let _mb = &mut point;
    }


    {
        let c = 'Q';

        // A `ref` borrow on the left side of an assignment is equivalent to
        // an `&` borrow on the right side.
        let ref ref_c1 = c;
        let ref_c2 = &c;

        println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);
    }

    let point = Point2 { x: 0, y: 0 };

    // we can borrow the field x
    let _copy_of_x = {
        // `ref_to_x` is a reference to the `x` field of `point`.
        let Point2 { x: ref ref_to_x, y: _ } = point;

        // Return a copy of the `x` field of `point`.
        *ref_to_x
    };

    // lifetimes
    let a: i32 = -423;
    let b: i32 = 2;

    let cr: ComplexRef = ComplexRef {
        int_ref: &a,
    };

    let e;
    {
        let r = cr;
        e = some_func(&a, &b, &r);
    }
    println!("{}", *e);
}

// If we used only lifetime 'a, the time would be constrained to the inner scope(in which r is
// declared)

// Note cr lives 'b, but has a reference to an object that keeps references that live longer
fn some_func<'a, 'b>(a: &'a i32, _b: &i32, cr: &'b ComplexRef<'a>) -> &'a i32 {
    if *a < 5 {
        cr.int_ref
    } else {
        a
    }
}

// generic lifetime example
#[allow(dead_code)]
struct Ref<'a, T: 'a>(&'a T);

// `<'a: 'b, 'b>` reads as lifetime `'a` is at least as long as `'b`.
#[allow(dead_code)]
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}
