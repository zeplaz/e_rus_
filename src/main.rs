//use std::fmt;

#[derive(Debug)]
struct TestStruct<'a> {
    id: isize,
    val: f64,
    name: String,
    mutable_pointer: *mut &'a str,
}

impl<'a> TestStruct<'a> {
    fn print(&'a self) {
        println!("id:{} | val:{} | name:{}", self.id, self.val, self.name)
    }
}


fn main() {
    basefunc(3948.4)
}

fn basefunc(x: f64) {
    let y: f64 = 4.545;
    let z: u32 = 49923;

    if y < x {
        println!(
            "appears as though you sent a biggy!{0}::so theirz {x}",
            format!("{:10x}!", z)
        );
        otherstuctaction()
    } else {
        eprintln!("error!")
    }
}

fn otherstuctaction() {
    let x: u32 = 32;
    let namer = "somestnighname";
    //x as isize;gggggggg
    let stringref: &str = "refstringo";
    let *mut mpotrstr =  stringref
    let ts = TestStruct {
        val: 752.6,
        id: x as isize,
        name: namer.to_string(),
        mutable_pointer: mpotrstr
    };

    ts.print();
}

#[cfg(test)]
mod sample_test_modual {

    #[test]
    fn test() {
        let mut y: f64 = 4.545;
        let x: u32 = 32;

        if y < x.into() {
            println!(
                "appears as though you sent a biggy!{0}::so theirz {y}",
                format!("{:10x}!", x)
            );
            assert_ne!(y, x.into(), "some stuff about a test")
        } else {
            assert_eq!(y, x.into(), "sshoudl daild?t")
        }
    }
}
