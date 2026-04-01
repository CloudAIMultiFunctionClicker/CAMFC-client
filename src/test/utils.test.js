/**
 * 工具函数测试
 * 测试 fileIcon.js 中的工具函数
 */
import { describe, it, expect } from "vitest"
import { getFileType, getFileIcon, getFileTypeName, getExtension } from "../utils/fileIcon.js"

describe("getExtension 函数测试", () => {
  it("应该正确提取小写扩展名", () => {
    expect(getExtension("file.txt")).toBe("txt")
    expect(getExtension("photo.jpg")).toBe("jpg")
  })

  it("应该将大写扩展名转为小写", () => {
    expect(getExtension("file.TXT")).toBe("txt")
    expect(getExtension("image.PNG")).toBe("png")
    expect(getExtension("document.PDF")).toBe("pdf")
  })

  it("应该处理混合大小写", () => {
    expect(getExtension("file.TxT")).toBe("txt")
    expect(getExtension("image.PnG")).toBe("png")
  })

  it("应该处理没有扩展名的文件", () => {
    expect(getExtension("README")).toBe("")
    expect(getExtension("Makefile")).toBe("")
  })

  it("应该处理多个点的情况", () => {
    expect(getExtension("file.tar.gz")).toBe("gz")
    expect(getExtension("archive.tar.bz2")).toBe("bz2")
  })

  it("应该处理空文件名", () => {
    expect(getExtension("")).toBe("")
  })

  it("应该处理 null 和 undefined", () => {
    expect(getExtension(null)).toBe("")
    expect(getExtension(undefined)).toBe("")
  })

  it("应该处理隐藏文件", () => {
    expect(getExtension(".gitignore")).toBe("gitignore")
    expect(getExtension(".bashrc")).toBe("bashrc")
  })
})

describe("getFileType 函数测试", () => {
  it("应该正确识别图片文件", () => {
    expect(getFileType("photo.jpg")).toBe("image")
    expect(getFileType("image.jpeg")).toBe("image")
    expect(getFileType("icon.png")).toBe("image")
    expect(getFileType("animation.gif")).toBe("image")
    expect(getFileType("bitmap.bmp")).toBe("image")
    expect(getFileType("graphic.webp")).toBe("image")
    expect(getFileType("vector.svg")).toBe("image")
  })

  it("应该正确识别视频文件", () => {
    expect(getFileType("movie.mp4")).toBe("video")
    expect(getFileType("clip.avi")).toBe("video")
    expect(getFileType("recording.mkv")).toBe("video")
    expect(getFileType("quicktime.mov")).toBe("video")
    expect(getFileType("stream.webm")).toBe("video")
  })

  it("应该正确识别音频文件", () => {
    expect(getFileType("song.mp3")).toBe("audio")
    expect(getFileType("sound.wav")).toBe("audio")
    expect(getFileType("music.flac")).toBe("audio")
    expect(getFileType("track.aac")).toBe("audio")
    expect(getFileType("podcast.ogg")).toBe("audio")
  })

  it("应该正确识别文档文件", () => {
    expect(getFileType("report.pdf")).toBe("document")
    expect(getFileType("letter.doc")).toBe("document")
    expect(getFileType("resume.docx")).toBe("document")
    expect(getFileType("data.xls")).toBe("document")
    expect(getFileType("spreadsheet.xlsx")).toBe("document")
    expect(getFileType("presentation.ppt")).toBe("document")
    expect(getFileType("slides.pptx")).toBe("document")
    expect(getFileType("notes.txt")).toBe("document")
  })

  it("应该正确识别压缩包文件", () => {
    expect(getFileType("archive.zip")).toBe("archive")
    expect(getFileType("compressed.rar")).toBe("archive")
    expect(getFileType("backup.7z")).toBe("archive")
    expect(getFileType("bundle.tar")).toBe("archive")
    expect(getFileType("compressed.gz")).toBe("archive")
  })

  it("应该正确识别代码文件", () => {
    expect(getFileType("script.js")).toBe("code")
    expect(getFileType("app.ts")).toBe("code")
    expect(getFileType("page.html")).toBe("code")
    expect(getFileType("style.css")).toBe("code")
    expect(getFileType("data.json")).toBe("code")
    expect(getFileType("config.xml")).toBe("code")
    expect(getFileType("program.py")).toBe("code")
    expect(getFileType("Main.java")).toBe("code")
  })

  it("应该返回 other 对于未知类型", () => {
    expect(getFileType("file.xyz")).toBe("other")
    expect(getFileType("unknown.abc")).toBe("other")
    expect(getFileType("noext")).toBe("other")
  })

  it("应该处理大小写不敏感", () => {
    expect(getFileType("FILE.JPG")).toBe("image")
    expect(getFileType("VIDEO.MP4")).toBe("video")
    expect(getFileType("SONG.MP3")).toBe("audio")
  })
})

describe("getFileIcon 函数测试", () => {
  it("应该返回图片文件的图标", () => {
    expect(getFileIcon("photo.jpg")).toBe("ri-image-line")
    expect(getFileIcon("image.png")).toBe("ri-image-line")
  })

  it("应该返回视频文件的图标", () => {
    expect(getFileIcon("movie.mp4")).toBe("ri-video-line")
    expect(getFileIcon("clip.avi")).toBe("ri-video-line")
  })

  it("应该返回音频文件的图标", () => {
    expect(getFileIcon("song.mp3")).toBe("ri-music-line")
    expect(getFileIcon("track.flac")).toBe("ri-music-line")
  })

  it("应该返回文档文件的图标", () => {
    expect(getFileIcon("doc.pdf")).toBe("ri-file-text-line")
    expect(getFileIcon("report.docx")).toBe("ri-file-text-line")
    expect(getFileIcon("data.xlsx")).toBe("ri-file-text-line")
  })

  it("应该返回压缩包的图标", () => {
    expect(getFileIcon("archive.zip")).toBe("ri-file-zip-line")
    expect(getFileIcon("backup.rar")).toBe("ri-file-zip-line")
  })

  it("应该返回代码文件的图标", () => {
    expect(getFileIcon("script.js")).toBe("ri-code-line")
    expect(getFileIcon("app.ts")).toBe("ri-code-line")
    expect(getFileIcon("page.html")).toBe("ri-code-line")
  })

  it("应该返回默认图标对于未知类型", () => {
    expect(getFileIcon("file.xyz")).toBe("ri-file-line")
    expect(getFileIcon("unknown")).toBe("ri-file-line")
  })
})

describe("getFileTypeName 函数测试", () => {
  it("应该返回图片的中文名称", () => {
    expect(getFileTypeName("photo.jpg")).toBe("图片")
    expect(getFileTypeName("image.png")).toBe("图片")
  })

  it("应该返回视频的中文名称", () => {
    expect(getFileTypeName("movie.mp4")).toBe("视频")
    expect(getFileTypeName("clip.avi")).toBe("视频")
  })

  it("应该返回音频的中文名称", () => {
    expect(getFileTypeName("song.mp3")).toBe("音频")
    expect(getFileTypeName("track.flac")).toBe("音频")
  })

  it("应该返回文档的中文名称", () => {
    expect(getFileTypeName("doc.pdf")).toBe("文档")
    expect(getFileTypeName("report.docx")).toBe("文档")
  })

  it("应该返回压缩包的中文名称", () => {
    expect(getFileTypeName("archive.zip")).toBe("压缩包")
    expect(getFileTypeName("backup.rar")).toBe("压缩包")
  })

  it("应该返回代码的中文名称", () => {
    expect(getFileTypeName("script.js")).toBe("代码")
    expect(getFileTypeName("app.ts")).toBe("代码")
  })

  it("应该返回其他的中文名称", () => {
    expect(getFileTypeName("file.xyz")).toBe("其他")
    expect(getFileTypeName("unknown")).toBe("其他")
  })
})

describe("边界情况测试", () => {
  it("应该处理空字符串", () => {
    expect(getFileType("")).toBe("other")
    expect(getFileIcon("")).toBe("ri-file-line")
    expect(getFileTypeName("")).toBe("其他")
  })

  it("应该处理只有扩展名的文件", () => {
    expect(getFileType(".txt")).toBe("document")
    expect(getFileIcon(".jpg")).toBe("ri-image-line")
  })

  it("应该处理非常长的文件名", () => {
    const longName = "a".repeat(200) + ".pdf"
    expect(getFileType(longName)).toBe("document")
  })

  it("应该处理带空格的文件名", () => {
    expect(getFileType("my file.txt")).toBe("document")
    expect(getFileType("photo 2024.jpg")).toBe("image")
  })

  it("应该处理带特殊字符的文件名", () => {
    expect(getFileType("file-name_v2.0.pdf")).toBe("document")
    expect(getFileType("test[1].jpg")).toBe("image")
  })

  it("应该处理中文文件名", () => {
    expect(getFileType("文档.pdf")).toBe("document")
    expect(getFileType("图片.jpg")).toBe("image")
    expect(getFileType("音乐.mp3")).toBe("audio")
  })
})

describe("所有支持的文件扩展名", () => {
  it("应该支持所有图片扩展名", () => {
    const imageExts = ["jpg", "jpeg", "png", "gif", "bmp", "webp", "svg", "ico"]
    imageExts.forEach(ext => {
      expect(getFileType(`file.${ext}`)).toBe("image")
      expect(getFileIcon(`file.${ext}`)).toBe("ri-image-line")
    })
  })

  it("应该支持所有视频扩展名", () => {
    const videoExts = ["mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "m4v"]
    videoExts.forEach(ext => {
      expect(getFileType(`video.${ext}`)).toBe("video")
      expect(getFileIcon(`video.${ext}`)).toBe("ri-video-line")
    })
  })

  it("应该支持所有音频扩展名", () => {
    const audioExts = ["mp3", "wav", "flac", "aac", "ogg", "m4a", "wma"]
    audioExts.forEach(ext => {
      expect(getFileType(`audio.${ext}`)).toBe("audio")
      expect(getFileIcon(`audio.${ext}`)).toBe("ri-music-line")
    })
  })

  it("应该支持所有文档扩展名", () => {
    const docExts = ["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "rtf", "odt", "ods", "odp"]
    docExts.forEach(ext => {
      expect(getFileType(`doc.${ext}`)).toBe("document")
      expect(getFileIcon(`doc.${ext}`)).toBe("ri-file-text-line")
    })
  })

  it("应该支持所有压缩包扩展名", () => {
    const archiveExts = ["zip", "rar", "7z", "tar", "gz", "bz2", "xz"]
    archiveExts.forEach(ext => {
      expect(getFileType(`archive.${ext}`)).toBe("archive")
      expect(getFileIcon(`archive.${ext}`)).toBe("ri-file-zip-line")
    })
  })

  it("应该支持所有代码扩展名", () => {
    const codeExts = ["js", "ts", "html", "css", "json", "xml", "py", "java", "cpp", "c", "h", "rs", "go", "php", "rb", "swift", "kt"]
    codeExts.forEach(ext => {
      expect(getFileType(`code.${ext}`)).toBe("code")
      expect(getFileIcon(`code.${ext}`)).toBe("ri-code-line")
    })
  })
})
