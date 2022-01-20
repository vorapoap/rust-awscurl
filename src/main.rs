use structopt::StructOpt;

mod opt;
mod curl_wrapper;


fn main() {
    let opt = opt::Opt::from_args();
    curl_wrapper::curl_request(&opt)
}
