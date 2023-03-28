/*文件系统*/
use actix_files::NamedFile;
use actix_web::http::header;
use actix_web::web::Path;
use actix_web::{get, HttpRequest, HttpResponse, Responder, Result};
use brave_config::interface::Interface;
use std::path::PathBuf;

#[get("/{name}/{filename:.*}")]
pub async fn file_load(path: Path<(String, String)>, req: HttpRequest) -> Result<impl Responder> {
    let (name, filename) = path.into_inner();

    /*文件路径先设置在当前目录public下*/
    let mut path_buf = PathBuf::new();
    path_buf.push("./page");
    path_buf.push(name.to_string());
    path_buf.push(filename.to_string());

    /*判断是否存在文件*/
    match NamedFile::open_async(&path_buf).await {
        Ok(content) => Ok(content.into_response(&req)),
        Err(_) => {
            /*如果没有文件存在就用户重定向*/
            let user_blog_home = Interface::redirect_user_blog_home(&name);
            Ok(HttpResponse::Found()
                .append_header((header::LOCATION, user_blog_home))
                .finish())
        }
    }
}