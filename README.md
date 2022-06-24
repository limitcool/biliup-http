# biliup-http

### 使用http请求调用[biliup-rs](https://github.com/ForgQi/biliup-rs)。
### biliup 支持自动录制各大直播平台实时流，上传到bilibili
## Quick Start

```shell
# 先使用biliup-rs获取cookie,将cookies.json移动至项目目录下
# 前往Release下载对应的版本进行解压
# windows
.\biliup-http.exe
curl http://127.0.0.1:3000/  # hello biliup-http!
# 如需在arm环境使用,请暂时先使用安装rust环境进行手动编译。
git clone https://github.com/limitcool/biliup-http.git
cargo build 
```

### 上传接口请求示例

##### 请求地址: [POST] 127.0.0.1:3000/upload

##### 请求体: Body raw(json)

```json
{
    "copyright":1,
    "source":"github.com/limitcool/biliup-http",
    "tid":17, // 分区id
    "cover":"", // 封面地址,如已有b站封面填写url即可,没有封面则填写下方的封面路径
    "title":"", // 标题
    "desc_format_id":0,
    "desc":"", // 描述
    "dynamic":"", // 动态
    "open_subtitle":true, // 是否开启字幕
    "tag":"", // 标签以逗号隔开
    "interactive":0, 
    "dolby":0,
    "up_selection_reply":false,
    "up_close_reply":false,
    "up_close_danmu":false,
    "video_path":"<视频路径>",
    "cover_path":"<视频封面路径>"
}

// 示例
{
    "copyright":1,
    "source":"github.com/limitcool/biliup-http",
    "tid":17, 
    "cover":"", 
    "title":"test",
    "desc_format_id":0,
    "desc":"desc",
    "dynamic":"test",
    "open_subtitle":true,
    "tag":"测试,",
    "interactive":0,
    "dolby":0,
    "up_selection_reply":false,
    "up_close_reply":false,
    "up_close_danmu":false,
    // Linux和Windows路径通用分隔符为"/",如只在win使用可以将Windows路径分隔符修改为"\\"
    "video_path":"C:/Users/Andorid/Videos/test.mp4",
    "cover_path":"C:/Users/Andorid/Desktop/test.jpg"
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

