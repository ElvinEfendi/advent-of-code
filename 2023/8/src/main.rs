use day_8::network;

fn main() {
    let input = include_str!("../input.txt");
    let network = network::Network::from(input);
    //println!("Distance from AA to ZZ: {}", network.distance(network::Element("AAA".into()), network::Element("ZZZ".into())).unwrap());
    println!("Part 2: {}", network.distance_from_as_to_zs());
}
