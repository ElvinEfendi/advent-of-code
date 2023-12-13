use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Element(pub String);

#[derive(PartialEq, Debug)]
struct Node(Element, Element);

#[derive(Debug, PartialEq)]
enum Turn {
    Left,
    Right,
}

pub struct Network {
    node_by_element: HashMap<Element, Node>,
    turns: Vec<Turn>,
}

impl Network {
    fn new() -> Self {
        Self {
            node_by_element: HashMap::new(),
            turns: Vec::new(),
        }
    }

    fn add_node(&mut self, element: Element, node: Node) {
        self.node_by_element.insert(element, node);
    }

    pub fn distance(&self, from: Element, to: Element) -> Option<usize> {
        let mut distance = 0;
        let mut current = from;
        loop {
            let node = self.node_by_element.get(&current)?;

            let next = match self.turns.get(distance % self.turns.len()) {
                Some(Turn::Left) => &node.0,
                Some(Turn::Right) => &node.1,
                None => return None,
            };
            distance += 1;

            if *next == to {
                return Some(distance);
            }

            current = next.clone();
        }
    }

    pub fn distance_from_as_to_zs(&self) -> usize {
        let mut distance = 0;
        let mut current = self.node_by_element
            .keys()
            .filter(|element| element.0.ends_with('A'))
            .collect::<Vec<_>>();

        loop {
            let nodes = current
                .iter()
                .map(|element| self.node_by_element.get(element).unwrap())
                .collect::<Vec<_>>();
            let next = match self.turns.get(distance % self.turns.len()) {
                Some(Turn::Left) => nodes.iter().map(|node| &node.0).collect::<Vec<_>>(),
                Some(Turn::Right) => nodes.iter().map(|node| &node.1).collect::<Vec<_>>(),
                None => panic!("No turns"),
            };
            distance += 1;

            if next.iter().all(|element| element.0.ends_with('Z')) {
                return distance;
            }

            current = next;
        }
    }
}

impl From<&str> for Network {
    fn from(input: &str) -> Self {
        let mut network = Network::new();

        let mut lines = input.lines();

        let turns_str = lines.next().unwrap();
        let turns = turns_str.chars().map(|c| match c {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => panic!("Invalid turn"),
        });
        network.turns.extend(turns);

        lines.next();

        for line in lines {
            let mut parts = line.split(" = ");

            let element = Element(parts.next().unwrap().into());

            let node_str = parts.next().unwrap().trim_matches(|c| c == '(' || c == ')');
            let mut node_parts = node_str.split(", ");
            let node = Node(Element(node_parts.next().unwrap().into()), Element(node_parts.next().unwrap().into()));

            network.add_node(element, node);
        }

        network
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_distance() {
        let mut network = Network::new();
        network.add_node(Element("AAA".into()), Node(Element("BBB".into()), Element("BBB".into())));
        network.add_node(Element("BBB".into()), Node(Element("AAA".into()), Element("ZZZ".into())));
        network.add_node(Element("ZZZ".into()), Node(Element("ZZZ".into()), Element("ZZZ".into())));
        network.turns.push(Turn::Left);
        network.turns.push(Turn::Left);
        network.turns.push(Turn::Right);
        let distance = network.distance(Element("AAA".into()), Element("ZZZ".into()));

        assert_eq!(distance, Some(6));
    }

    #[test]
    fn test_network_from_str() {
        let input = "\
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let network = Network::from(input);

        assert_eq!(network.turns, vec![Turn::Left, Turn::Left, Turn::Right]);
        assert_eq!(network.node_by_element.len(), 3);
        assert_eq!(network.node_by_element.get(&Element("AAA".into())), Some(&Node(Element("BBB".into()), Element("BBB".into()))));

        let distance = network.distance(Element("AAA".into()), Element("ZZZ".into()));
        assert_eq!(distance, Some(6));
    }

    #[test]
    fn test_network_distance_from_as_to_zs() {
        let input = "\
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let network = Network::from(input);

        assert_eq!(network.distance_from_as_to_zs(), 6);
    }
}
