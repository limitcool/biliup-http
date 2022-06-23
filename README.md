# biliup-http

### 使用http请求调用[biliup-rs](https://github.com/ForgQi/biliup-rs)。

## Quick Start

```shell
# 使用biliup-rs获取cookie,将cookies.json移动至项目目录下
Cargo run
```

### 上传接口请求示例

##### 请求地址: [POST] 127.0.0.1:3000/upload

##### 请求体: Body raw(json)

```json
{
    "copyright":1,
    "source":"github.com",
    "tid":17,
    "cover":"",
    "title":"",
    "desc_format_id":0,
    "desc":"",
    "dynamic":"",
    "open_subtitle":true,
    "subtitle":{
        "open":0,
        "lan":""
    },
    "tag":"",
    "videos":[],
    "interactive":0,
    "dolby":0,
    "up_selection_reply":false,
    "up_close_reply":false,
    "up_close_danmu":false,
    "video_path":"<视频路径>",
    "cover_path":"<视频封面路径>"
}
```



### 上传接口返回示例
```json
{
    "task_id": "cc36e47c-af5d-40e1-b149-d304a1c55d90",
    // 返回task_id可查询上传状态
    "state": "success"
}
```

### 查询上传状态

##### 请求地址:  [GET] 127.0.0.1:3000/state

##### 请求参数: task_id

```bash
curl http://127.0.0.1:3000/state?task_id=cc36e47c-af5d-40e1-b149-d304a1c55d90
```

