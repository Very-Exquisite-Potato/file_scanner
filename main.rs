use warp::Filter;

pub mod features;
    use warp::multipart::FormData;
    use futures::StreamExt;
    use bytes::Buf;

    pub async fn handle_file_upload(form: FormData) -> Result<impl warp::Reply, warp::Rejection> 
    {
        form.map(|part| async move 
            {
                if let Ok(part) = part 
                {
                    if let Some(filename) = part.filename() 
                    {
                        let filename = filename.to_string();
                        let mut data = part.stream();

                        while let Some(chunk_result) = data.next().await 
                        {
                            match chunk_result 
                            {
                                Ok(mut chunk) => 
                                {
                                    // Convert chunk to bytes for proper handling
                                    let bytes = chunk.copy_to_bytes(chunk.remaining());
                                    println!("Writing chunk for file {}: {:?}", filename, bytes);
                                }
                                Err(e) => 
                                {
                                    eprintln!("Failed to process chunk for file {}: {:?}", filename, e);
                                }
                            }
                        }
                    } 
                    else 
                    {
                        eprintln!("File part does not have a filename.");
                    }
                } 
                else 
                {
                    eprintln!("Failed to process a part of the form.");
                }
            })
            .buffer_unordered(10) // Process up to 10 files concurrently
            .collect::<Vec<_>>()
            .await;

    Ok(warp::reply::with_status(
        "File uploaded",
        warp::http::StatusCode::OK,
    ))
}

#[tokio::main]
async fn main() 
{
    warp::serve(
        warp::post()
            .and(warp::path("upload"))
            .and(warp::multipart::form())
            .and_then(features::handle_file_upload),
    )
    .run(([127, 0, 0, 1], 3030))
    .await;
}
