//! 本地书籍命令模块（TXT / EPUB）
//!
//! 导入策略：TXT 与 EPUB 统一归一为 UTF-8 章节文本文件
//! `books/<id>/chapters.txt`，章节以分隔符 \u0001CHAPTER\u0001 分割，
//! 元数据存于 local_books.json（含章节标题与字符偏移）。

use crate::models::{LocalBook, LocalChapter};
use crate::utils::data_dir::{ensure_data_dir, get_data_dir};
use log::{error, info};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// 章节分隔符（不会出现在正常文本中）
pub const CHAPTER_SEP: &str = "\u{0001}CHAPTER\u{0001}";

fn get_local_books_path() -> PathBuf {
    get_data_dir().join("local_books.json")
}

fn get_books_dir() -> PathBuf {
    get_data_dir().join("books")
}

fn get_chapters_file(id: &str) -> PathBuf {
    get_books_dir().join(format!("{}.chapters.txt", id))
}

fn read_books() -> Result<Vec<LocalBook>, String> {
    let path = get_local_books_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let json = fs::read_to_string(&path).map_err(|e| format!("读取本地书籍失败: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("解析本地书籍失败: {}", e))
}

fn write_books(books: &[LocalBook]) -> Result<(), String> {
    ensure_data_dir()?;
    let json = serde_json::to_string_pretty(books).map_err(|e| format!("序列化失败: {}", e))?;
    fs::write(get_local_books_path(), json).map_err(|e| format!("写入失败: {}", e))
}

fn now_ts() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

// ---------------------------------------------------------------- 编码解码

fn decode_bytes(bytes: &[u8], content_type_hint: &str) -> String {
    if let Some((enc, _)) = encoding_rs::Encoding::for_bom(bytes) {
        let (s, _, _) = enc.decode(bytes);
        return s.into_owned();
    }
    if std::str::from_utf8(bytes).is_ok() {
        return String::from_utf8_lossy(bytes).into_owned();
    }
    // GBK 兜底（中文 TXT 常用）
    let (s, _, _) = encoding_rs::GBK.decode(bytes);
    s.into_owned()
}

// ---------------------------------------------------------------- TXT 解析

/// 常见章节标题正则
fn chapter_title_regex() -> regex::Regex {
    regex::Regex::new(
        r"(?m)^\s*(第\s*[0-9一二三四五六七八九十百千万零两]+\s*[章回卷节部集篇][^\n]{0,40}|#{1,4}\s+.+)$",
    )
    .unwrap()
}

/// 解析 TXT：按章节正则切分，返回 (章节列表, 拼接后的 UTF-8 文本)
fn parse_txt(text: &str) -> (Vec<LocalChapter>, String) {
    let re = chapter_title_regex();
    let mut chapters: Vec<LocalChapter> = Vec::new();
    let mut parts: Vec<&str> = Vec::new();
    let mut last_match_end = 0usize;
    let mut chapter_index = 0i64;

    for m in re.find_iter(text) {
        // 记录上一段（从上次标题结束到本次标题开始）
        if last_match_end < m.start() {
            parts.push(&text[last_match_end..m.start()]);
        }
        let title = m.as_str().trim().to_string();
        chapters.push(LocalChapter {
            title,
            index: chapter_index,
            offset: parts.len() as i64, // 记录 parts 位置
        });
        chapter_index += 1;
        last_match_end = m.end();
    }
    // 剩余正文
    if last_match_end < text.len() {
        parts.push(&text[last_match_end..]);
    }

    // 无标题匹配：整本作为一章
    if chapters.is_empty() {
        chapters.push(LocalChapter {
            title: String::from("正文"),
            index: 0,
            offset: 0,
        });
        parts = vec![text];
    }

    let joined = parts.join(CHAPTER_SEP);
    (chapters, joined)
}

// ---------------------------------------------------------------- EPUB 解析

fn epub_text_from_html(html: &str) -> String {
    // 去 script/style
    let mut out = String::with_capacity(html.len());
    let re_script = regex::Regex::new(r"(?is)<script[^>]*>.*?</script>|<style[^>]*>.*?</style>").unwrap();
    let cleaned = re_script.replace_all(html, "").to_string();
    // 块级标签换行
    let re_block = regex::Regex::new(r"(?i)</(p|div|h[1-6]|br|li|tr|section|article)\s*>").unwrap();
    let with_nl = re_block.replace_all(&cleaned, "\n").to_string();
    // 去剩余标签 + 解码实体
    let re_tag = regex::Regex::new(r"(?s)<[^>]*>").unwrap();
    let text = re_tag.replace_all(&with_nl, "").to_string();
    let text = text
        .replace("&nbsp;", " ")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#39;", "'");
    let re_entity = regex::Regex::new(r"&#(\d+);").unwrap();
    let text = re_entity
        .replace_all(&text, |caps: &regex::Captures| {
            if let Ok(n) = caps[1].parse::<u32>() {
                char::from_u32(n).map(|c| c.to_string()).unwrap_or_default()
            } else {
                String::new()
            }
        })
        .to_string();
    // 压缩空行
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

/// 解析 EPUB 文件，返回 (书名, 作者, 章节列表, 章节正文数组)
fn parse_epub(path: &Path) -> Result<(String, Option<String>, Vec<LocalChapter>, Vec<String>), String> {
    let file = fs::File::open(path).map_err(|e| format!("打开 EPUB 失败: {}", e))?;
    let mut zip = zip::ZipArchive::new(file).map_err(|e| format!("EPUB 不是有效 ZIP: {}", e))?;

    // 1. container.xml → OPF 路径
    let container = zip
        .by_name("META-INF/container.xml")
        .map_err(|_| "EPUB 缺少 META-INF/container.xml".to_string())?;
    let mut container_buf = String::new();
    {
        use std::io::Read;
        let mut r = container;
        r.read_to_string(&mut container_buf)
            .map_err(|e| format!("读取 container.xml 失败: {}", e))?;
    }
    let container_doc = roxmltree::Document::parse(&container_buf)
        .map_err(|e| format!("解析 container.xml 失败: {}", e))?;
    let opf_path = container_doc
        .descendants()
        .find(|n| n.has_tag_name("rootfile"))
        .and_then(|n| n.attribute("full-path"))
        .ok_or_else(|| "container.xml 缺少 rootfile".to_string())?
        .to_string();

    // 2. OPF：书名/作者 + manifest + spine
    let opf = zip
        .by_name(&opf_path)
        .map_err(|_| "EPUB 缺少 OPF 文件".to_string())?;
    let mut opf_buf = String::new();
    {
        use std::io::Read;
        let mut r = opf;
        r.read_to_string(&mut opf_buf)
            .map_err(|e| format!("读取 OPF 失败: {}", e))?;
    }
    let opf_doc = roxmltree::Document::parse(&opf_buf)
        .map_err(|e| format!("解析 OPF 失败: {}", e))?;
    let opf_dir = Path::new(&opf_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let title = opf_doc
        .descendants()
        .filter(|n| n.tag_name().name() == "title")
        .filter_map(|n| n.text())
        .next()
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            path.file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "未知书名".to_string())
        });
    let author = opf_doc
        .descendants()
        .filter(|n| n.tag_name().name() == "creator")
        .filter_map(|n| n.text())
        .next()
        .map(|s| s.to_string());

    // manifest: id → href
    let mut manifest: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    for item in opf_doc.descendants().filter(|n| n.has_tag_name("item")) {
        if let (Some(id), Some(href)) = (item.attribute("id"), item.attribute("href")) {
            manifest.insert(id.to_string(), href.to_string());
        }
    }

    // spine: idref 顺序
    let mut spine_order: Vec<String> = Vec::new();
    for item in opf_doc.descendants().filter(|n| n.has_tag_name("itemref")) {
        if let Some(idref) = item.attribute("idref") {
            spine_order.push(idref.to_string());
        }
    }

    // 3. 按 spine 顺序读取章节
    let mut chapters: Vec<LocalChapter> = Vec::new();
    let mut contents: Vec<String> = Vec::new();
    for (i, idref) in spine_order.iter().enumerate() {
        let href = match manifest.get(idref) {
            Some(h) => h.clone(),
            None => continue,
        };
        // 相对 OPF 目录解析
        let full = if opf_dir.is_empty() {
            href.clone()
        } else {
            format!("{}/{}", opf_dir, href)
        };
        let item = match zip.by_name(&full) {
            Ok(i) => i,
            Err(_) => continue,
        };
        let mut buf = String::new();
        {
            use std::io::Read;
            let mut r = item;
            let mut bytes = Vec::new();
            if r.read_to_end(&mut bytes).is_err() {
                continue;
            }
            buf = decode_bytes(&bytes, "");
        }
        let text = epub_text_from_html(&buf);
        if text.trim().is_empty() {
            continue;
        }
        // 章节标题：取 xhtml 首个 h1/h2/title（如有），否则"第 N 章"
        let chapter_title = extract_epub_chapter_title(&buf, i + 1);
        chapters.push(LocalChapter {
            title: chapter_title,
            index: i as i64,
            offset: contents.len() as i64,
        });
        contents.push(text);
    }

    Ok((title, author, chapters, contents))
}

/// 从 xhtml 提取章节标题
fn extract_epub_chapter_title(html: &str, default_index: usize) -> String {
    let re = regex::Regex::new(r"(?is)<h[1-3][^>]*>(.*?)</h[1-3]>|<title[^>]*>(.*?)</title>")
        .unwrap();
    if let Some(caps) = re.captures(html) {
        let raw = caps.get(1).or_else(|| caps.get(2)).map(|m| m.as_str()).unwrap_or("");
        let text = epub_text_from_html(raw);
        if !text.is_empty() {
            return text;
        }
    }
    format!("第 {} 章", default_index)
}

// ---------------------------------------------------------------- 命令

/// 导入本地书籍（TXT / EPUB）
#[tauri::command]
pub fn import_local_book(path: String) -> Result<LocalBook, String> {
    let src = PathBuf::from(&path);
    if !src.exists() {
        return Err("文件不存在".to_string());
    }
    let ext = src
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    ensure_data_dir()?;
    let books_dir = get_books_dir();
    fs::create_dir_all(&books_dir).map_err(|e| format!("创建书籍目录失败: {}", e))?;

    // 唯一 ID
    let id = format!(
        "{}_{}",
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or_default(),
        src.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "book".to_string())
    );
    // ID 只保留安全字符
    let id: String = id
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-')
        .collect();

    let (title, author, chapters, joined): (String, Option<String>, Vec<LocalChapter>, String) =
        match ext.as_str() {
            "txt" => {
                let bytes = fs::read(&src).map_err(|e| format!("读取 TXT 失败: {}", e))?;
                let text = decode_bytes(&bytes, "");
                let (ch, joined) = parse_txt(&text);
                let title = src
                    .file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "未知书名".to_string());
                (title, None, ch, joined)
            }
            "epub" => {
                let (t, a, ch, contents) = parse_epub(&src)?;
                let joined = contents.join(CHAPTER_SEP);
                (t, a, ch, joined)
            }
            other => return Err(format!("不支持的格式: {}", other)),
        };

    // 写章节文件
    fs::write(get_chapters_file(&id), &joined).map_err(|e| format!("写入章节文件失败: {}", e))?;

    let book = LocalBook {
        id: id.clone(),
        title,
        author,
        format: ext,
        path: src.to_string_lossy().to_string(),
        chapter_count: chapters.len() as i64,
        added_at: now_ts(),
        cover: None,
    };

    let mut books = read_books()?;
    // 同路径去重
    if let Some(old) = books.iter_mut().find(|b| b.path == book.path) {
        let _ = fs::remove_file(get_chapters_file(&old.id));
        *old = book.clone();
    } else {
        books.push(book.clone());
    }
    write_books(&books)?;
    info!(
        "本地书导入成功: {} ({} 章, {})",
        book.title, book.chapter_count, book.format
    );
    Ok(book)
}

/// 获取本地书籍列表
#[tauri::command]
pub fn list_local_books() -> Result<Vec<LocalBook>, String> {
    Ok(read_books()?)
}

/// 删除本地书籍
#[tauri::command]
pub fn remove_local_book(id: String) -> Result<(), String> {
    let mut books = read_books()?;
    let before = books.len();
    books.retain(|b| b.id != id);
    if books.len() == before {
        return Err("书籍不存在".to_string());
    }
    let _ = fs::remove_file(get_chapters_file(&id));
    write_books(&books)?;
    Ok(())
}

/// 获取本地书籍章节列表
#[tauri::command]
pub fn get_local_chapters(book_id: String) -> Result<Vec<LocalChapter>, String> {
    let books = read_books()?;
    let book = books
        .iter()
        .find(|b| b.id == book_id)
        .ok_or_else(|| "书籍不存在".to_string())?;
    // 从章节文件重新切分获取标题（或直接解析 JSON 中的章节表）
    // 简化：章节信息存于 chapters.txt 结构；此处重新切分
    let path = get_chapters_file(&book_id);
    if !path.exists() {
        return Err("章节文件缺失".to_string());
    }
    let text = fs::read_to_string(&path).map_err(|e| format!("读取章节文件失败: {}", e))?;
    let parts: Vec<&str> = text.split(CHAPTER_SEP).collect();

    // 标题：TXT 无独立标题表，章节首行作为标题（若有）
    let mut chapters: Vec<LocalChapter> = Vec::new();
    for (i, part) in parts.iter().enumerate() {
        let title = match part.lines().next() {
            Some(l) if !l.trim().is_empty() && l.trim().chars().count() < 60 => l.trim().to_string(),
            _ => format!("第 {} 章", i + 1),
        };
        chapters.push(LocalChapter {
            title,
            index: i as i64,
            offset: i as i64,
        });
    }
    let _ = book;
    Ok(chapters)
}

/// 获取本地书籍章节正文
#[tauri::command]
pub fn get_local_chapter_content(book_id: String, index: i64) -> Result<String, String> {
    let path = get_chapters_file(&book_id);
    if !path.exists() {
        return Err("章节文件缺失".to_string());
    }
    let text = fs::read_to_string(&path).map_err(|e| format!("读取章节文件失败: {}", e))?;
    let parts: Vec<&str> = text.split(CHAPTER_SEP).collect();
    parts
        .get(index as usize)
        .map(|s| s.to_string())
        .ok_or_else(|| "章节不存在".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn txt_chapter_split() {
        let text = "第一章 开始\n正文内容一\n\n第二章 发展\n正文内容二\n第三章 结局\n正文内容三";
        let (chapters, joined) = parse_txt(text);
        assert_eq!(chapters.len(), 3, "应切出 3 章");
        assert_eq!(chapters[0].title, "第一章 开始");
        assert_eq!(chapters[1].title, "第二章 发展");
        assert_eq!(chapters[2].title, "第三章 结局");
        let parts: Vec<&str> = joined.split(CHAPTER_SEP).collect();
        assert_eq!(parts.len(), 3, "章节文件应有 3 段");
        assert!(parts[0].contains("正文内容一"));
        assert!(parts[1].contains("正文内容二"));
        assert!(parts[2].contains("正文内容三"));
    }

    #[test]
    fn txt_no_title_falls_back_single() {
        let text = "没有任何章节标题的一段长文本";
        let (chapters, joined) = parse_txt(text);
        assert_eq!(chapters.len(), 1);
        assert_eq!(chapters[0].title, "正文");
        assert_eq!(joined, text);
    }

    #[test]
    fn txt_chinese_numeral_chapters() {
        let text = "第一章 开篇\n内容甲\n第十二章 转折\n内容乙\n第一百零八章 收尾\n内容丙";
        let (chapters, _) = parse_txt(text);
        assert_eq!(chapters.len(), 3);
        assert_eq!(chapters[1].title, "第十二章 转折");
        assert_eq!(chapters[2].title, "第一百零八章 收尾");
    }

    #[test]
    fn decode_utf8_bom() {
        let bytes = [0xEF, 0xBB, 0xBF, b'h', b'i'];
        let s = decode_bytes(&bytes, "");
        assert_eq!(s, "hi");
    }

    #[test]
    fn decode_gbk_fallback() {
        // "中文" 的 GBK 编码
        let gbk = encoding_rs::GBK.encode("中文测试").0.to_vec();
        let s = decode_bytes(&gbk, "");
        assert_eq!(s, "中文测试");
    }

    #[test]
    fn epub_html_cleanup() {
        let html = r#"<html><head><style>.x{}</style></head><body>
            <h1>第一章 测试</h1>
            <p>第一段 &amp; 内容</p>
            <script>alert(1)</script>
            <p>第二段&#32;内容</p>
        </body></html>"#;
        let text = epub_text_from_html(html);
        assert!(text.contains("第一章 测试"), "应保留 h1 文本");
        assert!(text.contains("第一段 & 内容"), "应解码 &amp;");
        assert!(!text.contains("alert"), "应删除 script");
        assert!(!text.contains("<style"), "应删除 style");
        assert!(!text.contains("<h1>"), "应删除标签");
    }

    #[test]
    fn epub_chapter_title_extract() {
        let html = r#"<html><body><h2>测试章节</h2><p>内容</p></body></html>"#;
        assert_eq!(extract_epub_chapter_title(html, 1), "测试章节");
        let html2 = r#"<html><body><p>无标题内容</p></body></html>"#;
        assert_eq!(extract_epub_chapter_title(html2, 5), "第 5 章");
    }
}

#[cfg(test)]
mod epub_integration {
    use super::*;
    use std::io::Write;

    fn make_epub(path: &Path) {
        let file = fs::File::create(path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let opts = zip::write::SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

        zip.start_file("mimetype", opts).unwrap();
        zip.write_all(b"application/epub+zip").unwrap();

        zip.start_file("META-INF/container.xml", opts).unwrap();
        zip.write_all(
            r#"<?xml version="1.0"?><container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container"><rootfiles><rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/></rootfiles></container>"#.as_bytes(),
        ).unwrap();

        zip.start_file("OEBPS/content.opf", opts).unwrap();
        zip.write_all(
            r#"<?xml version="1.0"?><package xmlns="http://www.idpf.org/2007/opf" version="3.0"><metadata><dc:title xmlns:dc="http://purl.org/dc/elements/1.1/">测试之书</dc:title><dc:creator xmlns:dc="http://purl.org/dc/elements/1.1/">测试作者</dc:creator></metadata><manifest><item id="c1" href="c1.xhtml" media-type="application/xhtml+xml"/><item id="c2" href="c2.xhtml" media-type="application/xhtml+xml"/></manifest><spine><itemref idref="c1"/><itemref idref="c2"/></spine></package>"#.as_bytes(),
        ).unwrap();

        zip.start_file("OEBPS/c1.xhtml", opts).unwrap();
        zip.write_all(
            r#"<html><head><title>第一章 缘起</title></head><body><h1>第一章 缘起</h1><p>第一段正文&amp;内容。</p><p>第二段正文。</p></body></html>"#.as_bytes(),
        ).unwrap();

        zip.start_file("OEBPS/c2.xhtml", opts).unwrap();
        zip.write_all(
            r#"<html><body><h2>第二章 发展</h2><p>第三段正文。</p></body></html>"#.as_bytes(),
        ).unwrap();

        zip.finish().unwrap();
    }

    #[test]
    fn parse_real_epub() {
        let dir = std::env::temp_dir().join("novel_reader_epub_test");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test_book.epub");
        make_epub(&path);

        let (title, author, chapters, contents) = parse_epub(&path).unwrap();
        assert_eq!(title, "测试之书");
        assert_eq!(author.as_deref(), Some("测试作者"));
        assert_eq!(chapters.len(), 2);
        assert_eq!(chapters[0].title, "第一章 缘起");
        assert_eq!(chapters[1].title, "第二章 发展");
        assert!(contents[0].contains("第一段正文&内容。"), "实体解码错误: {}", contents[0]);
        assert!(contents[0].contains("第二段正文"));
        assert!(contents[1].contains("第三段正文"));

        let _ = fs::remove_dir_all(&dir);
    }
}
