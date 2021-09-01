本代码来自 b 站 up 主 [原子之音](https://space.bilibili.com/437860379) 的 [用rust打造实时天气命令行工具](https://www.bilibili.com/video/BV1eL411b7EL)。

### How to use

运行 `cargo run <city>`输出指定城市的天晴情况，其中 `<city>` 支持城市名以及邮编。

当输入的 city 无法查询时输出 API 返回的错误信息。
