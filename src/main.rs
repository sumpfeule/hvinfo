use serde_json;
use std::{collections::HashMap, io::Read, process::exit};

const HVPATH: &str = "/var/lib/hyperv/.kvp_pool_3";

const HVLEN: usize = 40960;

// derived from /usr/src/linux-headers-`uname -r`/include/uapi/linux/hyperv.h
const HV_KVP_EXCHANGE_MAX_KEY_SIZE: usize = 512;
const HV_KVP_EXCHANGE_MAX_VALUE_SIZE: usize = 2048;

fn main() {
    let f = std::fs::File::open(HVPATH).unwrap();
    let mut reader = std::io::BufReader::new(f);
    let mut read_bytes = 0;

    let mut buf_key: [u8; HV_KVP_EXCHANGE_MAX_KEY_SIZE] = [0u8; HV_KVP_EXCHANGE_MAX_KEY_SIZE];
    let mut buf_value: [u8; HV_KVP_EXCHANGE_MAX_VALUE_SIZE] = [0u8; HV_KVP_EXCHANGE_MAX_VALUE_SIZE];

    let mut kv_pairs: HashMap<String, String> = Default::default();

    while read_bytes < HVLEN {
        match reader.read_exact(&mut buf_key) {
            Ok(()) => {
                read_bytes += HV_KVP_EXCHANGE_MAX_KEY_SIZE;
                let key_size: usize = match buf_key.iter().position(|n| n == &0) {
                    Some(n) => n,
                    None => HV_KVP_EXCHANGE_MAX_KEY_SIZE,
                };

                match reader.read_exact(&mut buf_value) {
                    Ok(()) => {
                        read_bytes += HV_KVP_EXCHANGE_MAX_VALUE_SIZE;
                        let value_size: usize = match buf_value.iter().position(|n| n == &0) {
                            Some(n) => n,
                            None => HV_KVP_EXCHANGE_MAX_VALUE_SIZE,
                        };

                        let key = match String::from_utf8(buf_key[..key_size].to_vec()) {
                            Ok(s) => s,
                            Err(e) => {
                                eprintln!("{}", e);
                                exit(1);
                            }
                        };

                        let value = match String::from_utf8(buf_value[..value_size].to_vec()) {
                            Ok(s) => s,
                            Err(e) => {
                                eprintln!("{}", e);
                                exit(1);
                            }
                        };

                        kv_pairs.insert(key, value);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }

    assert_eq!(HVLEN, read_bytes);

    println!("{}", serde_json::to_string(&kv_pairs).unwrap());
}
