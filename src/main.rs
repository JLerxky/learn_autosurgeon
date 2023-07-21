use autosurgeon::{hydrate, reconcile, Hydrate, Reconcile};

// A simple contact document

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq)]
struct Contact {
    name: String,
    address: Address,
}

#[derive(Debug, Clone, Reconcile, Hydrate, PartialEq)]
struct Address {
    line_one: String,
    line_two: Option<String>,
    city: String,
    postcode: String,
}

fn main() {
    let mut contact = Contact {
        name: "Sherlock Holmes".to_string(),
        address: Address {
            line_one: "221B Baker St".to_string(),
            line_two: None,
            city: "London".to_string(),
            postcode: "NW1 6XE".to_string(),
        },
    };

    // Put data into a document
    let mut doc = automerge::AutoCommit::new();
    reconcile(&mut doc, &contact).unwrap();

    // Get data out of a document
    let contact2: Contact = hydrate(&doc).unwrap();
    assert_eq!(contact, contact2);

    // Fork and make changes
    let mut doc2 = doc
        .fork()
        .with_actor(automerge::ActorId::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));
    let mut contact2: Contact = hydrate(&doc2).unwrap();
    contact2.name = "Dangermouse3".to_string();
    reconcile(&mut doc2, &contact2).unwrap();

    let mut doc3 = doc
        .fork()
        .with_actor(automerge::ActorId::from(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 9]));
    let mut contact3: Contact = hydrate(&doc3).unwrap();
    contact3.name = "Dangermouse".to_string();
    reconcile(&mut doc3, &contact3).unwrap();

    // Concurrently on doc1
    contact.address.line_one = "221C Baker St".to_string();
    reconcile(&mut doc, &contact).unwrap();

    // Now merge the documents
    // Reconciled changes will merge in somewhat sensible ways
    doc.merge(&mut doc2).unwrap();
    doc.merge(&mut doc3).unwrap();

    let merged: Contact = hydrate(&doc).unwrap();
    
    assert_eq!(
        merged,
        Contact {
            name: "Dangermouse3".to_string(), // This was updated in the first doc
            address: Address {
                line_one: "221C Baker St".to_string(), // This was concurrently updated in doc2
                line_two: None,
                city: "London".to_string(),
                postcode: "NW1 6XE".to_string(),
            }
        }
    )
}
