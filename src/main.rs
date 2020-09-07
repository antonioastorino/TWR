use std::io;
mod ip;

/// Run a shell command. Panic if it fails in any way.
fn exec(cmd: &str, args: &[&str]) {
    let ecode = std::process::Command::new(&cmd)
        .args(args)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    assert!(ecode.success(), "Failed to execte {}", cmd);
}

fn init_iface(iface_name: &str, ip_addr: &'static str) {
            exec(
                "ip",
                &["addr", "add", ip_addr, "dev", iface_name],
            );
            exec("ip", &["link", "set", "up", "dev", iface_name]);
}

fn main() -> io::Result<()> {
    let iface = tun_tap_new::Iface::new("", tun_tap_new::Mode::Tun);
    match iface {
        Ok(iface) => {
            println!("Interface `{}` successfully created", iface.name());
            // Configure the device ‒ set IP address on it, bring it up.
            init_iface(iface.name(), "192.168.0.1/24");
            loop {
                let mut buffer = vec![0u8; 1504]; // MTU + 4 for the header
                let nbytes = iface.recv(&mut buffer).unwrap();
                let header: ip::CommonHeader = ip::CommonHeader::new(&buffer);
                if header.proto != 0x0800 {
                    continue;
                }
                header.print_header();
                match header.protocol {
                    1 => {
                        let specific_header = ip::IcmpHeader::new(&buffer);
                        ip::print_header(&header, &specific_header);
                    }
                    6 => {
                        let specific_header = ip::TcpHeader::new(&buffer);
                        ip::print_header(&header, &specific_header);
                        println!("{:x?}", &buffer[..nbytes]);
                    } 
                    _ => {
                        println!("Unknown protocol");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Interface not created with error: {}", e);
        }
    }
    Ok(())
}
