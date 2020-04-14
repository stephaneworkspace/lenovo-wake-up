use wake_on_lan;

fn main() {
    println!("Wakeup my server");
    let mac_adress: [u8; 6] = [0x44, 0x8A, 0x5B, 0x7F, 0x9B, 0x7F];
    let magic_packet = wake_on_lan::MagicPacket::new(&mac_adress);
    match magic_packet.send() {
        Err(e) => println!("{:?}", e),
        _ => (),
    }
}
