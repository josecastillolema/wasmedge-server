use hyper::{Client, Uri};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client instance.
    let client = Client::new();

    // Define the URL you want to make a request to.
    let url: Uri = "http://httpbin.org/get".parse()?;

    // Make the GET request asynchronously.
    let res = client.get(url).await?;

    // Print the response status.
    println!("Response: {}", res.status());

    // Get the response body and print it as a string.
    let body = hyper::body::to_bytes(res.into_body()).await?;
    println!("Body: {:?}", body);

    Ok(())
}
