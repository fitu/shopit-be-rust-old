use rocket::http::Status;
use mongodb::error::Error;
use rocket::response::{content, status};
use mongodb::{options::ClientOptions, Client};

pub async fn get_products() -> Result<status::Custom<content::Json<&'static str>>, Error> {
    let uri = "mongodb+srv://fitulp:computadorar@shopit.mzyfm.mongodb.net/shopit?retryWrites=true&w=majority";
    let options = ClientOptions::parse(uri).await?;
    let client = Client::with_options(options)?;
    let _dbs = client.list_database_names(None, None);
    // println!("{}", _dbs);

    let _products_string = r#"{"success":true,"productCount":2,"products":[{"price":45.89,"ratings":4.5,"numOfReviews":32,"_id":"61681fcbd7674b0013e9c166","name":"SanDisk Ultra 128GB SDXC UHS-I Memory Card up to 80MB/s","description":"Ultra-fast cards (2) to take better pictures and Full HD videos (1) with your compact to mid-range point-and-shoot cameras and camcorders. With SanDisk Ultra SDXC UHS-I cards you’ll benefit from faster downloads, high capacity, and better performance to capture and store 128GB (5) of high quality pictures and Full HD video (1). Take advantage of ultra-fast read speeds of up to 80MB/s (3) to save time moving photos and videos from the card to your computer. From a leader in flash memory storage, SanDisk Ultra SDXC UHS-I cards are compatible with SDHC and SDXC digital devices, and come with a 10-year limited warranty (6).","images":[{"_id":"61681fcbd7674b0013e9c167","public_id":"products/dsvbpny402gelwugv2le","url":"https://res.cloudinary.com/bookit/image/upload/v1608062030/products/dsvbpny402gelwugv2le.jpg"}],"category":"Electronics","seller":"Ebay","stock":50,"reviews":[],"user":"61681fcbd7674b0013e9c164","createdAt":"2021-10-14T12:17:15.434Z","__v":0},{"price":315,"ratings":1.65,"numOfReviews":2,"_id":"61681fcbd7674b0013e9c168","name":"CAN USB FD Adapter (GC-CAN-USB-FD)","description":"Monitor a CAN network, write a CAN program and communicate with industrial, medical, automotive or other CAN based device. Connect CAN FD and CAN networks to a computer via USB with the CAN USB FD adapter.","images":[{"_id":"61681fcbd7674b0013e9c169","public_id":"products/61oXGZ60GfL_fixco9","url":"https://res.cloudinary.com/bookit/image/upload/v1614877995/products/61oXGZ60GfL_fixco9.jpg"}],"category":"Electronics","seller":"Amazon","stock":0,"reviews":[],"user":"61681fcbd7674b0013e9c165","createdAt":"2021-10-14T12:17:15.435Z","__v":0}]}"#;
    Ok(status::Custom(Status::Ok, content::Json(_products_string)))
}
