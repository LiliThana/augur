use augur::Augur;

fn main()
{
    let aug = Augur::new();
    let aug = match aug {
        Ok(t) => t,
        Err(e) => panic!("Error creating Augur: {:?}", e)
    };
    for i in 0..9 {
        aug.log("Test", i);
    };
}
