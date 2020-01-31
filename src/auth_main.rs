
use crate::util::url_parser;

const RETURN_HTML: &str = include_str!("assets/site/index.html");
const AUTH_URL : &str =
 "https://auth.tdameritrade.com/auth?response_type=code&redirect_uri=http://127.0.0.1:8000&client_id=UTWO0CNUAERNK4OQDM5QC3LED4IAJBMU@AMER.OAUTHAP";

/*
 * This is a small implementation of a server meant to serve as the callback
 * for my little app on TD Ameritrade.
 *
 * When the refresh token has expired, the user will be sent to
 *
 * https://auth.tdameritrade.com/auth?response_type=code&redirect_uri=http://127.0.0.1:5432&client_id=UTWO0CNUAERNK4OQDM5QC3LED4IAJBMU@AMER.OAUTHAP
 *
 * in his/her web browser.
 *
 * Once logging in and giving the app permissions in the browser, Ameritrade
 * will redirect to this server and provide the code used to retrieve an access
 * token.
 *
 * This server will then store this information in a database for later access
 * by the rest of the apps.
 *
 *
 * The requset sent to the mini webserver comes in the formm of an HTTP request
 * that follows this CURL command:
 *
 * curl
 * 'https://127.0.0.1:5432/?code=NsYOaIraWzMJBCJnr0y9bIGb1%2F%2FHHW3cEoxMlJMWC4tWJPKYTl5tFslZr9ah7wUUaDzOM93Rf7h3oXl4eGoLInkfccYXxRdkm%2B%2FQmUm6rFkLkIazXrYZty283a5wKDEHBIU4XfZ7SJkILzrhTNvQntq1iAcFbKFmugji6stikEAe03GCd%2BbGGdKE9R7ZDAt1wFzsQRkekiRfp3IByRxKK3J1ilYsDwwXX4BY1ryELQxfA3ocV8ifCB7FZbsTg5JislS686%2BHcA0EadhpPnWkXbHPUw7LtVFggioBBGght8wVz6OyrcFtTs4wyPwi4OTw3yq47MbEOjsMVKjkFScFBszdbuYSukcmLG%2F1ZCoWy3PGe%2B9i5a34YTk7KGCe1mNMjTLFswU2mX99goZMj47QtcdcSWPYxvAm82OmxdLut4oOAT3GO9%2FJ2GqGg85100MQuG4LYrgoVi%2FJHHvlA91lDHYh3bvwcYCWWzFRTVKwPC8%2FfxYraH7thgJKEFClhfwR1enmQTOxtV3%2B7ZznEyya1ZxFePvB3VFg74O80miNGnkfirdYG%2FteXkUiPL5clT5wmiXyThtXXseNNbgFY1gqkdbXBn4q7lrYLOxqq9ALXQxRdINbY9BGZVlIQbBmcNLE8q6VVFV6MxWT5G8lv3NLhOPuB079WzmSV%2ByualGxH2JFjzM1XBPMFk%2FuQSD3HbCNUJ4Wf5NcspS6IZ2OomjHq2f3b3AGnFc%2FhKLkan7LawLkEpAc0T%2Ba2hfM2eJ%2Bue2dGYvp4OUtei2ob3cSFT3eSTxWramoXIkfC6hsnJKXKwl2Rr4RtKo%2BcogdhsXlQHxjfmi9DVUUIPqihoFkjeNou1wh5ac7YSNYWjS7GVsS2Wnv13ylsG2HdTLQYixZgJo0zN%2FrUkS%2BFug%3D212FD3x19z9sWBHDJACbC00B75E'
 * -H 'Origin: https://auth.tdameritrade.com' -H 'Upgrade-Insecure-Requests: 1'
 * -H 'Content-Type: application/x-www-form-urlencoded' -H 'User-Agent:
 * Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko)
 * Chrome/79.0.3945.130 Safari/537.36' -H 'Sec-Fetch-User: ?1' -H 'Referer:
 * https://auth.tdameritrade.com/oauth?client_id=UTWO0CNUAERNK4OQDM5QC3LED4IAJBMU@AMER.OAUTHAP&response_type=code&redirect_uri=http://127.0.0.1:5432'
 * --compressed
 */
pub fn authentication_main() {

    // Configure the server to listen on port 8000. Rumor has it that some
    // web-browsers will trust port 8000 on localhost.
    let server = tiny_http::Server::new(tiny_http::ServerConfig {
        addr: "0.0.0.0:8000",

        // Include the SSL certificates to provide SSL because TD ameritrade
        // will only redirect to SSL sites.
        ssl: Some(tiny_http::SslConfig {
            private_key: include_bytes!("assets/cert/key.pem").to_vec(),
            certificate: include_bytes!("assets/cert/cert.pem").to_vec(),
        }),
    })
    .unwrap();

    // Spawn Google Chrome to the TD-Ameritrade Auth page.
    std::thread::spawn(move || run_webbrowser());

    // Start running the webserver.
    loop {
        match server.recv() {
            Ok(rq) => {
                handle(rq);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        };
    }
}

/// Starts an instance of chrome that opens up a connection to TD Ameritrade
/// to prompt the user to give permission for the app to use the developer API.
fn run_webbrowser() {
    std::process::Command::new("google-chrome-stable")
        .arg(String::from("--app=") + &String::from(AUTH_URL))
        .output()
        .expect("Failed to run web browser.");
}

/// Handles the request for this webserver.
fn handle(req: tiny_http::Request) {
    let url: url_parser::Url = url_parser::parse_url(req.url());
    match url_parser::get_param(&url, &"code") {
        Some(code) => {
            let _ = req.respond(handle_code(&code));
        }
        None => println!("Expected code query parameter, got nothing."),
    }
}

fn handle_code(code: &String) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    if let Some(proj_dirs)  = directories::ProjectDirs::from("com", "r", "tdsh") {
        let dir = proj_dirs.config_dir();

        println!("Write to directory {:#?}", dir);
    } else {
        println!("Unable to get Config Directory.");
    }

    let mut resp = tiny_http::Response::from_string(
        RETURN_HTML.replace("${code}", &chunk(&code)));
    resp.add_header(tiny_http::Header::from_bytes("content-type", "text/html").unwrap());
    resp
}

// Chunk the input string into line sizes to fit on the webpage.
fn chunk(input : &str) -> String {
    let mut ret = String::new();
    let len = input.chars().count();

    let mut i = 0;

    const CHUNK_SIZE: usize = 61;

    loop {
        if (i + CHUNK_SIZE) >= len {
            ret += &input[i..];
            break ret;
        }
        ret += &input[i..i+CHUNK_SIZE];
        ret += "<br/>";
        i += CHUNK_SIZE;
    }
}
