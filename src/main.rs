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

fn main() -> io::Result<()> {
    let iface = tun_tap_new::Iface::new("", tun_tap_new::Mode::Tun);
    match iface {
        Ok(iface) => {
            println!("Interface `{}` successfully created", iface.name());
            // Configure the device ‒ set IP address on it, bring it up.
            println!("Setting IP address");
            exec(
                "ip",
                &["addr", "add", "192.168.0.1/24", "dev", iface.name()],
            );
            println!("Link up");
            exec("ip", &["link", "set", "up", "dev", iface.name()]);
            loop {
                let mut buffer = vec![0u8; 1504]; // MTU + 4 for the header
                let nbytes = iface.recv(&mut buffer).unwrap();

                let header: ip::Header = ip::Header::new(&buffer);
                if header.proto != 0x0800 {
                    continue;
                }
                println!("Received {} bytes: {:x?}", nbytes, &buffer[..nbytes]);
                println!(
                    "Flags: {:x?} Proto: {:x?} IP: v{} IHL: {} dw",
                    header.flags, header.proto, header.version, header.ihl
                );
                println!(
                    "{} -> {} {} {} id=0x{:x?}, seq={}/{}, ttl={}",
                    header.sender,
                    header.receiver,
                    ip::aipn_to_str(header.protocol),
                    header.tot_len,
                    header.id,
                    header.seq,
                    header.inv_seq,
                    header.ttl
                );
            }
        }
        Err(e) => {
            eprintln!("Interface not created with error: {}", e);
        }
    }
    Ok(())
}
