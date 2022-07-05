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
# linux-arm
wget https://github.com/limitcool/biliup-http/releases/download/v0.1.2/biliup-http-v0.1.2-aarch64-linux.tar.xz
xz -d biliup-http-v0.1.2-aarch64-linux.tar.xz
tar -xvf biliup-http-v0.1.2-aarch64-linux.tar
cd biliup-http-v0.1.2-aarch64-linux
./biliup-http
```
## 如需指定Http端口,请在目录下新建config.yaml文件,并在其中添加如下内容:
```yaml
port: 6000
```

### 上传视频接口请求示例

> ##### http://127.0.0.1:3000/upload

**请求方式:`[POST]`**

##### 请求体: Body raw(json) 

##### Content-Type:application/json

| 参数名             | 类型   | 内容                                                         | 是否必填 |
| ------------------ | ------ | ------------------------------------------------------------ | -------- |
| copyright          | uint   | 是否转载, 1-自制 2-转载                                      | 是       |
| source             | String | 转载来源                                                     | 转载必填 |
| tid                | uint   | 投稿分区                                                     | 是       |
| cover              | String | 封面地址,如已有b站封面填写url即可,没有封面则填写下方的封面路径 | 否       |
| title              | String | 视频标题                                                     | 是       |
| desc_format_id     | uint   | 简介类型                                                     | 否       |
| desc               | String | 视频简介                                                     | 是       |
| dynamic            | String | 空间动态                                                     | 是       |
| tag                | String | 视频标签, 以,号隔开                                          | 是       |
| dtime              | uint   | 延时发布时间，距离提交大于4小时，格式为10位时间戳            | 否       |
| interactive        | uint   | 是否开启互动 默认为0                                         | 否       |
| dolby              | uint   | 是否开启杜比音效,0-关闭 1-开启 默认为0                       | 否       |
| up_selection_reply | bool   | 是否开启评论精选                                             | 是       |
| up_close_reply     | bool   | 是否关闭评论区                                               | 是       |
| up_close_danmu     | bool   | 是否关闭弹幕                                                 | 是       |
| video_path         | String | 上传视频路径                                                 | 是       |
| cover_path         | String | 视频封面路径                                                 | 否       |



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

### 上传任务状态查询

> #####  http://127.0.0.1:3000/state

**请求方式`[GET]`**

| 参数名  | 类型   | 内容   | 是否必填 |
| ------- | ------ | ------ | -------- |
| task_id | String | 任务id | 是       |

##### 请求示例

```bash
# curl 示例
curl http://127.0.0.1:3000/state?task_id=cc36e47c-af5d-40e1-b149-d304a1c55d90
```

### 任务状态查询返回状态类型解答

| 错误类型              | 错误原因           |
| --------------------- | ------------------ |
| cookies.json不存在    | cookies.json不存在 |
| 登录失败,请检查cookie | cookie过期或失效   |
| 读取封面错误          | 上传封面路径不正确 |
| 视频文件不存在        |       上传视频路径不正确          |
| 上传失败 | 视频上传中出现问题 |

###### 正确状态

| 类型   | 详细               |
| ------ | ------------------ |
| 进行中 | 视频正在进行上传   |
| 已完成 | 视频任务已上传完成 |

