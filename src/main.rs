use anyhow::Context;
use thiserror::Error;

type Result<T, E = MyErr> = core::result::Result<T, E>;

#[derive(Debug, Error)]
enum MyErr {
    #[error("NotFound: {0}")]
    NotFound(String),
    #[error("InvalidParams: {0:?}")]
    InvalidParams(Vec<String>),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
trait Hoge {
    fn hoge(&self) -> (u32, String);
}
impl Hoge for MyErr {
    fn hoge(&self) -> (u32, String) {
        match self {
            MyErr::NotFound(_) => (404, self.to_string()),
            MyErr::InvalidParams(_) => (400, self.to_string()),
            MyErr::Other(_) => (500, self.to_string()),
        }
    }
}

fn err_test() -> Result<u32> {
    let ans = err_test2()?;
    let file_name = "hoge.png";
    std::fs::File::open(file_name).with_context(|| format!("failed to open file {}", file_name))?;
    Ok(ans)
}

fn err_test2() -> Result<u32> {
    let _hoge = Err(MyErr::NotFound(String::from("/users/delete/all")))?;
    let _hoge = Err(MyErr::InvalidParams(vec![
        String::from("User.name"),
        String::from("User.age"),
    ]))?;
    Ok(10)
}

fn main() {
    match err_test() {
        Ok(num) => println!("OK: {}", num),
        Err(err) => println!("ERR!!: [{}] {}", err.hoge().0, err.hoge().1),
    }
}

