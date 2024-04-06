mod types;
mod utils;

use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    /// The password to use for encryption.
    #[clap(short, long)]
    password: String,
    /// The JSON string representing the folders. Example:
    /// '[{"id":"00000000-0000-0000-0000-000000000001","name":"My Folder"}]'
    #[clap(short, long)]
    folders: String,
    /// The JSON string representing the ciphers. Example:
    /// '[{"folderId":"00000000-0000-0000-0000-000000000001","name":"My Test","notes":"My Notes","username":"my_username","password":"my_password","loginUris":["https://example.com"]}]'
    #[clap(short, long)]
    ciphers: String,
}

fn main() {
    // Parse the command line arguments.
    let args: Args = Args::parse();
    let json_folders_str = args.folders;
    let json_ciphers_str = args.ciphers;
    let password = args.password;

    // Generate the encrypted JSON.
    let json_str =
        match utils::generate_encrypted_json(&json_folders_str, &json_ciphers_str, &password) {
            Ok(json_str) => json_str,
            Err(e) => {
                eprintln!("Error: {}", e);
                return;
            }
        };

    println!("{}", json_str);
}
