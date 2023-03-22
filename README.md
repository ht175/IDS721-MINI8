# rust-new-project-template
A good starting point for a new Rust project

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)

# Rust Stock Fetcher
This is a simple Rust project that fetches the top stock names from Yahoo Finance.

# Introduction
This project uses the Reqwest crate to send an HTTP GET request to the Yahoo Finance website and fetches the HTML content of the page. It then parses the HTML to extract the top stock names and outputs them to the console.

# Usage
To use this project, you'll need to have Rust installed on your computer. You can download and install Rust from the official Rust website: https://www.rust-lang.org/tools/install

Once you have Rust installed, you can download and build the project using Cargo:
##
`git clone https://github.com/your-username/IDS_RUST_MINI8.git`
`cd myproject`
`cargo build`

This will download and install the Reqwest crate and build the project.

To run the project, use the following command:
`cargo run`

This will fetch the top stock names from Yahoo Finance and output them to the console.
![image](https://user-images.githubusercontent.com/122952572/227042081-3ef4e751-f1f2-432a-aab3-17ff5e92256c.png)
