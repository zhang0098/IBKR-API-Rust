//! Binary for manually testing crate

use log::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use twsapi::core::client::EClient;
use twsapi::core::{errors::*, streamer::TcpStreamer};
use twsapi::examples::test_helpers::TestWrapper;

use chrono::Local;
use env_logger;
use env_logger::Builder;
use log::LevelFilter;
use std::file;
use std::fs::File;
use std::io::Write;
/// Example of using client and wrapper.
/// Requires a running instance of TWS or IB Gateway connected to the port in main.
/// Upon connecting, TWS will send the next valid order ID which will cause the wrapper callback method
/// next_valid_id to be called, which will start sending tests requests to TWS (see the
/// start_requests function inn TestWrapper which is called by next_valid_id
//==================================================================================================
pub fn main() -> Result<(), IBKRApiLibError> {
    let target = Box::new(File::create("./output.log").expect("Can't create file"));
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{}:{} {} [{}] - {}",
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(target))
        .filter(None, LevelFilter::Info)
        .init();

    let wrapper = Arc::new(Mutex::new(TestWrapper::<TcpStreamer>::new()));
    let app = Arc::new(Mutex::new(EClient::new(wrapper.clone())));

    info!("getting connection...");

    wrapper.lock().expect("Wrapper mutex was poisoned").client = Option::from(app.clone());

    //use port 7497 for TWS or 4002 for IB Gateway, depending on the port you have set
    app.lock()
        .expect("EClient mutex was poisoned")
        .connect("127.0.0.1", 4002, 0)?;

    thread::sleep(Duration::new(2, 0));

    Ok(())
}
