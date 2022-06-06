use std::{collections::HashMap, fs, io, process};

const HVPATH: &str = "/var/lib/hyperv/.kvp_pool_3";

const HVLEN: usize = 40960;

// derived from /usr/src/linux-headers-`uname -r`/include/uapi/linux/hyperv.h
const HV_KVP_EXCHANGE_MAX_KEY_SIZE: usize = 512;
const HV_KVP_EXCHANGE_MAX_VALUE_SIZE: usize = 2048;

const MAX_INDEX: usize = HVLEN / (HV_KVP_EXCHANGE_MAX_KEY_SIZE + HV_KVP_EXCHANGE_MAX_VALUE_SIZE);

fn read_hvfile() -> Result<String, io::Error> {
    fs::read_to_string(HVPATH)
}

fn kv_at_index(hvdata: &String, index: usize) -> (String, String) {
    assert!(index <= MAX_INDEX, "index out of bounds");

    let k_start: usize = index * (HV_KVP_EXCHANGE_MAX_KEY_SIZE + HV_KVP_EXCHANGE_MAX_VALUE_SIZE);
    let v_start: usize = k_start + HV_KVP_EXCHANGE_MAX_KEY_SIZE;

    let k_end: usize = k_start + HV_KVP_EXCHANGE_MAX_KEY_SIZE;
    let v_end: usize = v_start + HV_KVP_EXCHANGE_MAX_VALUE_SIZE;
    (
        hvdata[k_start..k_end].to_string().replace("\0", r#""#),
        hvdata[v_start..v_end].to_string().replace("\0", r#""#),
    )
}

fn main() {
    let hvdata = match read_hvfile() {
        Ok(s) => s,
        Err(e) => {
            println!("Unable to open {}: {}", HVPATH, e.to_string());
            process::exit(1);
        }
    };
    if hvdata.len() != HVLEN {
        println!(
            "{} seems to have the wrong size ({}, expected {}).\nIncompatible hyper-V version?",
            HVPATH,
            hvdata.len(),
            HVLEN
        );
        process::exit(1);
    }

    let mut hvinfo: HashMap<String, String> = HashMap::new();
    for i in 0..MAX_INDEX {
        let (k, v) = kv_at_index(&hvdata, i);
        hvinfo.insert(k, v);
    }
    println!("{:?}", hvinfo);
}
