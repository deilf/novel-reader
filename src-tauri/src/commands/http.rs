//! 通用 HTTP 请求命令模块
//!
//! 为书源规则引擎提供底层网络请求能力，返回已解码为 UTF-8 的正文。

use crate::models::{HttpRequest, HttpResponse};
use log::{error, info};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::time::Duration;

/// 根据响应头 charset 与内容字节检测编码并解码为 UTF-8
fn decode_body(bytes: &[u8], content_type: &str) -> String {
    // 1. 从 Content-Type 中提取 charset
    if let Some(pos) = content_type.to_lowercase().find("charset=") {
        let charset = content_type[pos + "charset=".len()..]
            .trim()
            .trim_matches('"')
            .split([';', ',', ' '])
            .next()
            .unwrap_or("")
            .to_string();
        if !charset.is_empty() {
            if let Some(enc) = encoding_rs::Encoding::for_label(charset.as_bytes()) {
                let (decoded, _, had_errors) = enc.decode(bytes);
                if !had_errors {
                    return decoded.into_owned();
                }
            }
        }
    }

    // 2. BOM 检测
    if let Some((enc, _)) = encoding_rs::Encoding::for_bom(bytes) {
        let (decoded, _, _) = enc.decode(bytes);
        return decoded.into_owned();
    }

    // 3. UTF-8 尝试
    if std::str::from_utf8(bytes).is_ok() {
        return String::from_utf8_lossy(bytes).into_owned();
    }

    // 4. GBK 兜底（中文网站常用）
    let (decoded, _, _) = encoding_rs::GBK.decode(bytes);
    decoded.into_owned()
}

/// 执行一次通用 HTTP 请求
///
/// # 参数
/// * `request` - 请求参数（url、method、headers、body、超时）
///
/// # 返回
/// 状态码、最终 URL（跟随重定向后）、Content-Type、解码后的正文
#[tauri::command]
pub async fn http_fetch(request: HttpRequest) -> Result<HttpResponse, String> {
    let url = request.url.trim();
    if url.is_empty() {
        return Err("URL不能为空".to_string());
    }
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(format!("不支持的URL协议: {}", url));
    }

    let method = request.method.unwrap_or_else(|| "GET".to_string()).to_uppercase();
    let timeout_ms = request.timeout_ms.unwrap_or(15_000);
    let timeout_ms = timeout_ms.clamp(1_000, 60_000);

    info!("HTTP {} {}", method, url);

    // 构建客户端：默认 UA、超时、自动跟随重定向
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Linux; Android 14) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0 Mobile Safari/537.36")
        .timeout(Duration::from_millis(timeout_ms))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()
        .map_err(|e| format!("创建HTTP客户端失败: {}", e))?;

    // 组装请求
    let mut req = client.request(
        reqwest::Method::from_bytes(method.as_bytes())
            .map_err(|e| format!("不支持的请求方法: {}", e))?,
        url,
    );

    // 默认请求头 + 书源自定义头
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("Accept".to_string(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".to_string());
    headers.insert("Accept-Language".to_string(), "zh-CN,zh;q=0.9,en;q=0.8".to_string());

    if let Some(custom) = request.headers {
        for (k, v) in custom {
            headers.insert(k, v);
        }
    }

    let mut header_map = HeaderMap::new();
    for (k, v) in headers.iter() {
        if let (Ok(name), Ok(value)) = (HeaderName::from_bytes(k.as_bytes()), HeaderValue::from_str(v)) {
            header_map.insert(name, value);
        }
    }
    req = req.headers(header_map);

    if let Some(body) = request.body {
        if !body.is_empty() {
            req = req.body(body);
        }
    }

    // 发送请求
    let resp = match req.send().await {
        Ok(r) => r,
        Err(e) => {
            error!("请求失败: {} - {}", url, e);
            return Err(format!("请求失败: {}", e));
        }
    };

    let status = resp.status().as_u16();
    let final_url = resp.url().to_string();
    let content_type = resp
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let bytes = match resp.bytes().await {
        Ok(b) => b,
        Err(e) => return Err(format!("读取响应失败: {}", e)),
    };

    let body = decode_body(&bytes, &content_type);

    info!(
        "HTTP 完成: {} -> {} ({} 字节)",
        final_url,
        status,
        bytes.len()
    );

    Ok(HttpResponse {
        status,
        final_url,
        content_type,
        body,
    })
}
