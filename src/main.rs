use std::io::{stdout, Write, Read};
use curl::easy::{Easy, List};
use structopt::StructOpt;
use std::fmt;
use std::str;

#[derive(StructOpt, Debug)]
#[structopt(name = "rust-awscurl", about = "Simple CURL with AWS SigV4", author = "Vorapoap Lohwongwtana")]
struct Opt {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    verbose: u8,

    /// AWS Access Key
    #[structopt(short, long, help = "AWS Access Key", env = "AWS_ACCESS_KEY_ID")]
    access_key: Option<String>,

    /// AWS Secret
    #[structopt(short, long, help = "AWS Secret Key", env = "AWS_SECRET_ACCESS_KEY")]
    secret_key: Option<String>,

    
    /// Method
    #[structopt(short = "X", long, help = "Request Method", default_value ="GET")]
    method: String,

    /// Header
    #[structopt(short = "H", long, help = "HTTP Headers")]
    header: Vec<String>,

    /// AWS SigV4
    #[structopt(long, help = "AWS SigV4 Option, format is region:service")]
    aws_sigv4: Option<String>,

    /// Region
    #[structopt(long, help = "AWS Service (alternate way to build --aws-sigv4)", requires_all(&["service"]))]
    region: Option<String>,

    /// AWS Setrvice
    #[structopt(long, help = "AWS Service (alternate way to build --aws-sigv4)", requires_all(&["region"]))]
    service: Option<String>,


    /// Enable Curl Wrapper (libcurl)
    #[structopt(long, help = "Enable libcurl")]
    disable_libcurl: bool,

    /// Post Data
    #[structopt(short = "d", long, help = "POST data body")]
    post_data: Option<String>,

    /// Url
    #[structopt(help = "Target Url")]
    url: String,
}

impl fmt::Display for Opt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nCommand Line Arguments\n\
            verbose:         {}\n\
            aws_sigv4:       {:?}\n\
            access_key:      {}\n\
            secret_key:      {}\n\
            service:         {:?}\n\
            region:          {:?}\n\
            method:          {}\n\
            header:          {:?}\n\
            disable_libcurl: {}\n\
            post_data:       {:?}\n\
            url:             {}", 
            self.verbose, 
            self.aws_sigv4,
            self.access_key.as_ref().unwrap(), 
            self.secret_key.as_ref().unwrap(), 
            self.service, 
            self.region, 
            self.method, 
            self.header, 
            self.disable_libcurl,
            self.post_data,
            self.url)
    }
}

fn main() {
    let opt = Opt::from_args();
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

    let aws_sigv4: String = match opt.aws_sigv4 {
        Some(s) => s,
        None => {
            if opt.access_key.is_some() && opt.secret_key.is_some() {
                format!("aws:amz:{}:{}", opt.region.unwrap(), opt.service.unwrap())
            } else {
                String::new()
            }                    
        }
    };

    if aws_sigv4.len() > 0 {
        easy.aws_sigv4(aws_sigv4.as_str()).unwrap();
    }

    if opt.access_key.is_some() {
        easy.username(opt.access_key.unwrap().as_str()).unwrap();
    }
    if opt.secret_key.is_some() {
        easy.password(opt.secret_key.unwrap().as_str()).unwrap();
    }

    easy.url(&opt.url).unwrap();
    if opt.header.len() > 0 {
        easy.http_headers(header_list).unwrap();

    }

    let post_data = match opt.post_data {
        Some(s) => s.clone(),
        None => { String::new() }
    };
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

    transfer.perform().unwrap();
}
