/// Parse command line options
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub enum Content {
    #[structopt(name = "word")]
    Word {
        #[structopt(name = "WORD")]
        words: Vec<String>,
    },

    #[structopt(name = "sentence")]
    Sentence {
        #[structopt(name = "SENTENCE")]
        sentences: Vec<String>,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "yd", about = "A terminal dictionary app")]
pub struct Opt {
    #[structopt(subcommand)]
    pub content: Content,

    #[structopt(short, long)]
    pub debug: bool,

    #[structopt(short, long)]
    pub quiet: bool,
}

pub fn get_opt() -> Opt {
    Opt::from_args()
}

#[cfg(test)]
mod test {
    use structopt::StructOpt;

    use super::*;

    #[test]
    fn test_parse() {
        let opt = Opt::from_args();
        println!("{:#?}", opt);
    }
}
