use error_chain::error_chain;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    /* We are making a request to the mentioned URL and storing the response in the res varibale.
    The blocking Client will block the current thread to execute, instead of returning futures that need to be executed on a runtime. */
    let mut res = reqwest::blocking::get("http://httpbin.org/get")?;

    let mut body = String::new();

    /* Read all bytes until EOF in this source, appending them to buf.
    If successful, this function returns the number of bytes which were read and appended to buf */
    res.read_to_string(&mut body)?;

    // Printing the contents we would like to see on the terminal.
    println!("Status: {}", res.status());
    println!("Headers: \n{:#?}", res.headers());
    println!("Body: \n{}", body);
    Ok(())
}
