
#[derive(Debug, Clone, Copy)]
struct Somedata {
    a: i32,
}

impl Somedata {
    
    fn set_a(&mut self, newdata: i32) {
       self.a = newdata;
    }
}


fn main() {


let mut a = Somedata{a:1};

// take_ownership(a);
// take_ownership_mutable_infunc(a);

// take_ref(&a);
take_mut_ref_mutable_infunc(&mut a);
println!("a = {:?}", &a);



}


fn take_ownership(a: Somedata) {

    // can not change
    // a = Somedata{a:3};

    // can not change
    // a.set_a(2);

    println!("a = {:?}", a);
}

fn take_ownership_mutable_infunc(mut a: Somedata) {

    // cannot affect outside
    // a bind to another data
    a = Somedata{a:4};
    println!("a = {:?}", &a);

    // can change
    a.set_a(5);
    println!("a = {:?}", &a);
}



fn take_ref(a: & Somedata) {

    // can not change
    // a = Somedata{a:6};

    // can not change
    // a.set_a(7);

    println!("a = {:?}", a);
}


fn take_mut_ref(a: &mut Somedata) {

    // can change
    *a = Somedata{a:3};

    // can change
    a.set_a(8);

    println!("a = {:?}", a);
}


fn take_mut_ref_mutable_infunc(mut a: &mut Somedata) {

    // can not change
    // let b =  &mut Somedata{a:9};


    // can not change
    // a.set_a(9);

    println!("a = {:?}", a);
}