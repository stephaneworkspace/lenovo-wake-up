use wake_on_lan;

fn main() {
    println!("Wakeup nas");
    let mac_adress: [u8; 6] = [0x00, 0x90, 0xA9, 0xF4, 0xD0, 0x5A];
    let magic_packet = wake_on_lan::MagicPacket::new(&mac_adress);
    match magic_packet.send() {
        Err(e) => eprintln!("{:?}", e),
        _ => eprintln!("packet send"),
    }
}
