use augur::Augur;

fn main()
{
    let aug: Augur = Augur::verbose();
    for i in 0..9 {
        aug.log("Test", i);
    };
}
