//use std::fmt;

use printf;

use std::env;
use std::fs::File;
use std::io::Write;

use std::path::PathBuf;

struct FileManger {
    a_dir: PathBuf,
}

impl FileManger {
    fn set_dir() {
        let mut path = PathBuf::new();
    }
    fn dump_data<'a, T>(&mut self, in_data: T, filename: &str) {
        let mut path = PathBuf::from("/tmp");
        path.push(filename);
        self.a_dir = env::temp_dir();
        let f_path = self.a_dir.join(filename);
        let mut file = File::create(f_path).unwrap();

        let mut buffer = [0u8; 4096];

        //file.write(&buffer[..in_data]).unwrap()

        //writeln!(&mut file, );
    }
}

#[allow(dead_code)]
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
    let mut strref_01: &str = "refstringo";
    let _str_01_ptr: *const u8 = strref_01.as_ptr();
    let mpotrstr: *mut &str = &mut strref_01;

    let mut p = std::ptr::addr_of!(strref_01);

    let ts = TestStruct {
        val: 752.6,
        id: x as isize,
        name: namer.to_string(),
        mutable_pointer: mpotrstr,
    };
    println!("{:?}", mpotrstr);
    println!("{:?}", _str_01_ptr);
    println!("{:?}", p);

    ts.print();
}
//libusb_set_log_cb

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
