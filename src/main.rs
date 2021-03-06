use anyhow::Error;
use axum::{
    extract::{Extension, Query},
    http::status::StatusCode,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use biliup::{
    client::Client,
    line::Probe,
    video::{BiliBili, Studio, Subtitle, Video},
    {line, VideoFile},
};
use bytes::{Buf, Bytes};
use config::Config;
use futures::{Stream, StreamExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Body;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Poll;
use std::time::Instant;
use uuid::Uuid;
mod config;

#[tokio::main]
async fn main() {
    let cfg = load_config(Path::new("config.yaml")).unwrap_or(config::new());
    tracing_subscriber::fmt::init();
    let db: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/", get(root))
        .layer(Extension(db.clone()))
        .route("/upload", post(uploadr))
        .layer(Extension(db.clone()))
        .route("/state", get(state))
        .layer(Extension(db.clone()));
    let addr = SocketAddr::from(([127, 0, 0, 1], cfg.port));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    return "hello biliup-http!";
}

async fn state(
    Extension(db): Extension<Arc<Mutex<HashMap<String, String>>>>,
    Query(params): Query<Params>,
) -> String {
    // println!("taskid_{:?}",params.task_id);
    match Extension(db).lock().unwrap().clone().get(&params.task_id) {
        Some(v) => return v.to_string(),
        None => return "任务不存在!".to_string(),
    }
}

async fn uploadr(
    Extension(db): Extension<Arc<Mutex<HashMap<String, String>>>>,
    Json(payload): Json<UploadRequest>,
) -> impl IntoResponse {
    // println!("{:?}", payload);
    // println!("{:?}", payload.tid);
    if payload.source.is_empty() {
        println!("source is required");
        return (
            StatusCode::BAD_REQUEST,
            "source is required".into_response(),
        );
    };
    if payload.title.is_empty() {
        println!("title is required");
        return (StatusCode::BAD_REQUEST, "title is required".into_response());
    };
    if payload.desc.is_empty() {
        println!("desc is required");
        return (StatusCode::BAD_REQUEST, "desc is required".into_response());
    };
    if payload.tag.is_empty() {
        println!("tags is required");
        return (StatusCode::BAD_REQUEST, "tags is required".into_response());
    };
    let task_id = Uuid::new_v4().to_string();
    let rid = task_id.clone();
    tokio::spawn(async move {
        match upload_video(task_id, &payload, Extension(db)).await {
            Ok(v) => println!("{:?}", v),
            Err(e) => eprintln!("{:?}", e),
        };
    });
    let r = UploadResponse {
        task_id: rid,
        state: "success".to_string(),
    };

    (StatusCode::OK, Json(r).into_response())
}

#[derive(Serialize)]
struct UploadResponse {
    task_id: String,
    state: String,
}

#[allow(dead_code)]
fn make_studio(req: &UploadRequest) -> Studio {
    Studio {
        copyright: req.copyright,
        source: req.source.clone(),
        tid: req.tid,
        cover: req.cover.clone(),
        title: req.title.clone(),
        desc_format_id: 0,
        desc: req.desc.clone(),
        dynamic: req.dynamic.clone(),
        subtitle: Subtitle::default(),
        tag: req.tag.clone(),
        videos: Vec::new(),
        dtime: req.dtime,
        open_subtitle: req.open_subtitle,
        interactive: req.interactive,
        mission_id: req.mission_id,
        dolby: req.dolby,
        no_reprint: req.no_reprint,
        aid: req.aid,
        up_selection_reply: req.up_selection_reply,
        up_close_reply: req.up_close_reply,
        up_close_danmu: req.up_close_danmu,
        open_elec: req.open_elec,
    }
}
#[allow(dead_code)]
#[derive(Deserialize)]
struct UploadRequest {
    copyright: u8,
    source: String,
    tid: u16,
    cover: String,
    title: String,
    desc_format_id: u32,
    desc: String,
    dynamic: String,
    // subtitle: Subtitle,
    tag: String,
    // videos: Vec<Video>,
    dtime: Option<u32>,
    open_subtitle: bool,
    interactive: u8,
    mission_id: Option<u32>,
    dolby: u8,
    no_reprint: Option<u8>,
    aid: Option<u64>,
    up_selection_reply: bool,
    up_close_reply: bool,
    up_close_danmu: bool,
    open_elec: Option<u8>,
    video_path: String,
    cover_path: String,
}

pub async fn upload(
    video_path: &[PathBuf],
    client: &Client,
    line: Option<UploadLine>,
    limit: usize,
) -> Result<Vec<Video>, Error> {
    let mut videos = Vec::new();
    let line = match line {
        Some(UploadLine::Kodo) => line::kodo(),
        Some(UploadLine::Bda2) => line::bda2(),
        Some(UploadLine::Ws) => line::ws(),
        Some(UploadLine::Qn) => line::qn(),
        Some(UploadLine::Cos) => line::cos(),
        Some(UploadLine::CosInternal) => line::cos_internal(),
        None => Probe::probe().await.unwrap_or_default(),
    };
    // let line = line::kodo();
    for video_path in video_path {
        println!("{line:?}");
        let video_file = VideoFile::new(video_path)?;
        let total_size = video_file.total_size;
        let file_name = video_file.file_name.clone();
        let uploader = line.to_uploader(video_file);
        //Progress bar
        let pb = ProgressBar::new(total_size);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?);
        // pb.enable_steady_tick(Duration::from_secs(1));
        // pb.tick()

        let instant = Instant::now();

        let video = uploader
            .upload(client, limit, |vs| {
                vs.map(|chunk| {
                    let pb = pb.clone();
                    let (chunk, len) = chunk?;

                    Ok((Progressbar::new(chunk, pb), len))
                })
            })
            .await?;
        pb.finish_and_clear();
        let t = instant.elapsed().as_millis();
        println!(
            "Upload completed: {file_name} => cost {:.2}s, {:.2} MB/s.",
            t as f64 / 1000.,
            total_size as f64 / 1000. / t as f64
        );
        videos.push(video);
    }
    Ok(videos)
}

pub enum UploadLine {
    Bda2,
    Ws,
    Qn,
    Kodo,
    Cos,
    CosInternal,
}

#[derive(Clone)]
struct Progressbar {
    bytes: Bytes,
    pb: ProgressBar,
}

impl Progressbar {
    pub fn new(bytes: Bytes, pb: ProgressBar) -> Self {
        Self { bytes, pb }
    }

    pub fn progress(&mut self) -> Result<Option<Bytes>, Error> {
        let pb = &self.pb;

        let content_bytes = &mut self.bytes;

        let n = content_bytes.remaining();

        let pc = 4096;
        if n == 0 {
            Ok(None)
        } else if n < pc {
            pb.inc(n as u64);
            Ok(Some(content_bytes.copy_to_bytes(n)))
        } else {
            pb.inc(pc as u64);

            Ok(Some(content_bytes.copy_to_bytes(pc)))
        }
    }
}

impl Stream for Progressbar {
    type Item = Result<Bytes, Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.progress()? {
            None => Poll::Ready(None),
            Some(s) => Poll::Ready(Some(Ok(s))),
        }
    }
}

impl From<Progressbar> for Body {
    fn from(async_stream: Progressbar) -> Self {
        Body::wrap_stream(async_stream)
    }
}

async fn upload_video(
    uuid: String,
    u: &UploadRequest,
    db: Extension<Arc<Mutex<HashMap<String, String>>>>,
) -> Result<String, Error> {
    let mut s = make_studio(u);
    let client = Client::default();
    let login_info = {
        let cookies_file = std::fs::File::options()
            .read(true)
            .write(true)
            .open(Path::new("cookies.json"));
        client.login_by_cookies(cookies_file?).await?
    };
    // 上传封面
    if !s.cover.starts_with("http") {
        let bilibili = BiliBili::new(&login_info, &client);
        match &std::fs::read(Path::new(&u.cover_path.clone())) {
            Ok(_cover) => {
                let cover_url = bilibili
                    .cover_up(&std::fs::read(Path::new(&u.cover_path.clone())).unwrap())
                    .await;
                s.cover = cover_url.unwrap();
            }
            Err(_) => {
                db.lock().unwrap().insert(uuid, "读取封面错误".to_string());
                return Err(anyhow::Error::msg("读取封面错误"));
            }
        };
    }
    let video_path = PathBuf::from(u.video_path.clone());
    let paths = vec![video_path];
    let _uid = uuid.clone();
    db.lock().unwrap().insert(uuid, "进行中".to_string());
    upload(&paths, &client, Some(UploadLine::Ws), 3).await?;
    s.submit(&login_info).await?;

    return Ok("已完成".to_string());
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct Params {
    task_id: String,
}

pub fn load_config(config: &Path) -> Result<Config, Error> {
    let file = std::fs::File::open(config)?;
    let config: Config = serde_yaml::from_reader(file)?;
    Ok(config)
}
