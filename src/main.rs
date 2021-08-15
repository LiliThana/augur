use augur::Augur;

fn main()
{
    let aug: Augur = Augur::new();
    for i in 0..9 {
        aug.log("Test", i);
    };
}
