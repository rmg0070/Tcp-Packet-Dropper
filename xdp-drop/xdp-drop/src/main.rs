// use anyhow::Context;
// use aya::{
//     include_bytes_aligned,
//     maps::HashMap,
//     programs::{Xdp, XdpFlags},
//     Bpf,
// };
// use aya_log::BpfLogger;
// use clap::Parser;
// use log::{info, warn};
// use std::net::Ipv4Addr;
// use tokio::signal;

// #[derive(Debug, Parser)]
// struct Opt {
//     #[clap(short, long, default_value = "enp0s3")]
//     iface: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//     let opt = Opt::parse();

//     env_logger::init();

//     // This will include your eBPF object file as raw bytes at compile-time and load it at
//     // runtime. This approach is recommended for most real-world use cases. If you would
//     // like to specify the eBPF program at runtime rather than at compile-time, you can
//     // reach for `Bpf::load_file` instead.
//     #[cfg(debug_assertions)]
//     let mut bpf = Bpf::load(include_bytes_aligned!(
//         "../../target/bpfel-unknown-none/debug/xdp-drop"
//     ))?;
//     #[cfg(not(debug_assertions))]
//     let mut bpf = Bpf::load(include_bytes_aligned!(
//         "../../target/bpfel-unknown-none/release/xdp-drop"
//     ))?;
//     if let Err(e) = BpfLogger::init(&mut bpf) {
//         // This can happen if you remove all log statements from your eBPF program.
//         warn!("failed to initialize eBPF logger: {}", e);
//     }
//     let program: &mut Xdp =
//         bpf.program_mut("xdp_firewall").unwrap().try_into()?;
//     program.load()?;
//     program.attach(&opt.iface, XdpFlags::default())
//         .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

//     // (1)
//     let mut blocklist: HashMap<_, u32, u32> =
//         HashMap::try_from(bpf.map_mut("BLOCKLIST").unwrap())?;

//     // (2) 108.177.122.99,0.0.2.15/152.199.4.33,
//     let block_addr: u32 = Ipv4Addr::new(172 , 20, 29, 214).try_into()?;

//     // (3)
//     blocklist.insert(block_addr, 0, 0)?;

//     info!("Waiting for Ctrl-C...");
//     signal::ctrl_c().await?;
//     info!("Exiting...");

//     Ok(())
// }


// use anyhow::Context;
// use aya::{
//     include_bytes_aligned,
//     maps::HashMap,
//     programs::{Xdp, XdpFlags},
//     Bpf,
// };
// use aya_log::BpfLogger;
// use clap::Parser;
// use log::{info, warn};
// use std::net::Ipv4Addr;
// use tokio::signal;

// #[derive(Debug, Parser)]
// struct Opt {
//     #[clap(short, long, default_value = "enp0s3")]
//     iface: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//     let opt = Opt::parse();

//     env_logger::init();

//     // Load the eBPF program
//     #[cfg(debug_assertions)]
//     let mut bpf = Bpf::load(include_bytes_aligned!(
//         "../../target/bpfel-unknown-none/debug/xdp-drop"
//     ))?;
//     #[cfg(not(debug_assertions))]
//     let mut bpf = Bpf::load(include_bytes_aligned!(
//         "../../target/bpfel-unknown-none/release/xdp-drop"
//     ))?;
//     if let Err(e) = BpfLogger::init(&mut bpf) {
//         warn!("failed to initialize eBPF logger: {}", e);
//     }

//     let program: &mut Xdp = bpf.program_mut("xdp_firewall").unwrap().try_into()?;
//     program.load()?;
//     program.attach(&opt.iface, XdpFlags::default())
//         .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

//     // Blocklist for IP addresses
//     let mut blocklist: HashMap<_, u32, u32> = HashMap::try_from(bpf.map_mut("BLOCKLIST").unwrap())?;
//     let block_addr: u32 = Ipv4Addr::new(127, 0, 0, 1).try_into()?;
//     blocklist.insert(block_addr, 0, 0)?;

//     // (1) Blocklist for ports (e.g., port 443)
//     let mut port_blocklist: HashMap<_, u16, u16> = HashMap::try_from(bpf.map_mut("PORT_BLOCKLIST").unwrap())?;

//     // (2) Insert port 443 (HTTPS) into the port blocklist
//     let blocked_port: u16 = 6969;
//     port_blocklist.insert(blocked_port, 0, 0)?;


//     info!("Waiting for Ctrl-C...");
//     signal::ctrl_c().await?;
//     info!("Exiting...");

//     Ok(())
// }




// use anyhow::Context;
// use aya::{
//     include_bytes_aligned,
//     maps::HashMap,
//     programs::{Xdp, XdpFlags},
//     Bpf,
// };
// use aya_log::BpfLogger;
// use clap::Parser;
// use log::{info, warn};
// use std::net::Ipv4Addr;
// use tokio::signal;

// #[derive(Debug, Parser)]
// struct Opt {
//     #[clap(short, long, default_value = "enp0s3")]
//     iface: String,
// }

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
//     let opt = Opt::parse();

//     env_logger::init();

//     // Load the eBPF program
//     #[cfg(debug_assertions)]
//     let mut bpf = Bpf::load(include_bytes_aligned!(
//         "../../target/bpfel-unknown-none/debug/xdp-drop"
//     ))?;
//     #[cfg(not(debug_assertions))]
//     let mut bpf = Bpf::load(include_bytes_aligned!(
//         "../../target/bpfel-unknown-none/release/xdp-drop"
//     ))?;
//     if let Err(e) = BpfLogger::init(&mut bpf) {
//         warn!("failed to initialize eBPF logger: {}", e);
//     }

//     let program: &mut Xdp = bpf.program_mut("xdp_firewall").unwrap().try_into()?;
//     program.load()?;
//     program.attach(&opt.iface, XdpFlags::default())
//         .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

//     // HashMap for blocklists (for IPs and Ports)
//     let mut blocklist: HashMap<_, u32, u32> = HashMap::try_from(bpf.map_mut("BLOCKLIST").unwrap())?;
//     let mut port_blocklist: HashMap<_, u16, u16> = HashMap::try_from(bpf.map_mut("PORT_BLOCKLIST").unwrap())?;

//     // EXAMPLES:

//     // Example 1: Block a specific IP address
//     // In this example, we block the IP address 127.0.0.1 (localhost)
//     // let block_addr: u32 = Ipv4Addr::new(127, 0, 0, 1).try_into()?;
//     // blocklist.insert(block_addr, 0, 0)?;
//     // info!("Blocking IP: 127.0.0.1");

//     // Example 2: Block a specific port
//     // In this example, we block port 443 (HTTPS)
//     let blocked_port: u16 = 6969;
//     port_blocklist.insert(blocked_port, 0, 0)?;
//     info!("Blocking Port: 443 (HTTPS)");

//     // // Example 3: Block both a specific IP and a specific port
//     // // In this example, we block the IP address 192.168.1.10 and port 6969
//     // let block_addr_example3: u32 = Ipv4Addr::new(192, 168, 1, 10).try_into()?;
//     // let blocked_port_example3: u16 = 6969;
    
//     // // Insert both IP and Port into the respective blocklists
//     // blocklist.insert(block_addr_example3, 0, 0)?;
//     // port_blocklist.insert(blocked_port_example3, 0, 0)?;
//     // info!("Blocking IP: 192.168.1.10 and Port: 6969");

//     // Wait for Ctrl-C to exit the program
//     info!("Waiting for Ctrl-C...");
//     signal::ctrl_c().await?;
//     info!("Exiting...");

//     Ok(())
// }


use anyhow::Context;
use aya::{
    include_bytes_aligned,
    maps::{HashMap, MapData},
    programs::{Xdp, XdpFlags},
    Bpf,
};
use aya_log::BpfLogger;
use log::{info, warn};
use std::net::Ipv4Addr;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Initialize logging
    env_logger::init();

    // Load the eBPF program
    #[cfg(debug_assertions)]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/debug/xdp-drop"
    ))?;
    #[cfg(not(debug_assertions))]
    let mut bpf = Bpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/xdp-drop"
    ))?;
    if let Err(e) = BpfLogger::init(&mut bpf) {
        warn!("failed to initialize eBPF logger: {}", e);
    }

    let program: &mut Xdp = bpf.program_mut("xdp_firewall").unwrap().try_into()?;
    program.load()?;
    program.attach("enp0s3", XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    // Blocking IPs logic
    // {
    //     let mut blocklist: HashMap<_, u32, u32> = HashMap::try_from(bpf.map_mut("BLOCKLIST").unwrap())?;
    //     let block_addr: u32 = Ipv4Addr::new(192, 168, 1, 112).try_into()?;
    //     blocklist.insert(block_addr, 0, 0)?;
    //     // info!("Blocking IP: 127.0.0.1");
    // } // `blocklist` mutable borrow ends here

    // // Blocking ports logic
    {
        let mut port_blocklist: HashMap<_, u16, u16> = HashMap::try_from(bpf.map_mut("PORT_BLOCKLIST").unwrap())?;
        let blocked_port: u16 = 6969;
        port_blocklist.insert(blocked_port, 0, 0)?;
        info!("Blocking Port: 443 (HTTPS)");
    } // `port_blocklist` mutable borrow ends here

    // // Blocking both IPs and ports
    // {
    //     let mut blocklist: HashMap<_, u32, u32> = HashMap::try_from(bpf.map_mut("BLOCKLIST").unwrap())?;
    //     let mut port_blocklist: HashMap<_, u16, u16> = HashMap::try_from(bpf.map_mut("PORT_BLOCKLIST").unwrap())?;

    //     let block_addr: u32 = Ipv4Addr::new(192, 168, 1, 10).try_into()?;
    //     let blocked_port: u16 = 6969;

    //     blocklist.insert(block_addr, 0, 0)?;
    //     port_blocklist.insert(blocked_port, 0, 0)?;
    //     info!("Blocking IP: 192.168.1.10 and Port: 6969");
    // }

    // Wait for Ctrl-C to exit the program
    info!("Waiting for Ctrl-C...");
    signal::ctrl_c().await?;
    info!("Exiting...");

    Ok(())
}
