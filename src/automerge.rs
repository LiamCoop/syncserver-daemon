/*
fn fun_exercise() -> Result<(), Box<dyn std::error::Error>> {
    let saved = std::fs::read("")?;

    // Load the document on the first device and change alices email
    let mut doc1 = AutoCommit::load(&saved)?;
    let contacts = match doc1.get(automerge::ROOT, "contacts")? {
        Some((automerge::Value::Object(ObjType::List), contacts)) => contacts,
        _ => panic!("contacts should be a list"),
    };
    let alice = match doc1.get(&contacts, 0)? {
        Some((automerge::Value::Object(ObjType::Map), alice)) => alice,
        _ => panic!("alice should be a map"),
    };
    doc1.put(&alice, "email", "alicesnewemail@example.com")?;

    // Load the document on the second device and change bobs name
    let mut doc2 = AutoCommit::load(&saved)?;
    let contacts = match doc2.get(automerge::ROOT, "contacts")? {
        Some((automerge::Value::Object(ObjType::List), contacts)) => contacts,
        _ => panic!("contacts should be a list"),
    };
    let bob = match doc2.get(&contacts, 1)? {
        Some((automerge::Value::Object(ObjType::Map), bob)) => bob,
        _ => panic!("bob should be a map"),
    };
    doc2.put(&bob, "name", "Robert")?;

    // Finally, we can merge the changes from the two devices
    doc1.merge(&mut doc2)?;
    let bobsname: Option<automerge::Value> = doc1.get(&bob, "name")?.map(|(v, _)| v);
    assert_eq!(
        bobsname,
        Some(automerge::Value::Scalar(Cow::Owned("Robert".into())))
    );

    let alices_email: Option<automerge::Value> = doc1.get(&alice, "email")?.map(|(v, _)| v);
    assert_eq!(
        alices_email,
        Some(automerge::Value::Scalar(Cow::Owned(
            "alicesnewemail@example.com".into()
        )))
    );
    Ok(())
}
*/
