use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use hyper::{Request, Body, Response};
use lazy_static::lazy_static;
use futures::TryStreamExt;

use crate::shortener::shorten_url;

type UrlDb = Arc<RwLock<HashMap<String, String>>>;

lazy_static! {
    static ref SHORT_URLS: UrlDb = Arc::new(RwLock::new(HashMap::new()));
}

// to_bytes version
// use hyper::body::to_bytes;
//
// pub(crate) async fn url_service(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
//     let body = to_bytes(req.into_body()).await?;
//     let c = body.iter().cloned().collect::<Vec<u8>>();
//     let url_to_shorten = std::str::from_utf8(&c).unwrap();
//     let shortened_url = shorten_url(url_to_shorten);
//     SHORT_URLS.write().unwrap().insert(shortened_url, url_to_shorten.to_string());
//     let a = &*SHORT_URLS.read().unwrap();
//     Ok(Response::new(Body::from(format!("{:#?}", a))))
// }

pub(crate) async fn url_service(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let reply = req
        .into_body()
        .map_ok(|chunk| {
            let c = chunk.iter().map(|byte| byte.to_owned()).collect::<Vec<u8>>();
            let url_to_shorten = std::str::from_utf8(&c).unwrap();
            let shortened_url = {
                let mut urls = SHORT_URLS.write().unwrap();
                urls.entry(url_to_shorten.to_owned())
                    .or_insert(shorten_url(url_to_shorten))
                    .clone()
            };
            shortened_url
        });
    Ok(Response::new(Body::wrap_stream(reply)))
}
