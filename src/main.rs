use bigdecimal::BigDecimal;
use std::collections::BTreeSet;
use std::cmp::Ordering;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Entry {
    id: String,
    value: BigDecimal
}

impl Entry {
    fn new(id: String, value: BigDecimal) -> Entry {
        Entry { id, value }
    }
}

#[derive(Debug)]
struct WrapEntry(Entry);

impl WrapEntry {
    fn new(e: Entry) -> WrapEntry {
        WrapEntry(e)
    }
}

impl Ord for WrapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

impl PartialOrd for WrapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Eq for WrapEntry {}

impl PartialEq for WrapEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.0.id == other.0.id {
            if self.0.value == other.0.value {
                true
            } else { false }
        } else { false }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Start");

    let mut set: BTreeSet<WrapEntry> = BTreeSet::new();

    let a1 = WrapEntry::new(Entry::new(
        "aaa".to_string(), BigDecimal::from(100)));
    let a2 = WrapEntry::new(Entry::new(
        "aaa".to_string(), BigDecimal::from(100)));
    let a3 = WrapEntry::new(Entry::new(
        "aaa".to_string(), BigDecimal::from(100)));
    let b1 = WrapEntry::new(Entry::new(
        "bbb".to_string(), BigDecimal::from(100)));
    let b2 = WrapEntry::new(Entry::new(
        "bbb".to_string(), BigDecimal::from(100)));
    let b3 = WrapEntry::new(Entry::new(
        "bbb".to_string(), BigDecimal::from(100)));
    let c1 = WrapEntry::new(Entry::new(
        "ccc".to_string(), BigDecimal::from(100)));

    set.replace(a1);
    set.replace(b1);
    set.replace(c1);

    for i in set.iter() {
        println!("{:?}", i);
    }

    Ok(())
}
