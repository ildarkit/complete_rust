use actix::MailboxError;
use anyhow::{Error, Context};
use actix_web::{web, error::ResponseError, HttpRequest, HttpResponse};
use crate::{State, state::{AddLink, GetLinks, RmLink}};

type Result<T> = std::result::Result<T, LinkSnapError>;

#[derive(thiserror::Error, Debug)]
pub enum LinkSnapError {
    #[error(transparent)]
    SendMessage(#[from] MailboxError),
    #[error(transparent)]
    Anyhow(#[from] Error),
}

impl ResponseError for LinkSnapError {}

pub async fn index(_req: HttpRequest) -> &'static str {
    "Welcome to Linksnap API server"
}

pub async fn add_link(link: web::Json<AddLink>, state: web::Data<State>)
    -> Result<HttpResponse> {
    let state = state.get();
    state.send(link.0).await?
        .context("Failed to add link")?;
    Ok(HttpResponse::Ok().finish())
}

pub async fn links(state: web::Data<State>) -> Result<HttpResponse> {
    let state = &state.get();
    let res = state.send(GetLinks).await?
        .context("Failed to retrieve links")?;
    Ok(HttpResponse::Ok().body(res))
}

pub async fn rm_link(state: web::Data<State>, params: web::Query<RmLink>)
    -> Result<HttpResponse>
{
    let state = &state.get();
    let linkid = state.send(RmLink { id: params.id }).await?;
    Ok(HttpResponse::Ok().body(linkid))
}
