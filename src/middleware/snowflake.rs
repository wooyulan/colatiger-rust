use idgenerator::*;




pub fn init_snowflak(){

    let options = IdGeneratorOptions::new().worker_id(1).worker_id_bit_len(10);

    let _ = IdInstance::init(options).unwrap();


    let options = IdGeneratorOptions::new().seq_bit_len(12);
    let _ = IdInstance::set_options(options).unwrap();
}



// 获取雪狐id
pub fn next_id() -> i64 {

    let mut new_id: i64 = 0;
    let mut times = 5000;
    while times > 0 {
        // Call `next_id` to generate a new unique id.
        new_id = IdInstance::next_id();
        times -= 1;
    }
    new_id
}