use structopt::StructOpt;
use std::fmt;
use urlencoding::{encode,decode};



#[derive(StructOpt, Debug)]
#[structopt(name = "rust-awscurl", about = "Simple CURL with AWS SigV4", author = "Vorapoap Lohwongwtana")]
pub struct Opt {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// AWS Access Key
    #[structopt(short, long, help = "AWS Access Key", env = "AWS_ACCESS_KEY_ID")]
    pub access_key: Option<String>,

    /// AWS Secret
    #[structopt(short, long, help = "AWS Secret Key", env = "AWS_SECRET_ACCESS_KEY")]
    pub secret_key: Option<String>,

    /// Method
    #[structopt(short = "X", long, help = "Request Method", default_value ="GET")]
    pub method: String,

    /// Header
    #[structopt(short = "H", long, help = "HTTP Headers")]
    pub header: Vec<String>,

    /// AWS SigV4
    #[structopt(long, help = "AWS SigV4 Option, format is region:service")]
    pub aws_sigv4: Option<String>,

    /// Region
    #[structopt(long, help = "AWS Service (alternate way to build --aws-sigv4)", requires_all(&["service"]))]
    pub region: Option<String>,

    /// AWS Setrvice
    #[structopt(long, help = "AWS Service (alternate way to build --aws-sigv4)", requires_all(&["region"]))]
    pub service: Option<String>,

    /// Enable Curl Wrapper (libcurl)
    #[structopt(long, help = "Enable libcurl")]
    pub disable_libcurl: bool,

    /// Post Data
    #[structopt(short = "d", long, help = "POST data body")]
    pub post_data: Option<String>,

    /// Url
    #[structopt(help = "Target Url")]
    pub url: String,
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
impl Opt {  
    fn auto_urlencode(post_data: Option<&String>, enable_aws_sigv4: bool, verbose: bool) -> String {
        match post_data {
            Some(s) => {
                if enable_aws_sigv4 {
                    let decoded = decode(s.as_str()).unwrap();
                    // Check if post data is already URL Encoded, 
                    if decoded.len() < s.len() {
                        if verbose  {
                            eprintln!("* Post data is already URI encoded for AWS SigV4");
                        }
                        s.to_owned()
                    } else {
                        eprintln!("* Post data must be URI encoded for AWS SigV4");
                        encode(&s).to_string().to_owned()
                    }
                } else {
                    s.to_owned()
                }
            },
            None => { String::new() }
        }
    }
    pub fn get_postdata(&self) -> String {
        Opt::auto_urlencode(self.post_data.as_ref(), 
            self.aws_sigv4.is_some() || self.access_key.is_some() || self.secret_key.is_some(), 
            self.verbose > 0).to_owned()
    }
}
