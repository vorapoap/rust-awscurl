use curl::easy::{Easy, List};
use std::{io::{stdout, Write, Read}};

use crate::opt::Opt;

pub fn curl_request(opt: &Opt) -> () {

    let mut header_list = List::new();
    let mut easy = Easy::new();

    for header in opt.header.iter() {
        header_list.append(header.as_str()).unwrap();
    }

    // If verbose > 0 then print argument key/values
    if opt.verbose > 0 {
        eprintln!("{}", opt);
        easy.verbose(true).unwrap();
    }

    if opt.url.len() == 0 {
        eprintln!("No URL specified\n");
    }

    let aws_sigv4: String = match &opt.aws_sigv4 {
        Some(s) => s.to_owned(),
        None => {
            if opt.access_key.is_some() && opt.secret_key.is_some() {
                format!("aws:amz:{}:{}", opt.region.to_owned().unwrap(), opt.service.to_owned().unwrap())
            } else {
                "".to_owned()
            }                    
        }
    };
    let post_data: String = opt.get_postdata();
    

    if aws_sigv4.len() > 0 {
        easy.aws_sigv4(aws_sigv4.as_str()).unwrap();
    }
    if opt.access_key.is_some() {
        easy.username(opt.access_key.to_owned().unwrap().as_str()).unwrap();
    }
    if opt.secret_key.is_some() {
        easy.password(opt.secret_key.to_owned().unwrap().as_str()).unwrap();
    }

    easy.url(&opt.url).unwrap();
    if opt.header.len() > 0 {
        easy.http_headers(header_list).unwrap();
    }

    let http_method: &str;
    match opt.method.as_str() {
        "GET" => {
            http_method = "GET";
        },

        "PUT" => {
            easy.put(true).unwrap();
            easy.post_field_size(post_data.as_bytes().len() as u64).unwrap();
            http_method = "PUT";
        },

        "POST" => {    
            easy.post(true).unwrap();
            easy.post_field_size(post_data.as_bytes().len() as u64).unwrap();
            http_method = "POST";
        },

        _ => {
            easy.custom_request(opt.method.as_str()).unwrap();
            easy.post_field_size(post_data.as_bytes().len() as u64).unwrap();
            http_method = opt.method.as_str();
        }
    }
    let mut transfer = easy.transfer();

    // Prepare POST/PUT/DELETE body data callback if post data exists
    if http_method != "GET" && post_data.len() > 0 {
        transfer.read_function(|buf| { 
            Ok(post_data.as_bytes().read(buf).unwrap_or(0))
        }).unwrap();            
    }

    // Prepare output callback.
    transfer.write_function(|buf| {
        stdout().write_all(buf).unwrap();
        Ok(buf.len())
    }).unwrap();

    transfer.perform().unwrap()
}