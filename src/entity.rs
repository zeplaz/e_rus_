

use std::sync::Mutex;
use std::thread;
use std::collections::HashMap;



use std::fs::File;
use std::io::Write;
use std::io::Read;
use std::path::PathBuf;

const CONFIG_DIR: str = "data/config";


#[derive(Default)]
struct Levl_Config<T>
{
        pub id : ID,
        pub name: &str,
        pub data: T,

}





trait Entity{
    fn new(name: &'static str) -> self;


    fn name(&self) ->&'static str;
    fn id(&self)-> u16;

}


impl

impl Levl_Config
{

}
fn load_config_data(c_type: &str)
{

    let filename = CONFIG_DIR + name +  ID.to_string()
    let mut path = PathBuf::from(CONFIG_DIR);

    let mut path.push(filename)

}

enum SyS_Status{
    STALL,
    RUN,
    IGNITION,
    PANIC,
    SHUTDOWN,
    PAUSE,
}

enum Entity_Status<T> {
    Paniced(T),
    Alertness(T),
    Alive(bool),
    HP(u16),
    Foucsed(T),
    Behavour(T),
}

struct Postional{
    deginati:
}

trait configurable<T,U>
{
    fn get_config_data()

    {
        read

    }
}
trait ID_have
{
        fn get_next_id() -> ID {
}

struct ID {

    pub static ref NEXT_ID: Mutex<f64>,
}

impl ID {
    fn init()
    {
        NEXT_ID = Mutex::new(0);
    }


struct Entity{
    id: ID,
}


fn calculate_location()-> Pos3d
{}

let _th01 = thread::spawn(move || -> ())
{


drop(data)
.lock().unwrap()

}


struct Entity_Ctl_Systems
{

}

impl Entity_Ctl_Systems{
    fn hash_enity_by_id( & mut in_ent)
    {
    }



}




struct vec3
{
    posx: f64,
    posy: f64,
    posy: f64,
}
struct entity_status{

}


fn get_physics_result()->(f64, f64, f64)
{
    let a_set_postion_in_tuple = 3
    match tuple.a_set_postion_in_tuple



}
